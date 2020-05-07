# Requests

## `Body Data`

```rust
#[post("/", data = "<input>")]
fn new(input: T) { /* .. */ } // T 必须实现 FromData 特质
```

处理 `Body Data`，需要为 `route handler` 标注 `data = "<param>"` 以指明其所期望的 `body data`，此 `param` 是 `route handler` 的参数，其必须实现 `FromData` 特质，任何实现了 `FromData` 特质的类型即为 `Data Guard`

### `Body Data > Form`

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

### `Body Data > JSON`

### `Body Data > Streaming`
