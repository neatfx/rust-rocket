# Responses

可返回实现了 `Responder` 特质的类型值

## Responder

实现 `Responder` 特质的类型知道如何从其值生成 `Response`。`Response` 包含 HTTP `status`、`headers` 以及 `body`。`body` 既可以是固定大小的也可以是 `streaming`，具体使用哪一种取决于 `Responder` 实现。`String` 使用固定大小的 `body`，而  `File` 使用流式 `response`。 `Responders` 根据其要响应的传入请求动态判断其 `response`。

### Wrapping

```rust
// R 为实现了 Responder 的类型
// WrappingResponder 在返回 response 之前会对 R 返回的 response 进行修改
struct WrappingResponder<R>(R);
```

```rust
use rocket::response::status;

#[post("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}
```

```rust
use rocket::response::content;

#[get("/")]
fn json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}
```

### Error

`Responder` 有可能会失败，它们不需要总是生成 `Response`，使用状态码返回 `Err`，Rocket 会将请求转递给与状态码相匹配的 `error catcher` 进行处理：

- 如果对应状态码的 `error catcher` 已注册，Rocket 会进行调用。`catcher` 会创建并返回 response 给客户端。
- 如果 `error catcher` 未注册，并且状态码为标准 HTTP 状态码，默认的 `error catcher` 会被调用，其将返回包含状态码及错误描述的 HTML 页面
- 如果没有与自定义状态码对应的 `catcher`，Rocket 将使用 500 `error catcher` 来返回 `Response`

### Status

可通过直接返回 Status 手动转递请求到一个 `catcher`，尽管并不鼓励这样做：

```rust
use rocket::http::Status;

#[get("/")]
fn just_fail() -> Status {
    Status::NotAcceptable
}
```

| Status Code Range | Response |
| :---:             | :---:    |
| [400, 599]        | Forwards to catcher for given status.|
| 100, [200, 205]   | Empty with given status.|
| All others.       | Invalid. Errors to 500 catcher.|

## Custom Responders

```rust
use rocket::http::{Header, ContentType};

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct MyResponder {
    inner: OtherResponder,
    header: ContentType,
    more: Header<'static>,
    #[response(ignore)]
    unrelated: MyType,
}
```

## Implementations

Rocket 为 Rust 标准库中的许多类型实现了 `Responder`，包括 `String`, `&str`, `File`, `Option` 以及 `Result`。

### Strings

```rust
use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

impl<'a> Responder<'a> for String {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        Response::build()
            .header(ContentType::Plain)
            .sized_body(Cursor::new(self))
            .ok()
    }
}
```

```rust
#[get("/string")]
fn handler() -> &'static str {
    "Hello there! I'm a string!"
}
```

#### Option

```rust
use rocket::response::NamedFile;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

#### Result

```rust
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}
```

## Rocket Responders

Rocket 最佳特性中的一些是通过 `Responder` 实现的。可在 `response` 模块以及 `rocket_contrib` 库中找到这些 `Responders`，这其中包括：

- Content - Used to override the Content-Type of a response.
- NamedFile - Streams a file to the client; automatically sets the Content-Type based on the file's extension.
- Redirect - Redirects the client to a different URI.
- Stream - Streams a response to a client from an arbitrary Reader type.
- status - Contains types that override the status code of a response.
- Flash - Sets a "flash" cookie that is removed when accessed.
- Json - Automatically serializes values into JSON.
- MsgPack - Automatically serializes values into MessagePack.
- Template - Renders a dynamic template using handlebars or Tera.

### Streaming

```rust
use std::os::unix::net::UnixStream;
use rocket::response::Stream;

#[get("/stream")]
fn stream() -> Result<Stream<UnixStream>, std::io::Error> {
    UnixStream::connect("/path/to/my/socket").map(Stream::from)
}
```

### JSON

```rust
use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Task { /* .. */ }

#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task { /* .. */ })
}
```

## Templates

```rust
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = /* object-like value */;
    Template::render("index", &context)
}
```

```rust
fn main() {
    rocket::ignite()
        .mount("/", routes![/* .. */])
        .attach(Template::fairing());
}
```

### Live Reloading

## Typed URIs

```rust
#[get("/person/<name>?<age>")]
fn person(name: String, age: Option<u8>) { /* .. */ }
```

```rust
// with unnamed parameters, in route path declaration order
let mike = uri!(person: "Mike Smith", 28);
assert_eq!(mike.to_string(), "/person/Mike%20Smith?age=28");

// with named parameters, order irrelevant
let mike = uri!(person: name = "Mike", age = 28);
let mike = uri!(person: age = 28, name = "Mike");
assert_eq!(mike.to_string(), "/person/Mike?age=28");

// with a specific mount-point
let mike = uri!("/api", person: name = "Mike", age = 28);
assert_eq!(mike.to_string(), "/api/person/Mike?age=28");

// with optional (defaultable) query parameters ignored
let mike = uri!(person: "Mike", _);
let mike = uri!(person: name = "Mike", age = _);
assert_eq!(mike.to_string(), "/person/Mike");
```

```bash
error: person route uri expects 2 parameters but 1 was supplied
 --> examples/uri/src/main.rs:9:29
  |
9 |     uri!(person: "Mike Smith");
  |                  ^^^^^^^^^^^^
  |
  = note: expected parameters: name: String, age: Option<u8>
```

```bash
error: the trait bound u8: FromUriParam<Query, &str> is not satisfied
 --> examples/uri/src/main.rs:9:35
  |
9 |     uri!(person: age = "10", name = "Mike");
  |                        ^^^^ FromUriParam<Query, &str> is not implemented for u8
  |
```

### Ignorables

### Deriving UriDisplay

```rust
use rocket::http::RawStr;
use rocket::request::Form;

#[derive(FromForm, UriDisplayQuery)]
struct UserDetails<'r> {
    age: Option<usize>,
    nickname: &'r RawStr,
}

#[post("/user/<id>?<details..>")]
fn add_user(id: usize, details: Form<UserDetails>) { /* .. */ }
```

```rust
let link = uri!(add_user: 120, UserDetails { age: Some(20), nickname: "Bob".into() });
assert_eq!(link.to_string(), "/user/120?age=20&nickname=Bob");
```

### Typed URI Parts

### Conversions

```rust
impl<'a, P: UriPart> FromUriParam<P, &'a str> for String {
    type Target = &'a str;
}
```

```rust
#[get("/person/<id>?<details..>")]
fn person(id: usize, details: Option<Form<UserDetails>>) { /* .. */ }

uri!(person: id = 100, details = UserDetails { age: Some(20), nickname: "Bob".into() });
```
