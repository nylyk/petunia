use iced::widget::{column, container, text_input};
use uuid::Uuid;

use crate::data::{self, Contact, Thread};
use crate::theme;
use crate::widget::{Element, message_view};

#[derive(Debug, Clone)]
pub struct Chat {
    pub thread: Thread,
    input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Submit,
}

pub enum Action {
    None,
    SendText(String),
}

impl Chat {
    pub fn new(thread: Thread) -> Self {
        Self {
            thread,
            input: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::InputChanged(input) => {
                self.input = input;
                Action::None
            }
            Message::Submit => {
                let body = self.input.trim().to_string();
                if body.is_empty() {
                    Action::None
                } else {
                    self.input.clear();
                    Action::SendText(body)
                }
            }
        }
    }

    pub fn view<'a>(
        &'a self,
        history: &'a [data::Message],
        aci: Uuid,
        contacts: &'a [Contact],
        title: &str,
    ) -> Element<'a, Message> {
        column![
            message_view::view(history, aci, contacts),
            container(
                text_input(&format!("Message {title}…"), &self.input)
                    .on_input(Message::InputChanged)
                    .on_submit(Message::Submit)
                    .size(13)
                    .padding([7, 10])
                    .style(theme::message_input),
            )
            .padding(8),
        ]
        .into()
    }
}
