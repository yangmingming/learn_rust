use druid::{AppLauncher, Widget, WindowDesc, widget::{Button, Flex}, Env, EventCtx};
use reqwest::blocking;

fn build_ui() -> impl Widget<()> {
    let button = Button::new("Send Request")
        .on_click(|_ctx: &mut EventCtx, _data: &mut (), _env: &Env| {
            let response = blocking::get("https://httpbin.org/get")
                .and_then(|r| r.text())
                .unwrap_or_else(|_| "Failed to fetch data".to_string());
            println!("{}", response);
        });

    let layout = Flex::column()
        .with_child(button);

    layout
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Druid Example");

    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}


