# Design Token 与主题归属

Ash Desktop 与 Ash Code 各自维护主题语义、默认值和资源校验。颜色和字体等视觉原则可复用，主题文件由解释它的产品定义。

| 产品 | 主题职责 | 状态 |
| --- | --- | --- |
| Ash Desktop | Ash UI Kit 定义组件使用的语义样式。Desktop 解析 profile 的 `[gui]` 主题选择和用户主题资源，再把解析结果交给组件。 | 提案，随原生界面实现。 [Ash UI](desktop-architecture.md#ash-ui) |
| Ash Code | TUI 持有调色板、用户主题和 `[tui].theme`。 | 已实现。 [TUI 主题文件](../crates/tui/README.md#tui-主题文件) |
