#[cfg(test)]
mod factory_method_tests {
    use crate::factory_method::factory::{MessageType, NotificationFactory};
    use tracing::{Level, info};
    use tracing_subscriber::fmt;

    // Setup inicial para configurar el logging
    fn setup_logging() {
        let _ = fmt()
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();
    }

    #[test]
    fn test_voice_notification() {
        setup_logging();
        let notification = NotificationFactory::create_notification(MessageType::Voice);
        let message = notification.notify();
        assert_eq!(message.message, "Voice Notification");

        match message.message_type {
            MessageType::Voice => info!("Voice message type confirmed: {:?}", message),
            _ => panic!("Expected Voice message type"),
        }
    }

    #[test]
    fn test_letter_notification() {
        setup_logging();
        let notification = NotificationFactory::create_notification(MessageType::Letter);
        let message = notification.notify();
        assert_eq!(message.message, "Letter Notification");
        match message.message_type {
            MessageType::Letter => info!("Letter message type confirmed: {:?}", message),
            _ => panic!("Expected Letter message type"),
        }
    }
}
