use iced::widget::{column, container, image, text};
use iced::{Center, Fill};

use crate::widget::{Element, qr};

pub struct Linking {
    state: State,
}

enum State {
    Connecting,
    Url(image::Handle),
    Failed(String),
}

impl Linking {
    pub fn new() -> Self {
        Self {
            state: State::Connecting,
        }
    }

    pub fn set_url(&mut self, url: &str) {
        self.state = match qr::handle(url) {
            Some(handle) => State::Url(handle),
            None => State::Failed("failed to render the provisioning QR code".into()),
        };
    }

    pub fn fail(&mut self, error: String) {
        self.state = State::Failed(error);
    }

    pub fn view<Message: 'static>(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = match &self.state {
            State::Connecting => text("Connecting to Signal…").size(16).into(),
            State::Url(handle) => column![
                text("Link Petunia to your phone").size(20),
                text("Open Signal on your phone, go to Settings, Linked devices, and scan this code.")
                    .size(14),
                image(handle.clone()),
            ]
            .spacing(16)
            .align_x(Center)
            .into(),
            State::Failed(error) => column![
                text("Linking failed").size(20),
                text(error).size(14),
                text("Restart Petunia to try again.").size(14),
            ]
            .spacing(8)
            .align_x(Center)
            .into(),
        };
        container(content).center(Fill).into()
    }
}
