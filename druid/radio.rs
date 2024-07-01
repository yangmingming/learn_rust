use druid::widget::{Flex, Label, RadioGroup};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

// 自定义类型
#[derive(Debug, Copy, Clone, PartialEq, Data)]
enum ControlMode {
    ModeNone = 0,
    ModeConduct = 1,      // 指挥式
    ModeStep = 2,         // 步进式
    ModeMax = 3,          // 无效数据，仅作为最大范围使用
}

#[derive(Clone, Data, Lens)]
struct AppData {
    choice: String,
    mode: ControlMode,
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

    const FLEX_TYPE_OPTIONS: [(&str, ControlMode); 2] =
    [("步进式", ControlMode::ModeStep), ("指挥式", ControlMode::ModeConduct)];
    let radio2 = RadioGroup::row(FLEX_TYPE_OPTIONS.to_vec()).lens(AppData::mode);

    Flex::column()
        .with_child(radio)
        .with_child(label)
        .with_child(radio2)
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("RadioGroup Example")
        .window_size((300.0, 200.0));

    let data = AppData {
        choice: "Value 1".to_string(),
        mode: ControlMode::ModeConduct,
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}
