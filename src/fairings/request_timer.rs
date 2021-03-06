//! 利用 local state 功能实现请求计时 Fairing 的示例
//! 

/// Fairing for timing requests.
pub struct RequestTimer;

/// Value stored in request-local state.
#[derive(Copy, Clone)]
struct TimerStart(Option<SystemTime>);

impl Fairing for RequestTimer {
    fn info(&self) -> Info {
        Info {
            name: "Request Timer",
            kind: Kind::Request | Kind::Response
        }
    }

    /// Stores the start time of the request in request-local state.
    fn on_request(&self, request: &mut Request, _: &Data) {
        // Store a `TimerStart` instead of directly storing a `SystemTime`
        // to ensure that this usage doesn't conflict with anything else
        // that might store a `SystemTime` in request-local cache.
        request.local_cache(|| TimerStart(Some(SystemTime::now())));
    }

    /// Adds a header to the response indicating how long the server took to
    /// process the request.
    fn on_response(&self, request: &Request, response: &mut Response) {
        let start_time = request.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            response.set_raw_header("X-Response-Time", format!("{} ms", ms));
        }
    }
}

/// Request guard used to retrieve the start time of a request.
#[derive(Copy, Clone)]
pub struct StartTime(pub SystemTime);

// Allows a route to access the time a request was initiated.
impl<'a, 'r> FromRequest<'a, 'r> for StartTime {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<StartTime, ()> {
        match *request.local_cache(|| TimerStart(None)) {
            TimerStart(Some(time)) => Outcome::Success(StartTime(time)),
            TimerStart(None) => Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}