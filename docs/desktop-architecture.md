# Desktop 架构与宿主边界

`app/` 是独立 Cargo 工作区，入口通过 winit 创建系统窗口并处理关闭。开发入口是 `just ash`，构建入口是 `just build-ash`。macOS、Windows 和 Linux X11 共用 Rust 实现。

后续桌面界面拥有窗口、输入和视图状态。Session、Thread、Turn、审批和持久化归共享 Rust 后端，桌面通过 [App Server 契约](app-server-api.md) 访问。桌面安装包和新的工作台见 [Rust App 路线图](../TODO.md#roadmap--rust-app)。

## Ash GPUI

以下是原生桌面前端提案。**Ash GPUI** 是 Rust GPU 界面框架，**Ash UI Kit** 是它的基础组件库。可以借鉴 GPUI 用 Rust 组合视图、由框架统一布局和绘制的开发体验。Ash GPUI 自己定义视图和输入契约，窗口及 GPU 后端交给 `winit` 和 `wgpu`。 [Zed GPUI](https://github.com/zed-industries/zed/blob/main/crates/gpui/README.md)

### 层与所有权

| 层 | 建议位置 | 职责 |
| --- | --- | --- |
| Ash Desktop | `app/` | 装配窗口和工作台，注册产品命令，管理本地配置、App Server 连接及工作流。 |
| Ash UI Kit | `app/ui-kit/`（`ash-ui-kit`） | 提供有 Ash 默认样式和交互语义的组件。接收数据、主题和动作回调，供工作台与组件预览复用。 |
| Ash GPUI | `app/gpui/`（`ash-gpui`） | 管理视图身份、布局、滚动、命中测试、焦点、输入分发、无障碍语义、重绘调度和绘制指令。 |
| 平台与绘制适配 | `ash-gpui` 内部模块 | 通过 `winit` 接收窗口及输入事件，`ash_gpui::renderer` 通过 `wgpu` 提交绘制，通过 AccessKit 发布系统无障碍树。 |

```text
Ash Desktop
├── Ash UI Kit → Ash GPUI → winit / wgpu / AccessKit
└── 产品领域客户端 → App Server
```

Ash GPUI 和 Kit 作为独立 crate 放在现有 `app/` Cargo 工作区，随实际调用方引入，运行在桌面窗口所属的进程中。根工作区的 `crates/` 承载共享领域与基础设施，例如 App Server 客户端和快捷键规则。依赖从桌面宿主指向这些共享库。若出现第二个独立图形宿主并实际复用 UI，再评估将共用部分提取到独立的前端工作区。

平台差异优先由上游库吸收。系统菜单、剪贴板、文件选择器等能力通过桌面宿主的窄接口接入，确有缺口时再在对应适配层补平台实现。`wgpu` 提供跨平台图形接口，原生窗口的输入、缩放和读屏体验仍需分平台验证。 [winit 窗口 API](https://docs.rs/winit/latest/winit/window/struct.Window.html) · [wgpu 后端](https://docs.rs/wgpu/latest/wgpu/#backends) · [AccessKit 平台适配](https://github.com/AccessKit/accesskit)

### 一次交互如何变成一帧

1. 窗口事件在 UI 线程转换为逻辑坐标、按键或文本事件，按窗口、焦点与命中结果分发。使用稳定的元素 ID 保留列表项、输入和弹层的交互状态。异步 I/O 在后台执行，结果经事件循环投递给对应窗口；窗口销毁时取消关联工作，并忽略迟到结果。
2. 视图由 Rust 函数组合。状态变化标记需要更新的窗口，布局结果供命中测试、文本插入点、无障碍边界和绘制共同使用。先从单窗口按需重绘开始，再依据测量细分布局或绘制失效。空闲时等待事件，动画期间才持续请求下一帧。
3. 视图生成有顺序的绘制指令，首批覆盖圆角矩形、边框、裁剪、文字和图标。GPU 适配层负责管线、图集、批次、颜色格式和呈现，组件接口只传布局、主题与绘制语义。逻辑尺寸统一在窗口边界按 DPI 换算；缩放变化时同步刷新布局、命中和 IME 候选窗位置。

每个窗口维护自己的呈现表面与焦点。表面尺寸为零时暂停提交，恢复到可用尺寸后重新配置。表面失效时先恢复表面；设备丢失时重建设备及其 GPU 资源，再由现有视图生成一帧。恢复尝试要有上限，持续失败时由宿主给出可操作的错误信息。 [wgpu Surface](https://docs.rs/wgpu/latest/wgpu/struct.Surface.html)

布局先评估 Taffy 的 Flexbox 和文本测量接口；文字先评估 Parley 的字形整形、换行和字体回退，绘制端单独负责字形栅格化与图集。插入点、鼠标选区、读屏文本范围和绘制应使用同一次文本布局的结果。IME 组合态归当前输入控件，提交事件才写入文本；组合期间由输入法消费的按键应先于产品快捷键处理。产品快捷键匹配复用 `ash-keybinding`，命令表及上下文由 Desktop 持有。 [Taffy](https://github.com/DioxusLabs/taffy) · [Parley](https://github.com/linebender/parley) · [Ash 快捷键契约](keybindings.md)

### Kit 的第一批组件

| 组件 | 首批交互契约 |
| --- | --- |
| `Button`、`IconButton` | 共用激活、禁用与焦点行为，图标按钮有可本地化的无障碍名称。 |
| `TextInput` | 单行编辑、选择、剪贴板、撤销和 IME 组合，支持候选窗跟随插入点。 |
| `List` | 稳定条目身份、键盘导航与滚动定位；需要虚拟化时向辅助技术提供条目位置和总数。 |
| `Dialog` | 模态焦点范围、Escape 关闭和焦点恢复，发布标题、描述与可执行动作。 |

Kit 集中定义语义颜色、字体角色、间距、圆角和焦点样式，覆盖浅色、深色、高对比度及减少动态效果。图标消费 Ash 的语义图标标识。焦点、角色、名称、状态与动作由控件声明，框架将其投影到 AccessKit；读屏动作与键盘、鼠标调用同一控件操作。产品负责可本地化文案、功能的无障碍帮助、详细度设置，以及复杂内容的纯文本可访问视图。用至少一种非默认语言验证可见文案和读屏标签。 [AccessKit 应用验收指南](https://github.com/AccessKit/accesskit/blob/main/README-APPLICATION-DEVELOPERS.md)

Desktop 从本地 profile 的 `[gui]` 读取界面偏好及有序快捷键配置，将解析后的主题和交互值注入 Kit。窗口几何、选区与滚动位置属于 GUI 状态。桌面与后端的装配见 [App Server 接入边界](frontend-app-server-boundary.md)。

### 实施顺序与验收

1. **GPU 窗口。** 从 `just ash` 打开现有桌面入口，绘制色块和可读文本。验证缩放、最小化恢复、反复调整尺寸、第二个窗口和关闭清理。记录实际图形后端、适配器和错误路径，并模拟表面失效及设备丢失。平台范围建议先以现有的 macOS、Windows、Linux X11 为验收矩阵，Wayland 另行验证输入法、缩放和读屏。
2. **命令面板。** 用上述 Kit 组件组成首个自托管切片，连接实际本地命令。验证键盘全流程、焦点恢复、中文及日文输入、混排文字的光标和选择、跨屏缩放、高对比度，以及 VoiceOver、NVDA、Orca 对控件名称、列表选择和动作的实际反馈。补充无窗口的布局、命中、键盘与语义树断言。记录首帧时间、空闲重绘次数、输入到可见更新、滚动帧时长和反复开关窗口后的资源占用。
3. **真实产品切片。** 暂定 Chat，从共享 App Server 创建会话、提交输入、呈现流式响应，并验证关闭窗口、断线重连及从后端快照恢复。用实际工作负载决定虚拟化、缓存和更新合并的优先级，再把已验证的组件用于工作区与编辑器。
