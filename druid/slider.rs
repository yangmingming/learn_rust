use druid::widget::{Flex, Label, Slider};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppData {
    value: f64,
}

fn build_ui() -> impl Widget<AppData> {
    let slider = Slider::new().lens(AppData::value);

    let label = Label::new(|data: &AppData, _env: &druid::Env| {
        format!("Slider value: {:.2}", data.value)
    });

    Flex::column()
        .with_child(slider)
        .with_child(label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Slider Example")
        .window_size((300.0, 100.0));

    let data = AppData { value: 0.0 };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
