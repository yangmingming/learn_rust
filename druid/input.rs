use druid::widget::{Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    text: String,
    text2: String,
}

fn build_ui() -> impl Widget<AppData> {
    // 单行输入框
    let input_box = TextBox::new()
        .with_placeholder("Enter text here")
        .lens(AppData::text);

    let label = Label::new(|data: &AppData, _env: &Env| {
        format!("You typed: {}", data.text)
    });

    // 多行输入框
    let input_box2 = TextBox::multiline()
        .with_placeholder("Enter text2 here")
        .lens(AppData::text2);

    let label2 = Label::new(|data: &AppData, _env: &Env| {
        format!("You typed: {}", data.text2)
    });

    Flex::column()
        .with_child(input_box)
        .with_child(label)
        .with_child(input_box2)
        .with_child(label2)
        .padding(20.0)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Input Example")
        .window_size((300.0, 200.0));

    let data = AppData {
            text: String::new(),
            text2: String::new()
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
