//! # Basic Usage Example for SSG I18n
//!
//! This example demonstrates how to use the `langweave` library for language detection and translation in a basic static site generation workflow.
//!

use langweave::translator::Translator;
use langweave::{detect_language, translate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic translation
    let translated_text = translate("fr", "Hello")?;
    println!("Translated: {}", translated_text);

    // Language detection
    let detected_language = detect_language("Le chat noir")?;
    println!("Detected language: {}", detected_language);

    // Custom Translator usage
    let custom_translator = Translator::new("de")?;
    let custom_translation = custom_translator.translate("Goodbye")?;
    println!("Custom translation: {}", custom_translation);

    Ok(())
}
