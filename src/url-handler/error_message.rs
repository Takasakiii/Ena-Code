use iced::{
    executor, window, Alignment, Application, Column, Command, Length, Row, Settings, Text,
};

#[derive(Default)]
pub struct ErrorFlags {
    pub title: String,
    pub message: String,
}

pub struct ErrorMessage {
    flags: ErrorFlags,
}

impl Application for ErrorMessage {
    type Executor = executor::Default;

    type Message = ();

    type Flags = ErrorFlags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let error_message = ErrorMessage { flags };

        let command = Command::none();
        (error_message, command)
    }

    fn title(&self) -> String {
        self.flags.title.clone()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let header_text = Text::new(&self.flags.message);

        let center_column = Column::new()
            .push(header_text)
            .width(Length::Fill)
            .align_items(Alignment::Center);

        let row_center = Row::new()
            .push(center_column)
            .height(Length::Fill)
            .align_items(Alignment::Center);

        row_center.into()
    }
}

impl ErrorMessage {
    pub fn show(title: &str, message: &str) -> iced::Result {
        let settings = Settings {
            flags: ErrorFlags {
                title: title.to_owned(),
                message: message.to_owned(),
            },
            window: window::Settings {
                size: (300, 50),
                always_on_top: true,
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        };

        Self::run(settings)
    }
}
