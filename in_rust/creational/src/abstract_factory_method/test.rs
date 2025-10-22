#[cfg(test)]
mod abstract_factory_tests {
    use crate::abstract_factory_method::abstract_factory::{
        Button, Checkbox, GUIFactory, GetGuiFactory, OSType,
    };
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
    fn test_abstract_factory_windows() {
        setup_logging();
        let factory: Box<dyn GUIFactory> = GetGuiFactory::get_factory(OSType::Windows);
        let button: Box<dyn Button> = factory.create_button();
        let checkbox: Box<dyn Checkbox> = factory.create_checkbox();
        let button_result: String = button.render(); // Output: Rendering Windows Button
        let checkbox_result: String = checkbox.render(); // Output: Rendering Windows Checkbox

        assert_eq!(button_result, "Rendering Windows Button".to_string());
        info!("Button Rendered: {}", button_result);
        assert_eq!(checkbox_result, "Rendering Windows Checkbox".to_string());
        info!("Checkbox Rendered: {}", checkbox_result);
    }

    #[test]
    fn test_abstract_factory_macos() {
        setup_logging();
        let factory: Box<dyn GUIFactory> = GetGuiFactory::get_factory(OSType::Macos);
        let button: Box<dyn Button> = factory.create_button();
        let checkbox: Box<dyn Checkbox> = factory.create_checkbox();
        let button_result: String = button.render(); // Output: Rendering macOS Button
        let checkbox_result: String = checkbox.render(); // Output: Rendering macOS Checkbox

        assert_eq!(button_result, "Rendering macOS Button".to_string());
        info!("Button Rendered: {}", button_result);
        assert_eq!(checkbox_result, "Rendering macOS Checkbox".to_string());
        info!("Checkbox Rendered: {}", checkbox_result);
    }
}
