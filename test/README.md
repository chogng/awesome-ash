# Ash 测试

测试随 Rust、Python 或 JavaScript 的职责模块维护。这里保存跨套件共享夹具和 [Agent 任务评测](agent-eval/README.md)。

- 桌面构建用 `just build-ash`，原生窗口验收用 `just ash`。
- Rust 单元和集成测试用 `just test <package>`，真实终端场景用 `just test-tui`。
- 构建与发布工具测试用 `just test-python build scripts code`。
- Remote 安装和 SSH 流程由 `build/release/verify_remote.py` 运行。
