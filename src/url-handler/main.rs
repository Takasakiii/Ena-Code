use std::env;

use app::App;
use error_message::ErrorMessage;
use iced::{window, Application, Settings};
mod app;
mod error_message;
mod launcher;

fn main() -> iced::Result {
    let url = env::args().nth(1);

    if let Some(url) = url {
        let settings = Settings {
            window: window::Settings {
                size: (400, 200),
                resizable: false,
                always_on_top: true,
                ..Default::default()
            },
            flags: url,
            ..Default::default()
        };

        App::run(settings)
    } else {
        ErrorMessage::show("Invalid Url", "The url is invalid")
    }
}
