# Configuration

获取自定义配置项:

- 在 `Ad-hoc attach Fairing` 中读取配置参数
- 使用 `Managed State` 保存解析后的配置参数
- `Route` 通过 `State Guard` 获取配置参数

> 以编程方式（ 通过 `rocket::custom()` 进行加载 ）生成的配置，会覆盖 `Rocket.toml` 以及环境变量中的配置选项。