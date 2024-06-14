use druid::widget::{Checkbox, Flex, Label};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

// 定义应用程序状态
#[derive(Clone, Data, Lens)]
struct AppState {
    option1: bool,
    option2: bool,
    option3: bool,
}

fn build_ui() -> impl Widget<AppState> {
    // 创建多选框
    let checkbox1 = Checkbox::new("Option 1").lens(AppState::option1);
    let checkbox2 = Checkbox::new("Option 2").lens(AppState::option2);
    let checkbox3 = Checkbox::new("Option 3").lens(AppState::option3);

    // 创建布局
    let layout = Flex::column()
        .with_child(checkbox1)
        .with_spacer(10.0)
        .with_child(checkbox2)
        .with_spacer(10.0)
        .with_child(checkbox3)
        .with_spacer(10.0)
        .with_child(Label::new(|data: &AppState, _env: &_| {
            format!(
                "Option 1: {}, Option 2: {}, Option 3: {}",
                data.option1, data.option2, data.option3
            )
        }));

    layout
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Checkbox Example")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        option1: false,
        option2: false,
        option3: false,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
