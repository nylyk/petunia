use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use super::Command;
use crate::data::{Contact, Group, Message, Status, Thread};

#[derive(Debug, Clone)]
pub enum Event {
    Ready(UnboundedSender<Command>),
    LinkUrl(String),
    Linked {
        aci: Uuid,
    },
    Contacts {
        contacts: Vec<Contact>,
        groups: Vec<Group>,
    },
    History {
        thread: Thread,
        messages: Vec<Message>,
    },
    Message {
        thread: Thread,
        message: Message,
    },
    MessageStatus {
        timestamps: Vec<u64>,
        status: Status,
    },
    Error(String),
}
