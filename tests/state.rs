extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::Status;
use rocket::local::Client;

fn register_hit(client: &Client) {
    let response = client.get("/state/add_hit_count").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

fn get_count(client: &Client) -> usize {
    let mut response = client.get("/state/get_hit_count").dispatch();
    response.body_string().and_then(|s| s.parse().ok()).unwrap()
}

#[test]
fn test_count() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");

    assert_eq!(get_count(&client), 0);

    for _ in 0..50 {
        register_hit(&client);
    }
    assert_eq!(get_count(&client), 50);

    register_hit(&client);
    assert_eq!(get_count(&client), 51);
}

#[test]
fn test_count_by_impl_fromrequest() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");

    let mut response = client
        .get("/state/get_hit_count_from_state_with_hitcounter")
        .dispatch();
    assert_eq!(response.body_string(), Some("0".into()));

    for _ in 0..10 {
        register_hit(&client);
    }

    let mut response_b = client
        .get("/state/get_hit_count_from_state_with_hitcounter")
        .dispatch();

    assert_eq!(response_b.body_string(), Some("10".into()));
}

#[test]
fn test_count_parallel() {
    test_count()
}
#[test]
fn test_count_parallel_2() {
    test_count()
}
#[test]
fn test_count_parallel_3() {
    test_count()
}
#[test]
fn test_count_parallel_4() {
    test_count()
}
#[test]
fn test_count_parallel_5() {
    test_count()
}
#[test]
fn test_count_parallel_6() {
    test_count()
}
#[test]
fn test_count_parallel_7() {
    test_count()
}
#[test]
fn test_count_parallel_8() {
    test_count()
}
#[test]
fn test_count_parallel_9() {
    test_count()
}

#[test]
fn test_generate_request_id_with_local_state() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");

    let mut response = client.get("/state/generate_request_id").dispatch();
    assert_eq!(response.body_string(), Some("0".into()));

    for _ in 0..50 {
        client.get("/state/generate_request_id").dispatch();
    }

    let mut response_b = client.get("/state/generate_request_id").dispatch();
    assert_eq!(response_b.body_string(), Some("51".into()));
}

#[test]
fn test_local_state_admin() {
    let client = Client::new(rocket_ins()).expect("valid rocket instance");

    let mut response = client.get("/state/local/admin").dispatch();
    assert_eq!(
        response.body_string(),
        Some("REQUEST_GUARD_CALL_COUNTER = 2, User.id = 1".into())
    );

    for _ in 0..10 {
        client.get("/state/local/admin").dispatch();
    }

    let mut response_b = client.get("/state/local/admin").dispatch();
    assert_eq!(
        response_b.body_string(),
        Some("REQUEST_GUARD_CALL_COUNTER = 24, User.id = 12".into())
    );
}
