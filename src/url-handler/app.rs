use ecode_core::actions;
use iced::{button, pick_list, Align, Button, Column, Length, PickList, Row, Sandbox, Text};

pub struct App {
    all_profiles: Vec<String>,
    selected: String,
    profile_list: pick_list::State<String>,
    confirm_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetProfile(String),
    Confirmed,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let all_profiles = actions::get_profiles_list();
        let fist_item = all_profiles
            .first()
            .cloned()
            .unwrap_or_else(|| "".to_string());

        App {
            all_profiles,
            profile_list: Default::default(),
            selected: fist_item,
            confirm_button: Default::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Ena-Code VS Code Url Handler")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SetProfile(profile) => {
                self.selected = profile;
            }
            Message::Confirmed => {
                println!("{}", self.selected);
            }
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let header_text = Text::new("Select the profile to send the url:");

        let profiles_list = PickList::new(
            &mut self.profile_list,
            &self.all_profiles,
            Some(self.selected.clone()),
            Message::SetProfile,
        )
        .width(Length::Units(250));

        let confirm_button = Button::new(&mut self.confirm_button, Text::new("Confirm"))
            .on_press(Message::Confirmed);

        let column_center = Column::new()
            .width(Length::Fill)
            .align_items(Align::Center)
            .spacing(20)
            .push(header_text)
            .push(profiles_list)
            .push(confirm_button);

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(column_center)
            .align_items(Align::Center)
            .into()
    }
}
