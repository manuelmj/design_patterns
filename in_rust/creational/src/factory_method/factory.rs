#![allow(dead_code)]

#[derive(Debug)]
pub enum MessageType {
    Voice,
    Letter,
}

#[derive(Debug)]
pub struct NotificationMesage {
    pub message: String,
    pub message_type: MessageType,
}

pub trait Notification {
    fn notify(&self) -> NotificationMesage;
}

pub struct VoiceNotification;

impl Notification for VoiceNotification {
    fn notify(&self) -> NotificationMesage {
        NotificationMesage {
            message: String::from("Voice Notification"),
            message_type: MessageType::Voice,
        }
    }
}

pub struct LetterNotification;

impl Notification for LetterNotification {
    fn notify(&self) -> NotificationMesage {
        NotificationMesage {
            message: String::from("Letter Notification"),
            message_type: MessageType::Letter,
        }
    }
}

pub struct NotificationFactory;

impl NotificationFactory {
    pub fn create_notification(notification_type: MessageType) -> Box<dyn Notification> {
        match notification_type {
            MessageType::Voice => Box::new(VoiceNotification),
            MessageType::Letter => Box::new(LetterNotification),
        }
    }
}
