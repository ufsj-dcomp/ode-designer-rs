use std::sync::atomic::AtomicU8;
use std::sync::OnceLock;
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use imgui::{Ui, WindowFlags};
use log::Level;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum State {
    #[default]
    Normal,
    Fading {
        num: f32,
        out: bool,
    },
    Dismissed,
}

#[derive(Debug, Clone)]
struct Message {
    body: String,
    level: Level,
    created_at: Instant,
    state: State,
    id: u8,
}

const WIN_SIZE: [f32; 2] = [160f32, -1.0];
const Y_PADDING: f32 = 12.0;
const X_PADDING: f32 = Y_PADDING;
const TIMEOUT: Duration = Duration::from_secs(5);

impl Message {
    fn draw(&mut self, ui: &Ui, x: f32, y: f32) -> Option<f32> {
        let winflags = WindowFlags::NO_RESIZE
            | WindowFlags::NO_MOVE
            /* | WindowFlags::NO_FOCUS_ON_APPEARING
            | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS */
            | WindowFlags::NO_SAVED_SETTINGS
            | WindowFlags::NO_COLLAPSE;
        let tk = ui.push_style_color(imgui::StyleColor::Text, level_color(self.level));
        let (pressed, y) = {
            let _alpha_token = match self.state {
                State::Fading { num, .. } => Some(ui.push_style_var(imgui::StyleVar::Alpha(num))),
                _ => None,
            };
            ui.window(format!(
                "{} {}##{}",
                level_icon(self.level),
                self.level,
                self.id
            ))
            .flags(winflags)
            .position([x, y], imgui::Condition::Always)
            .size(WIN_SIZE, imgui::Condition::Always)
            .build(|| {
                tk.pop();
                ui.text_wrapped(self.body.as_str());
                // Start dismissing the notification
                let pressed = ui.button("Ok");
                (pressed, ui.window_size()[1] + Y_PADDING)
            })
            .expect("Notifications aren't able to be hidden")
        };
        match self.state {
            State::Normal => {
                if self.created_at.elapsed() >= TIMEOUT || pressed {
                    self.state = State::Fading {
                        num: 1.0,
                        out: true,
                    };
                }
            }
            State::Dismissed => return None,
            State::Fading { ref mut num, out } => {
                let delta = ui.io().delta_time;
                if out {
                    *num -= delta;
                    if *num <= 0.0 {
                        self.state = State::Dismissed;
                    }
                } else {
                    *num += delta;
                    if *num >= 1.0 {
                        self.state = State::Normal;
                    }
                }
            }
        }
        Some(y)
    }
}

impl NotificationLogger {
    fn render_messages(&self, ui: &Ui) {
        let mut elements = self.messages.lock().unwrap();

        let parent_size = ui.window_size();
        let x = parent_size[0] - WIN_SIZE[0] - X_PADDING;
        let mut y = 0.0;
        for msg in elements.iter_mut() {
            y += msg.draw(ui, x, y).unwrap_or(0.0);
            // Don't draw windows outside view.
            if y >= parent_size[1] {
                break;
            }
        }
        elements.retain(|s| s.state != State::Dismissed);
    }
}

#[derive(Debug)]
pub struct NotificationLogger {
    max_log_level: Level,
    next_id: AtomicU8,
    messages: Mutex<Vec<Message>>,
}

static NOTIFICATION: OnceLock<NotificationLogger> = OnceLock::new();

/// Renders the recorded messages as ImGUI child windows.
///
/// # Panics
/// This function panics if there is no [`NotificationLogger`] registered.
pub fn render_messages(ui: &Ui) {
    NOTIFICATION.get().unwrap().render_messages(ui)
}

// RGBA
fn level_color(level: Level) -> [f32; 4] {
    let col = match level {
        Level::Error => [1.0, 0.1, 0.1], // Red
        Level::Warn => [0.7, 0.7, 0.2],  // Yellow
        Level::Info => [0.2, 0.2, 1.0],  // Blue
        Level::Debug => [0.2, 1.0, 0.2], // Green
        Level::Trace => [1.0; 3],        // White
    };
    [col[0], col[1], col[2], 1.0f32]
}

fn level_icon(level: Level) -> &'static str {
    match level {
        Level::Error => "",
        Level::Warn => "",
        Level::Info => "",
        Level::Debug => "",
        Level::Trace => "󰙜",
    }
}

impl NotificationLogger {
    /// Creates a new [`NotificationLogger`]
    pub fn new() -> Self {
        Self {
            max_log_level: Level::Warn,
            next_id: Default::default(),
            messages: Default::default(),
        }
    }
    /// Sets the maximum log level.
    /// Check out [this link](https://docs.rs/log/latest/log/#usage) for an explanation of what
    /// this means.
    pub fn with_max_log_level(mut self, level: Level) -> Self {
        self.max_log_level = level;
        self
    }
    /// Initializes [`NotificationLogger`] as the current logger.
    ///
    /// # Panics
    /// This function panics if it has already been called or if a logger has already been set.
    pub fn init(self) {
        log::set_max_level(self.max_log_level.to_level_filter());
        let logger = NOTIFICATION.try_insert(self).unwrap();
        log::set_logger(logger).unwrap();
    }
}

impl log::Log for NotificationLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        dbg!(metadata);
        metadata.target().contains(env!("CARGO_PKG_NAME")) && metadata.level() >= self.max_log_level
    }

    fn log(&self, record: &log::Record) {
        self.messages.lock().unwrap().push(Message {
            body: record.args().to_string(),
            level: record.level(),
            created_at: Instant::now(),
            state: State::Fading {
                num: 0.0,
                out: false,
            },
            id: self
                .next_id
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        })
    }

    fn flush(&self) {
        // Nothing to be done here.
        // The application is responsible  consuming the `Message`s.
    }
}
