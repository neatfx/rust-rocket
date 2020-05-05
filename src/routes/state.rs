// State

use std::sync::atomic::Ordering;

use super::super::state::HitCount;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Route;
use rocket::State;

pub fn routes() -> Vec<Route> {
    routes![
        record_hit_count_with_state,
        get_hit_count_from_state,
        get_hit_count_from_state_with_hitcounter,
        generate_request_id_with_local_state,
        admin_panel,
        admin_panel_user,
    ]
}

#[get("/state/add_hit_count")]
fn record_hit_count_with_state(hit_count: State<HitCount>) -> String {
    hit_count.count.fetch_add(1, Ordering::Relaxed);
    let count = get_hit_count_from_state(hit_count);
    format!("hit_count has been recorded: {}", count)
}

#[get("/state/get_hit_count")]
fn get_hit_count_from_state(hit_count: State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("{}", current_count)
}

#[derive(Debug)]
struct HitCounter(usize);
impl<'a, 'r> FromRequest<'a, 'r> for HitCounter {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        // 使用 Request Guard 取回 managed state
        match req.guard::<State<HitCount>>() {
            rocket::Outcome::Failure(_e) => rocket::Outcome::Failure((Status::BadRequest, ())),
            rocket::Outcome::Success(counter) => {
                rocket::Outcome::Success(HitCounter(counter.count.load(Ordering::Relaxed)))
            }
            rocket::Outcome::Forward(req) => rocket::Outcome::Forward(req),
        }
    }
}

#[get("/state/get_hit_count_from_state_with_hitcounter")]
fn get_hit_count_from_state_with_hitcounter(counter: HitCounter) -> String {
    format!("{}", counter.0)
}

use std::sync::atomic::AtomicUsize;
/// A global atomic counter for generating IDs.
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
/// A type that represents a request's ID.
struct RequestId(pub usize);

/// Returns the current request's ID, assigning one only as necessary.
impl<'a, 'r> FromRequest<'a, 'r> for &'a RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // 此处的闭包每次请求期间至多执行一次：第一次 `RequestId` 会被使用，如果再次请求，`local_cache` 将返回相同的值
        request::Outcome::Success(
            request.local_cache(|| RequestId(ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
        )
    }
}

#[get("/state/generate_request_id")]
fn generate_request_id_with_local_state(id: &RequestId) -> String {
    format!("{}", id.0)
}

#[derive(Debug)]
struct Admin {
    admin: bool,
    id: usize,
}
#[derive(Debug)]
struct User {
    admin: bool,
    id: usize,
}
static MOCK_USER_ID: AtomicUsize = AtomicUsize::new(0);
static REQUEST_GUARD_CALL_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl<'a, 'r> FromRequest<'a, 'r> for &'a User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<&'a User, !> {
        REQUEST_GUARD_CALL_COUNTER.fetch_add(1, Ordering::Relaxed);

        let user_result = request.local_cache(|| {
            // 获取 USER_ID 是一个开销较大的操作，因此可使用 local state 进行缓存

            // 以下注释部分仅为说明实际代码逻辑
            // let db = request.guard::<Database>().succeeded()?;
            // request.cookies()
            //     .get_private("user_id")
            //     .and_then(|cookie| cookie.value().parse().ok())
            //     .and_then(|id| db.get_user(id).ok())

            MOCK_USER_ID.fetch_add(1, Ordering::Relaxed);

            User {
                admin: false,
                id: MOCK_USER_ID.load(Ordering::Relaxed),
            }
        });

        request::Outcome::Success(user_result)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, !> {
        let user = request.guard::<&User>()?; // 第 1 次调用 Rquest Guard - User

        if user.admin {
            request::Outcome::Success(Admin {
                admin: true,
                id: user.id,
            })
        } else {
            rocket::Outcome::Forward(()) // 第 2 次调用 Rquest Guard - User
        }
    }
}

#[get("/state/local/admin")]
fn admin_panel(admin: Admin) -> &'static str {
    println!("admin_panel - {:#?}", admin);
    "Hello, administrator. This is the admin panel!"
}

#[get("/state/local/admin", rank = 2)]
fn admin_panel_user(user: &User) -> String {
    println!(
        "Sorry, you must be an administrator to access this page. {:#?}",
        user
    );
    let request_guard_call_count = REQUEST_GUARD_CALL_COUNTER.load(Ordering::Relaxed);
    println!("CALL - {:#?}", request_guard_call_count);
    format!(
        "REQUEST_GUARD_CALL_COUNTER = {}, User.id = {}",
        request_guard_call_count, user.id
    )
}

#[test]
fn test_raw_state_count() {
    let rocket = super::super::rocket_ins();

    assert_eq!(get_hit_count_from_state(State::from(&rocket).unwrap()), "0");
    assert!(record_hit_count_with_state(State::from(&rocket).unwrap())
        .contains("hit_count has been recorded: 1"));
    assert_eq!(get_hit_count_from_state(State::from(&rocket).unwrap()), "1");
}
