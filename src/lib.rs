// Import the anime module
mod anime;

// Re-export public functions from the anime module
pub use anime::{
    arrow, arrow_brackets, bouncing_equals, custom, custom_loading, dots, dots_spinner, flint,
    hide, loading_bar, mini_dots_spinner, pulse, show, spinner,
};

// Add your unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner() {
        // Basic test to ensure the spinner doesn't panic
        spinner("", "", 1, 100);
        hide();
        show();
    }

    #[test]
    fn test_loading_bar() {
        // Basic test for loading bar functionality
        loading_bar("Test", "", 5, 50);
        hide();
        show();
    }
}
