use druid::widget::Label;
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

fn build_ui() -> impl Widget<()> {
    Label::new("Hello, Druid!")
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Label Example")
        .window_size((300.0, 100.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
        .expect("Failed to launch application");
}
