use presage::libsignal_service::content::{Content, ContentBody};
use presage::libsignal_service::proto::sync_message::Sent;
use presage::libsignal_service::proto::{SyncMessage, receipt_message};
use presage::store::ContentExt;
use uuid::Uuid;

use super::Thread;

#[derive(Debug, Clone)]
pub struct Message {
    pub timestamp: u64,
    pub sender: Uuid,
    pub body: String,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Sending,
    Failed,
    Sent,
    Delivered,
    Read,
}

pub fn from_content(content: &Content) -> Option<(Thread, Message)> {
    let thread = presage::store::Thread::try_from(content).ok()?;
    let body = match &content.body {
        ContentBody::DataMessage(data_message) => data_message.body.clone(),
        ContentBody::SynchronizeMessage(SyncMessage {
            sent:
                Some(Sent {
                    message: Some(data_message),
                    ..
                }),
            ..
        }) => data_message.body.clone(),
        _ => None,
    }?;

    let message = Message {
        timestamp: content.timestamp(),
        sender: content.metadata.sender.raw_uuid(),
        body,
        status: None,
    };
    Some(((&thread).into(), message))
}

pub fn receipt_from_content(content: &Content) -> Option<(Vec<u64>, Status)> {
    let ContentBody::ReceiptMessage(receipt) = &content.body else {
        return None;
    };
    let status = match receipt.r#type() {
        receipt_message::Type::Delivery => Status::Delivered,
        receipt_message::Type::Read => Status::Read,
        receipt_message::Type::Viewed => return None,
    };
    Some((receipt.timestamp.clone(), status))
}

#[cfg(test)]
mod tests {
    use presage::libsignal_service::content::Metadata;
    use presage::libsignal_service::proto::{DataMessage, GroupContextV2, ReceiptMessage};
    use presage::libsignal_service::protocol::ServiceId;
    use presage::libsignal_service::push_service::DEFAULT_DEVICE_ID;
    use presage::store::ContentsStore;
    use presage_store_sqlite::{OnNewIdentity, SqliteStore};

    use super::*;
    use crate::data::ContactId;

    fn metadata(sender: Uuid, timestamp: u64) -> Metadata {
        let time = chrono::DateTime::from_timestamp_millis(timestamp as i64).unwrap();
        Metadata {
            sender: ServiceId::Aci(sender.into()),
            destination: ServiceId::Aci(sender.into()),
            sender_device: *DEFAULT_DEVICE_ID,
            timestamp: time,
            server_timestamp: time,
            needs_receipt: false,
            unidentified_sender: false,
            was_plaintext: false,
            server_guid: None,
        }
    }

    fn text_message(sender: Uuid, timestamp: u64, body: &str) -> Content {
        Content::from_body(
            DataMessage {
                body: Some(body.into()),
                timestamp: Some(timestamp),
                ..Default::default()
            },
            metadata(sender, timestamp),
        )
    }

    #[test]
    fn maps_incoming_text_to_contact_thread() {
        let sender = Uuid::new_v4();
        let (thread, message) = from_content(&text_message(sender, 1234, "hi")).unwrap();

        assert_eq!(thread, Thread::Contact(ContactId::Aci(sender)));
        assert_eq!(message.sender, sender);
        assert_eq!(message.timestamp, 1234);
        assert_eq!(message.body, "hi");
    }

    #[test]
    fn maps_group_message_to_group_thread() {
        let sender = Uuid::new_v4();
        let master_key = [7u8; 32];
        let content = Content::from_body(
            DataMessage {
                body: Some("hello group".into()),
                timestamp: Some(1234),
                group_v2: Some(GroupContextV2 {
                    master_key: Some(master_key.to_vec()),
                    revision: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            },
            metadata(sender, 1234),
        );

        let (thread, message) = from_content(&content).unwrap();
        assert_eq!(thread, Thread::Group(master_key));
        assert_eq!(message.body, "hello group");
    }

    #[test]
    fn maps_synced_sent_message_to_destination_thread() {
        let own = Uuid::new_v4();
        let destination = Uuid::new_v4();
        let content = Content::from_body(
            SyncMessage {
                sent: Some(Sent {
                    destination_service_id: Some(destination.to_string()),
                    timestamp: Some(9999),
                    message: Some(DataMessage {
                        body: Some("from my phone".into()),
                        timestamp: Some(9999),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            metadata(own, 9999),
        );

        let (thread, message) = from_content(&content).unwrap();
        assert_eq!(thread, Thread::Contact(ContactId::Aci(destination)));
        assert_eq!(message.sender, own);
        assert_eq!(message.timestamp, 9999);
        assert_eq!(message.body, "from my phone");
    }

    #[test]
    fn maps_read_receipt_to_statuses() {
        let content = Content::from_body(
            ReceiptMessage {
                r#type: Some(receipt_message::Type::Read as i32),
                timestamp: vec![1234, 5678],
            },
            metadata(Uuid::new_v4(), 9999),
        );
        assert_eq!(
            receipt_from_content(&content),
            Some((vec![1234, 5678], Status::Read))
        );
    }

    #[test]
    fn ignores_data_message_without_body() {
        let content = Content::from_body(
            DataMessage::default(),
            metadata(Uuid::new_v4(), 1234),
        );
        assert!(from_content(&content).is_none());
    }

    #[tokio::test]
    async fn maps_messages_round_tripped_through_store() {
        let store = SqliteStore::open(":memory:", OnNewIdentity::Trust)
            .await
            .unwrap();

        let sender = Uuid::new_v4();
        let content = text_message(sender, 1234, "stored");
        let thread = presage::store::Thread::try_from(&content).unwrap();
        store.save_message(&thread, content).await.unwrap();

        let stored: Vec<_> = store
            .messages(&thread, ..)
            .await
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        let (mapped_thread, message) = from_content(&stored[0]).unwrap();

        assert_eq!(mapped_thread, Thread::Contact(ContactId::Aci(sender)));
        assert_eq!(message.body, "stored");
        assert_eq!(message.timestamp, 1234);
    }
}
