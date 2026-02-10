// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Library Example
//!
//! This program demonstrates the main features of the LangWeave library,
//! including language detection, translation, and error handling.

use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::prelude::*;
use langweave::translator::Translator;

/// The main function of the LangWeave library example.
///
/// This function demonstrates the main features of the LangWeave library,
/// including language detection, translation, and error handling.
///
/// # Parameters
///
/// None
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`:
///   - `Ok(())`: If the LangWeave library example is executed successfully.
///   - `Err(e)`: If an error occurs during the execution of the LangWeave library example.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Library Example\n");

    language_detection_example().await?;
    translation_example()?;
    error_handling_example().await?;
    supported_languages_example()?;

    println!("\n🎉 LangWeave library example completed successfully!");

    Ok(())
}
/// The main function of the LangWeave library example.
///
/// This function demonstrates the main features of the LangWeave library,
/// including language detection, translation, and error handling.
///
/// # Parameters
///
/// None
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`:
///   - `Ok(())`: If the LangWeave library example is executed successfully.
///   - `Err(e)`: If an error occurs during the execution of the LangWeave library example.
///
/// # Examples
///
///

/// Demonstrates language detection capabilities.
pub(crate) async fn language_detection_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Language Detection Example");
    println!("---------------------------------------------");

    let texts = vec![
        "Hello, how are you?",
        "Bonjour, comment allez-vous ?",
        "Hallo, wie geht es dir?",
        "こんにちは、お元気ですか？",
    ];

    for text in texts {
        match detect_language(text).await {
            Ok(lang) => println!(
                "    ✅ Detected language for '{}': {}",
                text, lang
            ),
            Err(e) => println!(
                "    ❌ Error detecting language for '{}': {:?}",
                text, e
            ),
        }
    }

    // Using LanguageDetector directly
    let detector = LanguageDetector::new();
    let mixed_text = "Hello mundo";
    match detector.detect_async(mixed_text).await {
        Ok(lang) => println!(
            "    ✅ Detected language for mixed text '{}': {}",
            mixed_text, lang
        ),
        Err(e) => println!(
            "    ❌ Error detecting language for mixed text '{}': {:?}",
            mixed_text, e
        ),
    }

    Ok(())
}
/// Demonstrates language detection capabilities.
///
/// This function demonstrates the language detection capabilities of the LangWeave library.
/// It prints out the detected language for a list of provided texts, as well as for a mixed text.
///
/// # Parameters
///
/// * None
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`:
///   - `Ok(())`: If the language detection examples are executed successfully.
///   - `Err(e)`: If an error occurs during the execution of the language detection examples.
///
/// # Examples
///
///

/// Demonstrates translation capabilities.
fn translation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Translation Example");
    println!("---------------------------------------------");

    let texts =
        vec![("en", "Hello"), ("fr", "Bonjour"), ("de", "Hallo")];

    for (lang, text) in texts {
        match translate(lang, text) {
            Ok(translated) => println!(
                "    ✅ Translated '{}' to {}: {}",
                text, lang, translated
            ),
            Err(e) => println!(
                "    ❌ Error translating '{}' to {}: {:?}",
                text, lang, e
            ),
        }
    }

    // Using Translator directly
    let translator = Translator::new("fr")?;
    match translator.translate("Goodbye") {
        Ok(translated) => println!(
            "    ✅ Translated 'Goodbye' to French: {}",
            translated
        ),
        Err(e) => println!(
            "    ❌ Error translating 'Goodbye' to French: {:?}",
            e
        ),
    }

    Ok(())
}

/// Demonstrates error handling in various scenarios.
async fn error_handling_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    // Unsupported language
    match translate("xx", "Hello") {
        Ok(_) => println!(
            "    ❓ Unexpected success for unsupported language"
        ),
        Err(e) => println!(
            "    ✅ Expected error for unsupported language: {:?}",
            e
        ),
    }

    // Empty text for language detection
    match detect_language("").await {
        Ok(_) => println!(
            "    ❓ Unexpected success for empty text detection"
        ),
        Err(e) => println!(
            "    ✅ Expected error for empty text detection: {:?}",
            e
        ),
    }

    // Non-existent translation key
    let translator = Translator::new("en")?;
    match translator.translate("NonexistentKey") {
        Ok(_) => println!("    ❓ Unexpected success for non-existent translation key"),
        Err(e) => println!("    ✅ Expected error for non-existent translation key: {:?}", e),
    }

    Ok(())
}

/// Demonstrates the supported languages in the library.
fn supported_languages_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Supported Languages Example");
    println!("---------------------------------------------");

    let languages = supported_languages();
    println!("    Supported languages:");
    for lang in languages {
        println!("    - {}", lang);
    }

    // Check if specific languages are supported
    let check_languages = vec!["en", "fr", "de", "es"];
    for lang in check_languages {
        if is_language_supported(lang) {
            println!("    ✅ {} is supported", lang);
        } else {
            println!("    ❌ {} is not supported", lang);
        }
    }

    Ok(())
}
