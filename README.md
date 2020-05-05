# *rust-rocket*

以单个项目集成示例的方式对 Rust Web 开发框架 [Rocket](https://rocket.rs/) 的功能及使用方法进行探索。

## 如何使用

```bash
# 运行服务
cargo run

# 集成测试（ 无需事先运行服务 ）
cargo test
```

## 文档笔记

### Getting Started

Rocket 框架依赖 `nightly` 版本的 Rust，需要切换：

```bash
# 全局切换
rustup default nightly

# 仅切换 Rocket 项目所使用的 Rust 版本
cd rocket-project-dir && rustup override set nightly
```

### Requests

#### `Body Data`

```rust
#[post("/", data = "<input>")]
fn new(input: T) { /* .. */ } // T 必须实现 FromData 特质
```

处理 `Body Data`，需要为 `route handler` 标注 `data = "<param>"` 以指明其所期望的 `body data`，此 `param` 是 `route handler` 的参数，其必须实现 `FromData` 特质，任何实现了 `FromData` 特质的类型即为 `Data Guard`

#### `Body Data > Form`

```rust
use rocket::request::Form; // `Form` 类型实现了 `FromData` 特质

#[derive(FromForm)]
struct Task {
    complete: bool,
    description: String,
}

#[post("/todo", data = "<task>")]
fn new(task: Form<Task>) { /* .. */ } // Form<T> 的泛型参数要求实现 FromForm 特质
```

当 `POST /todo` 请求到达时，表单数据将被自动解析为 `Task` 结构：

- 如果到达数据的 `Content-Typ` 不正确，请求会被 `forwarded`
- 如果数据未解析或者无效，将返回自定义 `400 - Bad Request` 或者 `422 - Unprocessable Entity` 错误

可以使用 `Option` 及 `Result` 类型对 `forward` 或 `failure` 进行捕获：

```rust
#[post("/todo", data = "<task>")]
fn new(task: Option<Form<Task>>) { /* .. */ }
```

#### `Body Data > Form > Lenient Parsing`

Rocket 默认以严格方式对 `Form` 进行解析，比如说请求提供了 `a、b、c`，而 `From<T>` 的 `T` 只包含 `a、c`，表单将不会被解析为 `Form<T>`。这时可使用 `LenientForm<T>` 类型进行解析，只要表单数据包含 `T` 中字段的超集即可。`LenientForm` 会自动丢弃额外表单字段而不产生错误。

```rust
use rocket::request::LenientForm;

#[derive(FromForm)]
struct Task {
    /* .. */
}

#[post("/todo", data = "<task>")]
fn new(task: LenientForm<Task>) { /* .. */ } // LenientForm<T> 的泛型参数同样要求实现 FromForm 特质
```

#### `Body Data > Form > Field Renaming`

使用 `#[form(field = "name")]` 对重命名的字段进行注释，Rocket 会为结构体字段查找表单字段，并正常解析。

```rust
#[derive(FromForm)]
struct External {
    #[form(field = "type")]
    api_type: String // Rocket 会自动将表单字段 “type” 与 ”api_type“ 字段进行匹配
}
```

#### `Body Data > Form > Field Validation`

通过实现 `FromFormValue` 特质可对表单字段进行验证。

```rust
use rocket::http::RawStr;
use rocket::request::FromFormValue;

struct AdultAge(usize);

impl<'v> FromFormValue<'v> for AdultAge {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<AdultAge, &'v RawStr> {
        match form_value.parse::<usize>() {
            Ok(age) if age >= 21 => Ok(AdultAge(age)),
            _ => Err(form_value),
        }
    }
}

#[derive(FromForm)]
struct Person {
    age: AdultAge
}
```

表单数据验证未通过时，Rocket 不会调用 `route handler`，可为字段使用 `Option` 或 `Result` 类型以对解析失败进行捕获：

```rust
#[derive(FromForm)]
struct Person {
    age: Option<AdultAge>
}
```

枚举也可以衍生获得 `FromFormValue` 特质：

```rust
#[derive(FromFormValue)]
enum MyValue {
    First,
    Second,
    Third,
}
```

当表单值与大小写不敏感，字符串化版本的枚举成员名匹配时，会成功返回 `derive` 为枚举生成的 `FromFormValue` 特质实现 - 即所匹配的枚举成员实例。

#### `Body Data > JSON`

#### `Body Data > Streaming`

---

### Responses

---

### State

`Rocket` 支持的 `State` 包括：`Managed State`、`Request-Local State`、`Databases`

#### `Managed State`

由 `Rocket` 基于单个类型值进行管理的应用状态（ 类似 `React Hook` 中的 `state` ），Rocket 的多线程特性，因此所有 `Managed State` 必须是线程安全的。

添加 `Managed State` 的步骤：

取回 `Managed State` 的方法：

- 在 `Route` 的 `request handler` 中使用 `State` 类型作为 `Request Guard`
- 从 `FromRequest` 的实现中，以 `State<T>` 为 `guard` 调用 `Request::guard()` 方法

#### `Request-Local State`

可用于对验证及授权计算操作进行缓存，以及在 `Fairing` 中使用，比如实现请求计时功能。

#### `Databases`

连接数据库：

- 在 `Rocket.toml` 中配置数据库
- 将每个数据库与 `request guard type` 以及 `fairing` 进行关联
- 在 `route handler` 中使用 `request guard` 获取数据库连接

---

### Fairings

---

### Testing

```bash
# Codegen Debug
ROCKET_CODEGEN_DEBUG=1 cargo build
```

---

### Configuration

获取自定义配置项:

- 在 `Ad-hoc attach Fairing` 中读取配置参数
- 使用 `Managed State` 保存解析后的配置参数
- `Route` 通过 `State Guard` 获取配置参数

> 以编程方式（ 通过 `rocket::custom()` 进行加载 ）生成的配置，会覆盖 `Rocket.toml` 以及环境变量中的配置选项。

---

### 附加库

[`rocket_contrib`](https://api.rocket.rs/v0.4/rocket_contrib/)

|  模块  |  功能说明  |
| :---: | :---: |
| databases | Traits, utilities, and a macro for easy database connection pooling.
| helmet | Security and privacy headers for all outgoing responses.
| json | Automatic JSON (de)serialization support.
| msgpack | Automatic MessagePack (de)serialization support.
| serve | Custom handler and options for static file serving.
| templates | Dynamic template engine support for handlebars and tera.
| uuid | UUID parameter and form value parsing support.
