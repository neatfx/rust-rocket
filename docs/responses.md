# Responses

路由处理器（ route handler ）可返回任意实现了 `Responder` 特质的类型值。

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

通过 `Status` 生成响应依赖于状态码自身。对于错误状态码（ 400 ～ 599 ），`Status` 转递给对应的 `error catcher`，下面表格总结了状态码与响应的对应关系：

| Status Code Range | Response |
| :---:             | :---:    |
| [400, 599]        | Forwards to error catcher for given status |
| 100, [200, 205]   | Empty with given status |
| All others.       | Invalid. Errors to 500 catcher |

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

Rocket 通过以上代码生成的 `Responder` 实现：

- 设置响应状态为 `500: Internal Server Error.`
- 设置 `Content-Type` 为 `application/json`
- 为响应添加 `self.header` 以及 `self.more` headers
- 使用 `self.inner` 完成响应

## Implementations

Rocket 为 Rust 标准库中的许多类型实现了 `Responder` 特质，包括 `String`、`&str`、`File`、`Option` 以及 `Result`。

### Strings

字符串被当作 `sized body` 使用，响应的内容类型被设置为 `text/plain`：

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

基于以上实现，可从 `handler` 中直接返回 `&str` 或者 `String` 类型：

```rust
#[get("/string")]
fn handler() -> &'static str {
    "Hello there! I'm a string!"
}
```

#### `Option`

`Option` 是 `wrapping responder`：`Option<T>` 仅在 T 实现了 `Responder` 时才能被返回。如果 Option 为 Some，`wrapping responder` 被用来响应客户端，否则，返回 `404 - Not Found` 错误给客户端。

`Option` 适合作为运行时才能确定内容是否存在的情况下的返回类型。

```rust
// 由于使用了 Option，当文件存在时将返回 200，文件不存在是时代码返回 404
use rocket::response::NamedFile;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}
```

#### `Result`

`Result` 是一类特殊的 `wrapping responder`：其功能取决于错误类型 `E` 是否实现了 `Responder`。

当错误类型 `E` 实现了 `Responder`，`Ok` 或 `Err` 中可能存在的 `wrapped Responder` 将被用于响应给客户端。这意味着 responder 能够在运行时被动态选定，不同情况下将会有两种不同类型的响应。

```rust
use rocket::response::NamedFile;
use rocket::response::status::NotFound;

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}
```

如果错误类型 `E` 没有实现 `Responder`，它将利用自身的 `Debug` 实现简单输出至控制台，并返回一个 500 错误给客户端。

## Rocket Responders

很多实用 Rocket 特性都是通过 `Responder` 实现的。这些 `Responders` 位于 `response` 模块及 `rocket_contrib` 库，包括：

- `Content` - Used to override the Content-Type of a response.
- `NamedFile` - Streams a file to the client; automatically sets the Content-Type based on the file's extension.
- `Redirect` - Redirects the client to a different URI.
- `Stream` - Streams a response to a client from an arbitrary Reader type.
- `status` - Contains types that override the status code of a response.
- `Flash` - Sets a "flash" cookie that is removed when accessed.
- `Json` - Automatically serializes values into JSON.
- `MsgPack` - Automatically serializes values into MessagePack.
- `Template` - Renders a dynamic template using handlebars or Tera.

### Streaming

当需要发送大量数据到客户端的时候，最好使用流式数据，以避免消耗大量内存。Rocket 为此提供了 `Stream` 类型，以简化流式数据处理。可从任意 `Read` 类型中创建获得 `Stream` 类型。

```rust
use std::os::unix::net::UnixStream;
use rocket::response::Stream;

#[get("/stream")]
fn stream() -> Result<Stream<UnixStream>, std::io::Error> {
  // stream from a local Unix stream
  UnixStream::connect("/path/to/my/socket").map(Stream::from)
}
```

### JSON

此类型位于 `rocket_contrib` 库，允许以 JSON 数据进行响应：简单返回一个 `Json<T>` 类型值（ T 为序列化为 JSON 后的结构类型，且必须实现 `serde` 库中的 `Serialize` 特质，该特质可以自动获得 ）

```rust
use serde::Serialize;
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct Task { /* .. */ }

// Json 类型将结构序列化为 JSON、设置 Content-Type 为 JSON，并以固定大小的 body 发出序列化数据
// 如果序列化失败，返回 `500 - Internal Server Error`
#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task { /* .. */ })
}
```

## Templates

Rocket 所包含的内置模版支持，主要由 `rocket_contrib` 库中提供的 `Template` `responder` 实现。

```rust
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = /* object-like value */;
    Template::render("index", &context)
}
```

`context` 可以是任何实现了 `Serialize` 特质并序列化为 `Object` 的值，比如结构体、`HashMaps` 或者其它。

模版必须先注册然后才可以被渲染：

```rust
fn main() {
    rocket::ignite()
        .mount("/", routes![/* .. */])
        .attach(Template::fairing());
}
```

Rocket 在可配置的 `template_dir` 目录中查找模版。

Rocket 中的模版支持是引擎无关的。使用何种引擎渲染模版取决于模版文件扩展名。例如，如果文件以 `.hbs` 结尾，使用 `Handlebars` 引擎，如果文件以 `.tera` 结尾，则使用 `Tera` 引擎。

> 注意：模版名并不包括扩展名
>
> 例如模版文件名为 `index.html.tera`，应该以 `render("index")` 的调用方式渲染模版，并在模版中使用名称 "index"
>
> 引用 `index.html.tera` 的方式为 `{% extends "index" %}`，引用 `base.html.tera` 的方式为 `{% extends "base" %}`

### Live Reloading

当应用在 `debug` 模式下编译时，模版会被自动重新加载。这意味着不需要重新编译应用来观察模版变化：简单刷新即可！

在 `release` 模式下，重新加载功能被禁用。

[Template](https://api.rocket.rs/v0.4/rocket_contrib/templates/struct.Template.html) API 文档包含了更多信息，包括如何自定义模版引擎添加 `helpers` 以及 `filters`。

## Typed URIs

Rocket 中的 `uri!`  宏能够以可靠、类型安全、`URI-safe` 的方式创建应用路由 `URIs`。不匹配的类型或路由参数会在编译时被捕获，并且，路由 `URIs` 的变化会自动反映到生成的 `URIs` 中。

`uri!` 宏会将传给它的值插入指定路由形成 `URI`，然后返回一个 `Origin` 结构。

所有传递给 `uri!` 的值都会使用值类型的 `UriDisplay` 实现渲染到 `URI` 中的合适位置，`UriDisplay` 实现确保渲染后的值是 `URI-safe` 的。

注意，`Origin` 实现了 `Into<Uri>` ( 并可通过 `TryInto<Uri>` 扩展 )，因而它可以根据需要使用 `into()` 转换为一个 `Uri` 并传递给 `Redirect::to()` 等方法。

```rust
#[get("/person/<name>?<age>")]
fn person(name: String, age: Option<u8>) { /* .. */ }
```

指向以上路由的 `URIs` 可通过以下方式创建：

```rust
// 使用不具名参数，以路由路径声明的顺序创建
let mike = uri!(person: "Mike Smith", 28);
assert_eq!(mike.to_string(), "/person/Mike%20Smith?age=28");

// 使用具名参数，顺序无关
let mike = uri!(person: name = "Mike", age = 28);
let mike = uri!(person: age = 28, name = "Mike");
assert_eq!(mike.to_string(), "/person/Mike?age=28");

// 指定挂载点
let mike = uri!("/api", person: name = "Mike", age = 28);
assert_eq!(mike.to_string(), "/api/person/Mike?age=28");

// 忽略可选请求参数
let mike = uri!(person: "Mike", _);
let mike = uri!(person: name = "Mike", age = _);
assert_eq!(mike.to_string(), "/person/Mike");
```

Rocket 会告知编译时任何参数不匹配的情况：

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

推荐使用 `uri!` 宏作为构建路由 `URIs` 的唯一方式。

### Ignorables

使用 `_` 忽略请求参数时，路由 `URI` 中对应的类型必须实现 `Ignorable` 特质。被忽略参数不会添加到结果 `Origin` 中。

路径参数是不可忽略的。

### Deriving UriDisplay

自定义类型可衍生获得 `UriDisplay` 特质。出现在 `URI` 路径部分中的类型，使用 `UriDisplayPath` 进行衍生，出现在 `URI` 请求部分的类型，使用 `UriDisplayQuery` 进行衍生。

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

以上代码通过使用 `UriDisplayQuery`，自动生成一个 `UriDisplay<Query>` 的实现，从而允许使用 `uri!` 生成指向 `add_user` 的 `URIs`：

```rust
let link = uri!(add_user: 120, UserDetails { age: Some(20), nickname: "Bob".into() });
assert_eq!(link.to_string(), "/user/120?age=20&nickname=Bob");
```

### Typed URI Parts

`UriPart` 特质将标记为 `URI` 的一部分的类型分类为 `Path` 或 `Query`。换言之，实现了 `UriPart` 的类型是在类型层面上表示 URI 部分的标记类型。像 `UriDisplay` 以及 `FromUriParam` 这样的特质通过 `UriPart: P: UriPart` 与一个泛型参数进行绑定。这将为每个特质创建 2 个实例：`UriDisplay<Query>` 及 `UriDisplay<Path>`, `FromUriParam<Query>` 及 `FromUriParam<Path>`。

正如名称暗示的那样，`Path` 版本的特质被用于在 `URI` 的路径部分显示参数，而 `Query` 版本用于在 `URI` 的请求部分显示参数。这些不同版本的特质确实存在区分，在类型层面上，在值被写入 URI 的位置，允许为了类型安全而不顾及两个位置间的差异。例如，在请求部分使用一个 `None` 值是有效的，而在路径部分忽略整个参数却是无效的。通过使用类型系统，凭借不同的`FromUriParam<Path>` 以及 `FromUriParam<Query>` 实现进行区分，这些条件能够以合适的方式得到强制执行。

### Conversions

`FromUriParam` 用于对传给 `uri!` 的值在其使用 `UriDisplay` 进行显示前实施转换。

对于 `URI` 中的一部分 `P`，如果存在实现了 `FromUriParam<P, S>` 的类型 `T`，那么，对于在声明中的 `P` 部分使用了类型 `T` 的路由 `URI` 参数，可在 `uri!` 中使用类型 `S` 的值。

例如，以下实现由 Rocket 提供，对于声明为 `String` 类型的路由 `URI` 参数，允许在 `uri!` 调用中使用一个 `&str`。

```rust
impl<'a, P: UriPart> FromUriParam<P, &'a str> for String {
    type Target = &'a str;
}
```

其它需知的转换还包括：

- `&str` to `RawStr`
- `String` to `&str`
- `String` to `RawStr`
- `T` to `Option<T>`
- `T` to `Result<T, E>`
- `T` to `Form<T>`
- `&str` to `&Path`
- `&str` to `PathBuf`

嵌套转换：

```rust
#[get("/person/<id>?<details..>")]
fn person(id: usize, details: Option<Form<UserDetails>>) { /* .. */ }

uri!(person: id = 100, details = UserDetails { age: Some(20), nickname: "Bob".into() });
```

更多细节可参考 [FromUriParam](https://api.rocket.rs/v0.4/rocket/http/uri/trait.FromUriParam.html)
