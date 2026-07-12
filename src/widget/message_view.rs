use chrono::{DateTime, Local};
use iced::Fill;
use iced::widget::{column, row, scrollable, text};
use uuid::Uuid;

use super::Element;
use crate::data::{Contact, Message, Status, contact_name};

pub fn view<'a, M: 'a>(messages: &'a [Message], aci: Uuid, contacts: &'a [Contact]) -> Element<'a, M> {
    let rows = messages.iter().map(|message| {
        let own = message.sender == aci;
        let mut item = row![
            text(format_time(message.timestamp))
                .size(12)
                .style(text::secondary),
            text(sender_name(message.sender, aci, contacts))
                .size(14)
                .style(if own { text::primary } else { text::success }),
            text(&message.body).size(14),
        ]
        .spacing(8);
        if let Some(status) = message.status {
            item = item.push(text(status_label(status)).size(12).style(
                if status == Status::Failed {
                    text::danger
                } else {
                    text::secondary
                },
            ));
        }
        item.into()
    });

    scrollable(column(rows).spacing(4).padding(8).width(Fill))
        .anchor_bottom()
        .height(Fill)
        .into()
}

fn status_label(status: Status) -> &'static str {
    match status {
        Status::Sending => "sending…",
        Status::Failed => "failed to send",
        Status::Sent => "sent",
        Status::Delivered => "delivered",
        Status::Read => "read",
    }
}

fn sender_name(sender: Uuid, aci: Uuid, contacts: &[Contact]) -> String {
    if sender == aci {
        return "You".into();
    }
    contact_name(contacts, sender)
        .map(str::to_string)
        .unwrap_or_else(|| sender.to_string()[..8].to_string())
}

fn format_time(timestamp: u64) -> String {
    let Some(time) = DateTime::from_timestamp_millis(timestamp as i64) else {
        return String::new();
    };
    let time = time.with_timezone(&Local);
    if time.date_naive() == Local::now().date_naive() {
        time.format("%H:%M").to_string()
    } else {
        time.format("%b %d %H:%M").to_string()
    }
}
