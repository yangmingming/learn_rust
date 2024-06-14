use druid::widget::{Flex, Label, RadioGroup};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    choice: String,
}

fn build_ui() -> impl Widget<AppData> {
    let choices = vec![
        ("Option 1".to_string(), "Value 1".to_string()),
        ("Option 2".to_string(), "Value 2".to_string()),
        ("Option 3".to_string(), "Value 3".to_string()),
    ];
    let radio = RadioGroup::new(choices).lens(AppData::choice);

    let label = Label::new(|data: &AppData, _env: &druid::Env| {
        format!("You selected: {}", data.choice)
    });

    Flex::column()
        .with_child(radio)
        .with_child(label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("RadioGroup Example")
        .window_size((300.0, 200.0));

    let data = AppData { choice: "Value 1".to_string() };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
