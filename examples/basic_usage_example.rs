//! # Basic Usage Example for SSG I18n
//!
//! This example demonstrates how to use the `langweave` library for language detection and translation in a basic static site generation workflow.
//!

use langweave::translator::Translator;
use langweave::{detect_language, translate};

/// This is the main function for the basic usage example of the `langweave` library.
/// It demonstrates how to use the `langweave` library for language detection and translation in a basic static site generation workflow.
///
/// # Arguments
///
/// * None
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`: A `Result` indicating whether the execution was successful or an error occurred.
///
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic translation
    let translated_text = translate("fr", "Hello")?;
    println!("Translated: {}", translated_text);

    let detected_language = detect_language("Le chat noir").await?;
    println!("Detected language: {}", detected_language);

    // Custom Translator usage
    let custom_translator = Translator::new("de")?;
    let custom_translation = custom_translator.translate("Goodbye")?;
    println!("Custom translation: {}", custom_translation);

    Ok(())
}
