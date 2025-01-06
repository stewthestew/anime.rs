// Import the anime module
mod anime;

// Re-export public functions from the anime module
pub use anime::{
    spinner, arrow, arrow_brackets, dots, dots_spinner, mini_dots_spinner, pulse,
    loading_bar, show, hide, flint, bouncing_equals
};

// Add your unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner() {
        // Basic test to ensure the spinner doesn't panic
        spinner(1, 100);
        hide();
        show();
    }

    #[test]
    fn test_loading_bar() {
        // Basic test for loading bar functionality
        loading_bar("Test", 5, 50);
        hide();
        show();
    }
}
