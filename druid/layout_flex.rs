use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    text: String,
}

fn build_ui() -> impl Widget<AppData> {
    let input_box = TextBox::new()
        .with_placeholder("Enter text here")
        .lens(AppData::text);

    let label = Label::new(|data: &AppData, _env: &druid::Env| {
        format!("You typed: {}", data.text)
    });

    let button = Button::new("Click me");

    let row1 = Flex::row()
            .with_child(Label::new("row1-1"))
            .with_child(Label::new("row1-2"));

    let row2 = Flex::row()
            .with_child(Label::new("row2-1"))
            .with_child(Label::new("row2-2"));

    Flex::column()
        .with_child(input_box)
        .with_spacer(8.0) // 添加空白空间
        .with_child(label)
        .with_spacer(8.0) // 添加空白空间
        .with_child(button)
        .with_spacer(8.0) // 添加空白空间
        .with_child(row1)
        .with_spacer(8.0) // 添加空白空间
        .with_child(row2)
        .padding(20.0)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Flex Layout Example")
        .window_size((300.0, 200.0));

    let data = AppData { text: String::new() };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
