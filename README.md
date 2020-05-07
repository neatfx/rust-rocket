# *rust-rocket*

以单个项目集成示例的方式对 Rust Web 开发框架 [Rocket](https://rocket.rs/) 的功能及使用方法进行探索。

## 如何使用

文档笔记位于 `docs/` 目录。

Rocket 框架依赖 `nightly` 版本的 Rust，需要切换：

```bash
# 全局切换
rustup default nightly

# 仅切换 Rocket 项目所使用的 Rust 版本
cd rocket-project-dir && rustup override set nightly
```

```bash
# 运行服务
cargo run

# 集成测试（ 无需事先运行服务 ）
cargo test
```
