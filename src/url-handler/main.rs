use app::App;
use iced::{window, Application, Settings};

mod app;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (400, 200),
            resizable: false,
            always_on_top: true,
            ..Default::default()
        },
        ..Default::default()
    };

    App::run(settings)
}
