from abc import ABC, abstractmethod
from enum import Enum
from pydantic import BaseModel



# Tipo de vehículos usando Enum
class NotificationType(Enum):
    VOICE = "vocie"
    LETTER = "letter"
    

class NotificationMessage(BaseModel):
    message: str
    type: NotificationType

class Notification(ABC):
    @abstractmethod
    def notify(self, message: str) -> NotificationMessage:
        pass


class VoiceMessage(Notification):
    def notify(self, message: str) -> NotificationMessage:
        return NotificationMessage(message=f"Voice Message: {message}", type=NotificationType.VOICE)
    
class LetterMessage(Notification):
    def notify(self, message: str) -> NotificationMessage:
        return NotificationMessage(message=f"Letter Message: {message}", type=NotificationType.LETTER)
    


class NotificationFactory:

    @staticmethod
    def create_notification(notification_type: NotificationType) -> Notification:
        if notification_type == NotificationType.VOICE:
            return VoiceMessage()
        elif notification_type == NotificationType.LETTER:
            return LetterMessage()
        else:
            raise ValueError(f"Unknown notification type: {notification_type}")    




if __name__ == "__main__":
    voice_notification = NotificationFactory.create_notification(NotificationType.VOICE)
    letter_notification = NotificationFactory.create_notification(NotificationType.LETTER)

    print(voice_notification.notify("This is a voice notification"))
    print(letter_notification.notify("This is a letter notification"))