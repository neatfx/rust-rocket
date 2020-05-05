extern crate network_rocket;

use network_rocket::rocket_ins;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

const UPLOAD_CONTENTS: &str = "I'M GOING TO BE UPLOADED.";

#[test]
fn test_streaming_upload() {
    let client = Client::new(rocket_ins()).unwrap();
    let mut res = client
        .post("/body-data/streaming")
        .header(ContentType::Plain)
        .body(UPLOAD_CONTENTS)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.body_string(), Some(UPLOAD_CONTENTS.len().to_string()));

    let mut res_b = client.get("/body-data/streaming/upload.txt").dispatch();

    assert_eq!(res_b.status(), Status::Ok);
    assert_eq!(res_b.body_string(), Some(UPLOAD_CONTENTS.to_string()));

    let mut res_c = client.delete("/body-data/streaming/upload.txt").dispatch();

    assert_eq!(res_c.status(), Status::Ok);
    assert_eq!(res_c.body_string(), Some("deleted".into()));

    let mut res_d = client.delete("/body-data/streaming/upload.txt").dispatch();

    assert_eq!(res_d.status(), Status::Ok);
    assert_eq!(
        res_d.body_string(),
        Some("No such file or directory (os error 2)".into())
    );
}
