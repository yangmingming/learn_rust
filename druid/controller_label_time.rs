use druid::widget::{Label, Flex};
use druid::{AppLauncher, Data, Env, EventCtx, Lens, TimerToken, Widget, WidgetExt, WindowDesc, UpdateCtx};
use std::time::Duration;
use chrono::Local;

#[derive(Clone, Data, Lens)]
struct AppState {
    time: String,
}

fn build_ui() -> impl Widget<AppState> {
    let time_label = Label::new(|data: &AppState, _env: &Env| {
        format!("{}", data.time)
    })
    .controller(TimeController {
        timer: TimerToken::INVALID,
    });

    Flex::column()
        .with_child(time_label)

}

struct TimeController {
    timer: TimerToken,
}

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for TimeController {
    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        child.update(ctx, old_data, data, env);
    }

    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppState, env: &Env) {
        match event {
            druid::Event::WindowConnected => {
                self.timer = ctx.request_timer(Duration::from_secs(1));
            }
            druid::Event::Timer(id) if *id == self.timer => {
                data.time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                self.timer = ctx.request_timer(Duration::from_secs(1));
                ctx.request_update();
            }
            _ => (),
        }

        child.event(ctx, event, data, env);
    }
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Current Time")
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
