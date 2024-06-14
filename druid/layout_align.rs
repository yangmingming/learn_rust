use druid::widget::{Align, Button, Label};
use druid::UnitPoint;
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

/*
	•	Align::centered(widget): 将 widget 居中对齐。
	•	Align::vertical(point, widget): 将 widget 垂直对齐到 point 所指定的位置。
	•	Align::horizontal(point, widget): 将 widget 水平对齐到 point 所指定的位置。
	•	UnitPoint 提供了各种对齐点，如 UnitPoint::TOP、UnitPoint::BOTTOM、UnitPoint::LEFT、UnitPoint::RIGHT 等。
 */
fn build_ui() -> impl Widget<()> {
    let label = Label::new("Centered Label");

    // 居中对齐
    // Align::centered(label)

    // 顶部垂直对齐
    Align::vertical(UnitPoint::TOP, label)

    // 底部水平对齐
    // Align::horizontal(UnitPoint::BOTTOM, label)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Align Layout Example")
        .window_size((300.0, 200.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(())
        .expect("Failed to launch application");
}
