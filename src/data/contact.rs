use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Contact {
    pub uuid: Uuid,
    pub name: String,
}

impl From<presage::model::contacts::Contact> for Contact {
    fn from(contact: presage::model::contacts::Contact) -> Self {
        Self {
            uuid: contact.uuid,
            name: contact.name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Group {
    pub master_key: [u8; 32],
    pub title: String,
}

pub fn contact_name(contacts: &[Contact], uuid: Uuid) -> Option<&str> {
    contacts
        .iter()
        .find(|contact| contact.uuid == uuid)
        .map(|contact| contact.name.as_str())
        .filter(|name| !name.is_empty())
}
