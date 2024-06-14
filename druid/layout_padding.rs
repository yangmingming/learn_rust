use druid::widget::{Button, Flex, Label, Padding};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

// 定义应用程序状态
#[derive(Clone, Data, Lens)]
struct AppState {
    label_text: String,
}

fn build_ui() -> impl Widget<AppState> {
    // 创建标签和按钮，并添加填充
    let label = Label::new("This is a label").padding(10.0);
    let button = Button::new("Click me").padding((10.0, 5.0, 10.0, 5.0));

    // 创建布局
    Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(button)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Padding Example")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        label_text: "Hello, Druid!".into(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}