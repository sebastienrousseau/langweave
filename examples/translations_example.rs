// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Translations Example
//!
//! This program demonstrates the usage of the translations module
//! in the LangWeave library, showcasing translation capabilities
//! and handling various translation scenarios.

use langweave::error::I18nError;
use langweave::translations;

/// Executes the LangWeave translations example program.
///
/// This program demonstrates the usage of the translations module
/// in the LangWeave library, showcasing translation capabilities
/// and handling various translation scenarios.
///
/// # Errors
///
/// Returns a `Box<dyn std::error::Error>` if any error occurs during the execution of the program.
pub(crate) fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Translations Example\n");

    basic_translation_example()?;
    multiple_languages_example()?;
    missing_translation_example()?;
    case_sensitivity_example()?;
    logger_messages_example()?;
    error_handling_example()?;

    println!("\n🎉 All translation examples completed successfully!");

    Ok(())
}

/// Demonstrates basic translation functionality.
fn basic_translation_example() -> Result<(), I18nError> {
    println!("🦀 Basic Translation Example");
    println!("---------------------------------------------");

    let languages = ["en", "fr", "de"];
    let keys = ["Hello", "Goodbye", "Yes", "No", "Thank you"];

    for lang in &languages {
        println!("    Translations for {}:", lang);
        for key in &keys {
            match translations::translate(lang, key) {
                Ok(translated) => {
                    println!("        {} -> {}", key, translated)
                }
                Err(e) => println!(
                    "        ❌ Error translating '{}': {:?}",
                    key, e
                ),
            }
        }
        println!();
    }

    Ok(())
}

/// Demonstrates translations across multiple languages.
fn multiple_languages_example() -> Result<(), I18nError> {
    println!("\n🦀 Multiple Languages Example");
    println!("---------------------------------------------");

    let translations = [
        ("en", "Hello", "Hello"),
        ("fr", "Hello", "Bonjour"),
        ("de", "Hello", "Hallo"),
        ("en", "Goodbye", "Goodbye"),
        ("fr", "Goodbye", "Au revoir"),
        ("de", "Goodbye", "Auf Wiedersehen"),
    ];

    for (lang, key, expected) in &translations {
        match translations::translate(lang, key) {
            Ok(translated) => {
                assert_eq!(&translated, expected);
                println!(
                    "    ✅ '{}' in {} correctly translated to '{}'",
                    key, lang, translated
                );
            }
            Err(e) => println!(
                "    ❌ Error translating '{}' to {}: {:?}",
                key, lang, e
            ),
        }
    }

    Ok(())
}

/// Demonstrates behavior with missing translations.
fn missing_translation_example() -> Result<(), I18nError> {
    println!("\n🦀 Missing Translation Example");
    println!("---------------------------------------------");

    let non_existent_keys = [
        "NonExistentKey1",
        "AnotherMissingKey",
        "YetAnotherMissingKey",
    ];

    for key in &non_existent_keys {
        match translations::translate("en", key) {
            Ok(_) => println!(
                "    ❓ Unexpected success for non-existent key '{}'",
                key
            ),
            Err(e) => {
                println!("    ✅ Expected error for '{}': {:?}", key, e)
            }
        }
    }

    Ok(())
}

/// Demonstrates case sensitivity in translations.
fn case_sensitivity_example() -> Result<(), I18nError> {
    println!("\n🦀 Case Sensitivity Example");
    println!("---------------------------------------------");

    let test_cases = [
        ("en", "hello", "Hello"),
        ("fr", "BONJOUR", "Bonjour"),
        ("de", "AuF wIeDeRsEhEn", "Auf Wiedersehen"),
    ];

    for (lang, key, expected) in &test_cases {
        match translations::translate(lang, key) {
            Ok(translated) => {
                assert_eq!(&translated, expected);
                println!("    ✅ '{}' in {} correctly translated to '{}' (case-insensitive)", key, lang, translated);
            }
            Err(e) => println!(
                "    ❌ Error translating '{}' to {}: {:?}",
                key, lang, e
            ),
        }
    }

    Ok(())
}

/// Demonstrates translation of logger messages.
fn logger_messages_example() -> Result<(), I18nError> {
    println!("\n🦀 Logger Messages Example");
    println!("---------------------------------------------");

    let logger_keys = [
        ("en", "main_logger_msg"),
        ("fr", "lib_banner_log_msg"),
        ("de", "lib_server_log_msg"),
    ];

    for (lang, key) in &logger_keys {
        match translations::translate(lang, key) {
            Ok(translated) => println!("    ✅ Logger message '{}' in {}: {}", key, lang, translated),
            Err(e) => println!("    ❌ Error translating logger message '{}' to {}: {:?}", key, lang, e),
        }
    }

    Ok(())
}

/// Demonstrates error handling in various translation scenarios.
fn error_handling_example() -> Result<(), I18nError> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    // Unsupported language
    match translations::translate("xx", "Hello") {
        Ok(_) => println!(
            "    ❓ Unexpected success for unsupported language"
        ),
        Err(e) => println!(
            "    ✅ Expected error for unsupported language: {:?}",
            e
        ),
    }

    // Empty key
    match translations::translate("en", "") {
        Ok(_) => println!("    ❓ Unexpected success for empty key"),
        Err(e) => {
            println!("    ✅ Expected error for empty key: {:?}", e)
        }
    }

    // Non-existent key
    match translations::translate("en", "NonExistentKey") {
        Ok(_) => {
            println!("    ❓ Unexpected success for non-existent key")
        }
        Err(e) => println!(
            "    ✅ Expected error for non-existent key: {:?}",
            e
        ),
    }

    Ok(())
}
