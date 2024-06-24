use druid::widget::{Button, Flex, SizedBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

// 定义应用程序状态
#[derive(Clone, Data)]
struct AppState;

// 构建 UI 布局
fn build_ui(button_size: (f64, f64)) -> impl Widget<AppState> {
    // 定义箭头符号
    let arrows = [
        ("↖️", "左前"),
        ("⬆️", "前进"),
        ("↗️", "右前"),
        ("⬅️", "左前"),
        ("⏸️", "暂停"),
        ("➡️", "右转"),
        ("↙️", "左后"),
        ("⬇️", "后退"),
        ("↘️", "右后"),
    ];

    // 创建按钮
    let mut buttons = Vec::new();
    for (arrow, label) in arrows.iter() {
        let label_copy = *label;
        let button = Button::new(format!("{} {}", arrow, label)).on_click(move |_ctx, _data, _env| {
            println!("Button {} clicked", label_copy);
        });
        let sized_button = SizedBox::new(button).width(button_size.0).height(button_size.1);
        buttons.push(sized_button);
    }

    // 使用 Flex 布局
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0)),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0)),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0))
                .with_spacer(8.0)
                .with_child(buttons.remove(0)),
        )
}

fn main() {
    // 按钮大小
    let button_size = (100.0, 50.0);

    // 描述主窗口
    let main_window = WindowDesc::new(move || build_ui(button_size))
        .title(LocalizedString::new("Arrow Buttons"))
        .window_size((400.0, 400.0));

    // 启动应用程序
    AppLauncher::with_window(main_window)
        .launch(AppState)
        .expect("Failed to launch application");
}