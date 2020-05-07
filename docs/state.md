# State

`Rocket` 支持的 `State` 包括：`Managed State`、`Request-Local State`、`Databases`

## `Managed State`

由 `Rocket` 基于单个类型值进行管理的应用状态（ 类似 `React Hook` 中的 `state` ），Rocket 的多线程特性，因此所有 `Managed State` 必须是线程安全的。

添加 `Managed State` 的步骤：

取回 `Managed State` 的方法：

- 在 `Route` 的 `request handler` 中使用 `State` 类型作为 `Request Guard`
- 从 `FromRequest` 的实现中，以 `State<T>` 为 `guard` 调用 `Request::guard()` 方法

## `Request-Local State`

可用于对验证及授权计算操作进行缓存，以及在 `Fairing` 中使用，比如实现请求计时功能。

## `Databases`

连接数据库：

- 在 `Rocket.toml` 中配置数据库
- 将每个数据库与 `request guard type` 以及 `fairing` 进行关联
- 在 `route handler` 中使用 `request guard` 获取数据库连接