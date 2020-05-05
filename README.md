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
