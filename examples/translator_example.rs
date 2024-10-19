// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Translator Example
//!
//! This program demonstrates the usage of the Translator struct
//! in the LangWeave library, showcasing its initialization,
//! translation capabilities, and error handling.

use langweave::error::I18nError;
use langweave::translator::Translator;

/// This is the main function of the LangWeave Translator Example program.
/// It demonstrates the usage of the Translator struct in the LangWeave library,
/// showcasing its initialization, translation capabilities, and error handling.
///
/// # Arguments
///
/// None.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`:
///   - `Ok(())`: If all examples completed successfully.
///   - `Err(Box<dyn std::error::Error>)`: If any example encountered an error.
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Translator Example\n");

    translator_initialization_example()?;
    basic_translation_example()?;
    multiple_languages_example()?;
    case_sensitivity_example()?;
    error_handling_example()?;
    display_implementation_example()?;

    println!("\n🎉 All translator examples completed successfully!");

    Ok(())
}

/// Demonstrates initializing translators for different languages.
fn translator_initialization_example() -> Result<(), I18nError> {
    println!("🦀 Translator Initialization Example");
    println!("---------------------------------------------");

    let supported_languages = ["en", "fr", "de"];
    let unsupported_language = "xx";

    for lang in &supported_languages {
        match Translator::new(lang) {
            Ok(translator) => println!(
                "    ✅ Successfully initialized translator for {}: {}",
                lang, translator
            ),
            Err(e) => println!(
                "    ❌ Failed to initialize translator for {}: {:?}",
                lang, e
            ),
        }
    }

    match Translator::new(unsupported_language) {
        Ok(_) => println!("    ❓ Unexpectedly initialized translator for unsupported language {}", unsupported_language),
        Err(e) => println!("    ✅ Expected error for unsupported language {}: {:?}", unsupported_language, e),
    }

    Ok(())
}

/// Demonstrates basic translation functionality.
fn basic_translation_example() -> Result<(), I18nError> {
    println!("\n🦀 Basic Translation Example");
    println!("---------------------------------------------");

    let translator = Translator::new("en")?;
    let words = ["Hello", "Goodbye", "Yes", "No", "Thank you"];

    for word in &words {
        match translator.translate(word) {
            Ok(translated) => println!(
                "    ✅ '{}' translated to: {}",
                word, translated
            ),
            Err(e) => {
                println!("    ❌ Error translating '{}': {:?}", word, e)
            }
        }
    }

    Ok(())
}

/// Demonstrates translations across multiple languages.
fn multiple_languages_example() -> Result<(), I18nError> {
    println!("\n🦀 Multiple Languages Example");
    println!("---------------------------------------------");

    let languages = ["en", "fr", "de"];
    let words = ["Hello", "Goodbye"];

    for lang in &languages {
        let translator = Translator::new(lang)?;
        println!("    Translations for {}:", lang);
        for word in &words {
            match translator.translate(word) {
                Ok(translated) => {
                    println!("        '{}' -> '{}'", word, translated)
                }
                Err(e) => println!(
                    "        ❌ Error translating '{}': {:?}",
                    word, e
                ),
            }
        }
    }

    Ok(())
}

/// Demonstrates case sensitivity in translations.
fn case_sensitivity_example() -> Result<(), I18nError> {
    println!("\n🦀 Case Sensitivity Example");
    println!("---------------------------------------------");

    let translator = Translator::new("en")?;
    let test_cases = ["hello", "GOODBYE", "ThAnK yOu"];

    for word in &test_cases {
        match translator.translate(word) {
            Ok(translated) => println!(
                "    ✅ '{}' translated to: {}",
                word, translated
            ),
            Err(e) => {
                println!("    ❌ Error translating '{}': {:?}", word, e)
            }
        }
    }

    Ok(())
}

/// Demonstrates error handling in various translation scenarios.
fn error_handling_example() -> Result<(), I18nError> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    let translator = Translator::new("en")?;

    // Attempt to translate a non-existent key
    match translator.translate("NonExistentKey") {
        Ok(_) => {
            println!("    ❓ Unexpected success for non-existent key")
        }
        Err(e) => println!(
            "    ✅ Expected error for non-existent key: {:?}",
            e
        ),
    }

    // Attempt to translate an empty string
    match translator.translate("") {
        Ok(_) => println!("    ❓ Unexpected success for empty string"),
        Err(e) => {
            println!("    ✅ Expected error for empty string: {:?}", e)
        }
    }

    Ok(())
}

/// Demonstrates the Display implementation for Translator.
fn display_implementation_example() -> Result<(), I18nError> {
    println!("\n🦀 Display Implementation Example");
    println!("---------------------------------------------");

    let languages = ["en", "fr", "de"];

    for lang in &languages {
        let translator = Translator::new(lang)?;
        println!("    {}", translator);
    }

    Ok(())
}
