#![allow(dead_code)]

pub trait Button {
    fn render(&self) -> String;
}
pub trait Checkbox {
    fn render(&self) -> String;
}

//------------------- Concrete Products for windows ------------------
struct WindowsButton;
impl Button for WindowsButton {
    fn render(&self) -> String {
        "Rendering Windows Button".to_string()
    }
}

struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn render(&self) -> String {
        "Rendering Windows Checkbox".to_string()
    }
}

//------------------- Concrete Products for macOS ------------------
struct MacOSButton;
impl Button for MacOSButton {
    fn render(&self) -> String {
        "Rendering macOS Button".to_string()
    }
}

struct MacOSCheckbox;
impl Checkbox for MacOSCheckbox {
    fn render(&self) -> String {
        "Rendering macOS Checkbox".to_string()
    }
}
//------------------- Abstract Factory ------------------
pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

//------------------- Concrete Factories ------------------
struct WindowsFactory;
impl GUIFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}

struct MacOSFactory;
impl GUIFactory for MacOSFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacOSButton)
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacOSCheckbox)
    }
}

pub enum OSType {
    Windows,
    Macos,
}

//------------------- Client Code ------------------
pub struct GetGuiFactory;
impl GetGuiFactory {
    pub fn get_factory(os_type: OSType) -> Box<dyn GUIFactory> {
        match os_type {
            OSType::Windows => Box::new(WindowsFactory),
            OSType::Macos => Box::new(MacOSFactory),
        }
    }
}
