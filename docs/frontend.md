# 前端开发

桌面端的原生入口位于 `app/`。在仓库根目录运行 `just build-ash` 构建，运行 `just ash` 打开系统窗口。macOS、Windows 和 Linux X11 共用 Rust 事件循环。

工作台按 [Ash UI 架构](desktop-architecture.md#ash-ui) 规划，业务能力通过 [App Server 接入边界](frontend-app-server-boundary.md) 接入。工作台和独立 Web 前端的进度见 [Rust App 路线图](../TODO.md#roadmap--rust-app)。
