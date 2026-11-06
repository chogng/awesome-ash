# 桌面与 Ash Code 的快捷键

Ash Code 已接入共享 `ash-keybinding`。Ash Desktop 的快捷键由后续工作台接入同一 Rust 规则库。快捷键在端侧解析，产品分别持有命令目录、焦点上下文和用户设置。 [Ash UI](desktop-architecture.md#ash-ui) · [规则库](../crates/keybinding/README.md)

## 输入与命令

1. 桌面将 `winit` 输入转换为逻辑键、可用的物理键和修饰键。输入法组合文字由当前输入控件处理。Ash Code 从终端事件取得逻辑键和修饰键。
2. 各端提供当前焦点及上下文。共享 Resolver 按来源、优先级和注册顺序匹配规则，返回命令、等待后续按键、阻止或不匹配。User 优先于 Workbench 和 Builtin，同来源先比较显式优先级，再以较晚注册者优先。
3. 产品负责多段按键的超时、取消和命令执行。Ash Code 对单键先让当前组件处理，再走应用级 fallback。已匹配的多段前缀先进入应用级 matcher，等待超过 1 秒、焦点上下文变化或按 Escape 时取消。后台只接收命令产生的业务请求。

## 配置

| 产品 | 配置归属 |
| --- | --- |
| Ash Desktop（目标） | GUI 进程所在的本地 profile 的 `config.toml` 中的 `[gui].keybindings`，由桌面命令目录解释。切换后端连接时仍使用本地 GUI profile。 |
| Ash Code | `config.toml` 中的 `[tui].keybindings`，由 TUI 命令目录解释。使用 `/shortcuts` 查看和编辑支持的命令。 |

用户规则为有序数组，最多 1024 项。每条规则含一至四段的 `key`、一个 `command` 或 `block = true`，可附加 `when` 条件。平台覆盖 `mac`、`linux`、`win` 可以给出替代按键或用 `false` 禁用本项。

Ash Code 支持 `inputFocus`、`chatInputEmpty`、`selectionVisible` 和 `keyEventPress` 条件。命令及其适用范围由 TUI 的 [键位声明](../crates/tui/src/keymap/bindings.rs) 维护。

```toml
[[tui.keybindings]]
key = "primary+k primary+c"
command = "ashCode.action.copyLastResponse"
when = "inputFocus && !selectionVisible"

[[tui.keybindings]]
key = "ctrl+o"
block = true
```

配置变更时先在临时规则集中编译完整数组，再替换生效规则并清理旧的多段输入状态。无效配置保留上一份有效规则并给出诊断。Ash Code 的 `/shortcuts` 可录制单键和两段组合，三至四段组合、条件、屏蔽和平台覆盖直接编辑 TOML。设置界面编辑目标命令的用户规则，保留默认及其他命令规则。

## 验证

共享规则使用 `just test ash-keybinding` 验证解析、优先级、条件、阻止和多段匹配。端侧另外验证系统输入映射、焦点和多段按键取消、配置替换失败，以及无障碍键盘操作。
