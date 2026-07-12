use iced::widget::{button, column, container, scrollable, text};
use iced::{Fill, Shrink};

use super::Element;
use crate::data::{Contact, ContactId, Group, Thread};
use crate::theme;

pub fn view<'a>(contacts: &'a [Contact], groups: &'a [Group]) -> Element<'a, Thread> {
    let mut items = column![].spacing(2).padding(8);

    if !contacts.is_empty() {
        items = items.push(header("Contacts"));
        for contact in contacts {
            let label = if contact.name.is_empty() {
                short_uuid(contact)
            } else {
                contact.name.clone()
            };
            items = items.push(entry(label, Thread::Contact(ContactId::Aci(contact.uuid))));
        }
    }

    if !groups.is_empty() {
        items = items.push(header("Groups"));
        for group in groups {
            items = items.push(entry(group.title.clone(), Thread::Group(group.master_key)));
        }
    }

    if contacts.is_empty() && groups.is_empty() {
        items = items.push(
            text("Waiting for contacts to sync…")
                .size(13)
                .style(text::secondary),
        );
    }

    container(scrollable(items.width(Fill)).height(Fill))
        .width(220)
        .style(theme::sidebar)
        .into()
}

fn header(label: &str) -> Element<'_, Thread> {
    text(label).size(12).style(text::secondary).into()
}

fn entry<'a>(label: String, thread: Thread) -> Element<'a, Thread> {
    button(text(label).size(14).height(Shrink))
        .on_press(thread)
        .width(Fill)
        .padding([4, 6])
        .style(button::text)
        .into()
}

fn short_uuid(contact: &Contact) -> String {
    contact.uuid.to_string()[..8].to_string()
}
