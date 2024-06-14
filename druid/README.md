# druid 使用举例

## 介绍

Druid 是一个纯 Rust 的 GUI 框架，它的设计理念是简洁、高效且易于使用。以下是几个使用 Druid 的开源项目，这些项目可以作为学习和参考的实例。

Druid 是一个纯Rust的GUI框架，不依赖于Web技术，非常适合需要原生界面的应用。
特点：
• 纯Rust实现，不依赖Web技术。
• 原生界面，适合需要高性能和原生体验的应用。

## 不同组件使用说明

### button 按扭

创建方式
```
Button::new("Send Request")
```

运行方式
```
cargo run --bin button
```

### TextBox 输入框

创建方式

```Plain Text
// 单行输入框
TextBox::new()

// 多行输入框
TextBox::multiline()
```

运行方式
```
cargo run --bin input
```

### label 标签
创建方式

```rust
Label::new("Hello, Druid!")
```

运行方式
```
cargo run --bin label
```

### radio 单选框
创建方式

```rust
    let choices = vec![
        ("Option 1".to_string(), "Value 1".to_string()),
        ("Option 2".to_string(), "Value 2".to_string()),
        ("Option 3".to_string(), "Value 3".to_string()),
    ];
RadioGroup::new(choices)
```

运行方式
```
cargo run --bin radio
```

### checkbox 多选框
创建方式

```rust
Checkbox::new
```

运行方式
```
cargo run --bin checkbox
```

### slider 滑块
创建方式
```
Slider::new()
```

运行方式
```
cargo run --bin slider
```

## 布局相关
在 Druid 中，可以使用布局容器来控制元素的位置和排列方式。常用的布局容器包括 `Flex`、`Align` 和 `Padding`。这些容器可以帮助你精确地控制不同元素在界面中的位置和排列。

以下是一些常用布局容器的说明和示例：

### `Flex` 布局
`Flex` 布局容器允许你将子组件垂直或水平排列。你可以使用 `Flex::row` 或 `Flex::column` 来创建水平或垂直布局。

运行方式
```
cargo run --bin layout_flex
```

### `Align` 对齐
`Align` 容器允许你将子组件对齐到特定的位置，例如顶部、中间或底部。

* Align::centered(widget): 将 widget 居中对齐。
* Align::vertical(point, widget): 将 widget 垂直对齐到 point 所指定的位置。
* Align::horizontal(point, widget): 将 widget 水平对齐到 point 所指定的位置。
* UnitPoint 提供了各种对齐点，如 UnitPoint::TOP、UnitPoint::BOTTOM、UnitPoint::LEFT、UnitPoint::RIGHT 等。

运行方式
```
cargo run --bin layout_align
```

### `Padding` 填充
Padding 是一种布局组件，用于在一个 Widget 的周围添加空白区域（填充）。这有助于控制组件之间的间距，使界面更加美观和可读。通过 Padding，你可以在组件的四周添加一定的空白，以防止组件彼此紧贴在一起。通过ascii图片的方式，展示padding后面跟一个参数和4个参数的区别

**单个参数的 ****Padding**

假设我们有一个标签，使用 `.padding(10.0)`，这将在标签的四周添加 10.0 像素的填充。

```Plain Text
+------------------------+
|                        |
|                        |
|    +-------------+     |
|    |             |     |
|    |   Label     |     |
|    |             |     |
|    +-------------+     |
|                        |
|                        |
+------------------------+
```
在这个例子中，`Label` 周围的空白区域表示填充。



**四个参数的 ****Padding**

假设我们有一个按钮，使用 `.padding((10.0, 5.0, 10.0, 5.0))`，这将在按钮的左、上、右、下分别添加 10.0、5.0、10.0 和 5.0 像素的填充。

```Plain Text
+------------------------+
|                        |
|   +---------------+    |
|   |               |    |
|   |    Button     |    |
|   |               |    |
|   +---------------+    |
|                        |
+------------------------+
```
在这个例子中，`Button` 的左、右填充为 10.0 像素，而上、下填充为 5.0 像素。

运行方式
```
cargo run --bin layout_padding
```

### 综合示例

下面是一个综合示例，展示了如何使用 Flex、Align 和 Padding 容器来构建一个复杂的布局：

运行方式
```
cargo run --bin layout
```

## 事件相关
### label显示和输入框内容
1. 创建结构体
2. TextBox通过lens和结构体绑定之后，输入框输入的内容就会更新结构体变量
3. Label::new 接受一个静态字符串或一个闭包。如果提供闭包，它将根据应用程序状态动态更新标签的文本。

```rust
#[derive(Clone, Data, Lens)]
struct AppData {
    text: String,
}

fn build_ui() -> impl Widget<AppData> {
    // 单行输入框
    let input_box = TextBox::new()
        .with_placeholder("Enter text here")
        .lens(AppData::text);

    let label = Label::new(|data: &AppData, _env: &Env| {
        format!("You typed: {}", data.text)
    });

    ...
}
```
### button 点击响应
```rust
    let button = Button::new("Save")
        .on_click(|_ctx: &mut EventCtx, _data: &mut (), _env: &Env| {
            // 执行save操作
            println!("save success");
        });
```


### label标签显示系统时间
1. **数据结构**：
   * `AppState` 结构体只包含一个 `time` 字段，用于存储当前时间的字符串表示。
2. **UI 元素**：
   * `Label::new` 创建一个标签，使用闭包动态设置标签内容为 `AppState` 的 `time` 字段。
   * `TimeController` 绑定到这个 `Label`，以便控制其时间更新逻辑。
3. **时间控制器**：
   * `TimeController` 结构体包含一个 `TimerToken` 字段。
   * `event` 方法处理 `WindowConnected` 事件，在窗口连接时启动定时器，并每秒更新一次时间。
   * `update` 方法在定时器触发时更新 `AppState` 的 `time` 字段，并请求 UI 更新。
4. **main**** 函数**：
   * 创建主窗口，并启动应用程序。

运行方式
```
cargo run --bin controller_label_time
```

### 鼠标点击 TODO

### 键盘点击 TODO
