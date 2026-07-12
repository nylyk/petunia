pub mod chat;

use std::collections::HashMap;

use iced::Fill;
use iced::widget::{container, text};
use uuid::Uuid;

pub use chat::Chat;

use crate::data::{Contact, Message, Thread};
use crate::widget::Element;

#[derive(Debug, Clone)]
pub struct Pane {
    pub buffer: Buffer,
}

#[derive(Debug, Clone)]
pub enum Buffer {
    Empty,
    Chat(Chat),
}

impl Pane {
    pub fn empty() -> Self {
        Self {
            buffer: Buffer::Empty,
        }
    }

    pub fn chat(thread: Thread) -> Self {
        Self {
            buffer: Buffer::Chat(Chat::new(thread)),
        }
    }

    pub fn thread(&self) -> Option<&Thread> {
        match &self.buffer {
            Buffer::Chat(chat) => Some(&chat.thread),
            Buffer::Empty => None,
        }
    }

    pub fn update(&mut self, message: chat::Message) -> chat::Action {
        match &mut self.buffer {
            Buffer::Chat(chat) => chat.update(message),
            Buffer::Empty => chat::Action::None,
        }
    }

    pub fn view<'a>(
        &'a self,
        histories: &'a HashMap<Thread, Vec<Message>>,
        aci: Uuid,
        contacts: &'a [Contact],
    ) -> Element<'a, chat::Message> {
        match &self.buffer {
            Buffer::Empty => container(text("Select a chat from the sidebar").size(14))
                .center(Fill)
                .into(),
            Buffer::Chat(chat) => {
                let history = histories
                    .get(&chat.thread)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                chat.view(history, aci, contacts)
            }
        }
    }
}
