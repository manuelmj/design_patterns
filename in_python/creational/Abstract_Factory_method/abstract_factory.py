from abc import ABC, abstractmethod
from enum import Enum

class Button(ABC):
    @abstractmethod
    def paint(self) -> str:
        pass

class Checkbox(ABC):
    @abstractmethod
    def paint(self) -> str:
        pass


#implementaciones concretas de los productos para Widows
class WinButton(Button):
    def paint(self) -> str:
        return "Rendering a button in Windows style"

class WinCheckbox(Checkbox):
    def paint(self) -> str:
        return "Rendering a checkbox in Windows style"
    
#implementaciones concretas de los productos para MacOS
class MacButton(Button):
    def paint(self) -> str:
        return "Rendering a button in MacOS style"

class MacCheckbox(Checkbox):
    def paint(self) -> str:
        return "Rendering a checkbox in MacOS style"


# --- Abstract Factory --- 
class GUIFactory(ABC):
    @abstractmethod
    def create_button(self) -> Button:
        pass

    @abstractmethod
    def create_checkbox(self) -> Checkbox:
        pass


# --fabricas concretas ---

class WinFactory(GUIFactory):
    def create_button(self) -> Button:
        return WinButton()

    def create_checkbox(self) -> Checkbox:
        return WinCheckbox()
    
class MacFactory(GUIFactory):
    def create_button(self) -> Button:
        return MacButton()

    def create_checkbox(self) -> Checkbox:
        return MacCheckbox()
    


class OSType(Enum):
    WINDOWS = "Windows"
    MACOS = "MacOS"


class GetGUIFactory:
    @staticmethod
    def get_factory(os_type: OSType) -> GUIFactory:
        if os_type == OSType.WINDOWS:
            return WinFactory()
        elif os_type == OSType.MACOS:
            return MacFactory()
        else:
            raise ValueError(f"Unknown OS type: {os_type}") 