# test_notifications.py
import logging
import pytest
from creational.Factory.factory import NotificationFactory, NotificationType, NotificationMessage

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def test_voice_notification():
    """Test para notificación de voz"""
    notification = NotificationFactory.create_notification(NotificationType.VOICE)
    message = notification.notify("This is a voice notification")    
    
    assert message.message == "Voice Message: This is a voice notification"
    assert message.type == NotificationType.VOICE

    logger.info(f"Voice Notification Message: {message.message}")
    

def test_letter_notification():
    """Test para notificación de carta"""
    notification = NotificationFactory.create_notification(NotificationType.LETTER)
    message = notification.notify("This is a letter notification")
    
    assert message.message == "Letter Message: This is a letter notification"
    assert message.type == NotificationType.LETTER
    logger.info(f"Letter Notification Message: {message.message}")
    

def test_unknown_notification_type():
    """Test para tipo de notificación desconocido"""
    with pytest.raises(ValueError, match="Unknown notification type"):
        NotificationFactory.create_notification("unknown_type")