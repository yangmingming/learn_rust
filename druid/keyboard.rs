// use druid::widget::{Flex, Label, TextBox};
// use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};

use druid::widget::prelude::*;
use druid::widget::{Controller, CrossAxisAlignment, Flex, Label, List, Scroll, SizedBox, TextBox};
use druid::{
    theme, AppLauncher, Color, Data, FontDescriptor, KeyEvent, Lens, Location, Modifiers,
    MouseButton, MouseEvent, WidgetExt, WindowDesc,
};
use std::sync::Arc;

#[derive(Clone, Data, Lens)]
struct AppData {
    text: String,
    events: Arc<Vec<LoggedEvent>>,      // 事件列表
}

struct EventLogger<F: Fn(&Event) -> bool> {
    filter: F,
}

impl<W: Widget<AppData>, F: Fn(&Event) -> bool> Controller<AppData, W> for EventLogger<F> {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        // Every time this controller receives an event we check `f()`.
        // If `f()` returns true it means that we can add it to the log,
        // if not then we can skip it.
        if (self.filter)(event) {
            if let Some(to_log) = LoggedEvent::try_from_event(event, data.events.len()) {
                Arc::make_mut(&mut data.events).push(to_log);
            }
        }
        // Always pass on the event!
        child.event(ctx, event, data, env)
    }
}

/// The types of events we display
#[derive(Clone, Copy, Data, PartialEq)]
enum EventType {
    KeyDown,
    KeyUp,
    MouseDown,
    MouseUp,
    Wheel,
}


/// A type that represents any logged event shown in the list
#[derive(Clone, Data)]
struct LoggedEvent {
    typ: EventType,
    number: usize,
    // To see what #[data(ignore)] does look at the docs.rs page on `Data`:
    // https://docs.rs/druid/latest/druid/trait.Data.html
    #[data(ignore)]
    mouse: Option<MouseEvent>,
    #[data(ignore)]
    key: Option<KeyEvent>,
}

/// Here we implement all the display elements of the log entry.
/// We have one method for every attribute we want to show.
/// This is not very interesting it is mostly just getting the data
/// from the events and handling `None` values.
impl LoggedEvent {
    fn try_from_event(event: &Event, number: usize) -> Option<Self> {
        let to_log = match event {
            Event::MouseUp(mouse) => {
                println!("{:?}", mouse.clone());
                Some((EventType::MouseUp, Some(mouse.clone()), None))
            }
            Event::MouseDown(mouse) => {
                println!("{:?}", mouse.clone());
                Some((EventType::MouseDown, Some(mouse.clone()), None))
            }
            Event::Wheel(mouse) => Some((EventType::Wheel, Some(mouse.clone()), None)),
            Event::KeyUp(key) => {
                println!("{:?}", key.clone());
                Some((EventType::KeyUp, None, Some(key.clone())))
            },
            Event::KeyDown(key) => {
                println!("{:?}", key.clone());
                Some((EventType::KeyDown, None, Some(key.clone())))
            },
            _ => None,
        };

        to_log.map(|(typ, mouse, key)| LoggedEvent {
            typ,
            number,
            mouse,
            key,
        })
    }

    fn number(&self) -> String {
        self.number.to_string()
    }

    fn name(&self) -> String {
        match self.typ {
            EventType::KeyDown => "KeyDown",
            EventType::KeyUp => "KeyUp",
            EventType::MouseDown => "MouseDown",
            EventType::MouseUp => "MouseUp",
            EventType::Wheel => "Wheel",
        }
        .into()
    }

    fn mouse_pos(&self) -> String {
        self.mouse
            .as_ref()
            .map(|m| format!("{:.2}", m.pos))
            .unwrap_or_default()
    }

    fn wheel_delta(&self) -> String {
        self.mouse
            .as_ref()
            .filter(|_| self.typ == EventType::Wheel)
            .map(|m| format!("({:.1}, {:.1})", m.wheel_delta.x, m.wheel_delta.y))
            .unwrap_or_default()
    }

    fn mouse_button(&self) -> String {
        self.mouse
            .as_ref()
            .map(|m| {
                match m.button {
                    MouseButton::Left => "Left",
                    MouseButton::Right => "Right",
                    MouseButton::X1 => "X1",
                    MouseButton::X2 => "X2",
                    MouseButton::None => "",
                    MouseButton::Middle => "Middle",
                }
                .into()
            })
            .unwrap_or_default()
    }

    fn click_count(&self) -> String {
        self.mouse
            .as_ref()
            .map(|m| m.count.to_string())
            .unwrap_or_default()
    }

    fn key(&self) -> String {
        self.key
            .as_ref()
            .map(|k| k.key.to_string())
            .unwrap_or_default()
    }

    fn code(&self) -> String {
        self.key
            .as_ref()
            .map(|k| k.code.to_string())
            .unwrap_or_default()
    }

    fn modifiers(&self) -> String {
        self.key
            .as_ref()
            .map(|k| modifiers_string(k.mods))
            .or_else(|| self.mouse.as_ref().map(|m| modifiers_string(m.mods)))
            .unwrap_or_default()
    }

    fn location(&self) -> String {
        match self.key.as_ref().map(|k| k.location) {
            None => "",
            Some(Location::Standard) => "Standard",
            Some(Location::Left) => "Left",
            Some(Location::Right) => "Right",
            Some(Location::Numpad) => "Numpad",
        }
        .into()
    }

    fn is_repeat(&self) -> String {
        if self.key.as_ref().map(|k| k.repeat).unwrap_or(false) {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }
}

fn build_ui() -> impl Widget<AppData> {
    // 多行输入框, 指定控制器 controller, TODO 固定长度
    let input_box = TextBox::multiline()
        .with_placeholder("Enter text here")
        .lens(AppData::text)
        .controller(EventLogger {
            // 事件过滤
            filter: |event| matches!(event, Event::KeyDown(_) | Event::KeyUp(_) | Event::MouseUp(_) | Event::MouseDown(_)),
        });

    let label = Label::new(|data: &AppData, _env: &Env| {
        format!("You typed: {}", data.text)
    });

    Flex::column()
        .with_child(input_box)
        .with_child(label)
        .padding(20.0)
}

fn main() {
    // 界面创建函数
    let main_window = WindowDesc::new(build_ui)
        .title("Input Example")
        .window_size((300.0, 100.0));

    // events 初始化
    let data = AppData { text: String::new(),
        events: Arc::new(Vec::new()),};

    // 绑定 AppData
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Failed to launch application");
}

fn modifiers_string(mods: Modifiers) -> String {
    let mut result = String::new();
    if mods.shift() {
        result.push_str("Shift ");
    }
    if mods.ctrl() {
        result.push_str("Ctrl ");
    }
    if mods.alt() {
        result.push_str("Alt ");
    }
    if mods.meta() {
        result.push_str("Meta ");
    }
    if result.is_empty() {
        result.push_str("None");
    }
    result
}
