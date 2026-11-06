# Ash 产品与宿主边界

| 入口 | 源码 | 职责 |
| --- | --- | --- |
| Ash Desktop | `app/` | Rust 系统窗口和事件循环，工作台待开发 |
| Ash Code | `cli/`、`crates/tui/` | 命令行、管理入口、终端输入与界面 |
| 共享 App Server | `crates/app-server/` 和领域 crate | 业务、持久化、执行和版本化协议 |
| Remote Server | `remote-server/` | 远程运行时与独立分发 |

在仓库根目录运行 `just ash` 打开原生窗口，或运行 `just ash-code` 启动终端界面。

各产品通过 [App Server 契约](app-server-api.md) 使用共享领域能力。系统窗口由桌面端持有，PTY 和终端输入由各自的终端宿主与共享后端分工。
