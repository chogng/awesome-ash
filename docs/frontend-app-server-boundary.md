# 原生桌面与 App Server 的连接边界

当前 Desktop 入口负责打开系统窗口。以下是接入首个后端工作流时的职责分工。 [Desktop 架构](desktop-architecture.md)

```text
工作台视图 → 产品领域接口 → 协议适配 → ash-app-server-client → App Server
```

- **工作台视图**持有焦点、草稿和选择等交互状态，通过领域接口读取数据、订阅变化和提交意图。Ash UI 和 Kit 接收视图数据及动作回调。
- **产品领域接口**为调用方提供业务操作、状态和错误；协议适配集中转换生成的请求、响应和通知。多个领域接口复用宿主提供的连接。
- **桌面宿主**装配并管理连接，负责启动、取消、重连和关闭。桌面通过共享 App Server 接入后台，Rust 客户端提供初始化、请求配对、通知和关闭；本地运行包的进程入口是独立的 `ash-app-server`。 [Rust 客户端](../crates/app-server-client/README.md)
- **App Server 和后端领域**负责权限、执行及持久化。Session、Thread、Turn 和审批状态从后端投影到界面。 [App Server API](app-server-api.md#唯一外部门禁)

连接恢复后重新获取领域快照并建立订阅。结果未知的修改请求由领域契约决定能否安全重试。窗口关闭时释放窗口订阅和关联任务，共用的后台进程由其生命周期所有者管理。连接和文件 I/O 在 UI 事件循环之外执行。
