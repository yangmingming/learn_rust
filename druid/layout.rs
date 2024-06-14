use druid::widget::{Button, Flex, Label, TextBox, Padding, Align};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc, Lens, Data, UnitPoint};
use druid::kurbo::Vec2;

#[derive(Clone, Data, Lens)]
struct AppState {
    input_value: String,
}

fn build_ui() -> impl Widget<AppState> {
    let input_box = TextBox::new().lens(AppState::input_value);
    let space = 15.0;

    let row1 = Flex::row()
        .with_child(input_box)
        .with_default_spacer()
        .with_child(Padding::new(space, Label::new("Label 1")))
        .with_default_spacer()
        .with_child(Padding::new(space, Label::new("Label 2")))
        .with_default_spacer()
        .with_child(Padding::new(space, Label::new("Label 3")))
        .with_default_spacer()
        .with_child(Padding::new(space, Label::new("Label 4")));

    let button1 = Button::new("Button 1");
    let button2 = Button::new("Button 2");
    let button3 = Button::new("Button 3");
    let button4 = Button::new("Button 4");
    let button5 = Button::new("Button 5");
    let button6 = Button::new("Button 6");

    let row2 = Flex::row()
        .with_child(Padding::new(space, button1))
        .with_default_spacer()
        .with_child(Padding::new(space, button2))
        .with_default_spacer()
        .with_child(Padding::new(space, button3))
        .with_default_spacer()
        .with_child(Padding::new(space, button4))
        .with_default_spacer()
        .with_child(Padding::new(space, button5))
        .with_default_spacer()
        .with_child(Padding::new(space, button6));

    let label1 = Label::new("Large Label 1");
    let label2 = Label::new("Large Label 2");

    let row3 = Flex::row()
        .with_child(Padding::new(100.0, Align::horizontal(UnitPoint::CENTER, label1)))
        .with_default_spacer()
        .with_child(Padding::new(50.0, Align::horizontal(UnitPoint::CENTER, label2)));

    let long_label = Label::new("This is a long narrow label");

    Flex::column()
        .with_child(Padding::new(space, Align::vertical(UnitPoint::TOP, row1)))
        .with_child(Padding::new(space, Align::vertical(UnitPoint::CENTER, row2)))
        .with_child(Padding::new(space, Align::vertical(UnitPoint::BOTTOM, row3)))
        .with_child(Padding::new(space, Align::horizontal(UnitPoint::CENTER, long_label)))
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Layout Example")
        .window_size((1000.0, 500.0));

    let initial_state = AppState {
        input_value: String::new(),
    };
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
