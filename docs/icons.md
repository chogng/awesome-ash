# 产品图标

## 1. 资源

仓库自有图稿位于 [resources/icons](../resources/icons)，品牌花朵的原稿为 [ash-mark.svg](../resources/icons/ash-mark.svg)。

## 2. 所有权

Ash GPUI 的图标接入随组件库实现。产品功能选择语义图标，Kit 负责图标与文字的布局以及图标按钮的交互，Ash GPUI 负责绘制。GPU 资源留在绘制适配层。 [Ash GPUI](desktop-architecture.md#ash-gpui)

## 3. 身份与图稿

- 调用方使用语义 ID，资源名只用于定位图稿。同一图稿可用于多个语义 ID，替换图稿可保留既有 ID。
- 单色图标使用主题前景色。彩色图标保留图稿指定的固定颜色，仅对声明为可着色的图层应用主题前景色。
- 交互控件提供本地化的无障碍名称。名称来自操作语义，例如“关闭面板”，装饰图标由控件隐藏于无障碍树。
