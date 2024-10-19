// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Error Handling Examples
//!
//! This program demonstrates the usage of various error types and functions
//! in the LangWeave library, including creating and handling
//! different types of I18nError.

use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::translate;
use langweave::translator::Translator;

/// This is the main function for running LangWeave error handling examples.
///
/// # Purpose
///
/// The main function demonstrates various error types and functions in the LangWeave library,
/// including creating and handling different types of I18nError.
///
/// # Parameters
///
/// None.
///
/// # Return Value
///
/// Returns a `Result` indicating success or failure.
///
/// - `Ok(())`: Indicates successful completion of all error handling examples.
/// - `Err(Box<dyn std::error::Error>)`: Indicates an error occurred during execution.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Error Handling Examples\n");

    unsupported_language_example()?;
    language_detection_failed_example().await?;
    translation_failed_example()?;
    unexpected_error_example()?;
    successful_translation_example()?;

    println!(
        "\n🎉 All error handling examples completed successfully!"
    );

    Ok(())
}

/// Demonstrates handling unsupported language errors.
fn unsupported_language_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Unsupported Language Example");
    println!("---------------------------------------------");

    match translate("xx", "Hello") {
        Ok(_) => println!("    ✅ Unexpected success"),
        Err(e) => match e {
            I18nError::UnsupportedLanguage(lang) => {
                println!("    ❌ Unsupported Language Error: {}", lang)
            }
            _ => println!("    ❌ Unexpected error type: {:?}", e),
        },
    }

    Ok(())
}

/// Demonstrates handling language detection failures.
async fn language_detection_failed_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Language Detection Failed Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    match detector.detect_async("").await {
        Ok(_) => println!("    ✅ Unexpected success"),
        Err(e) => match e {
            I18nError::LanguageDetectionFailed => {
                println!("    ❌ Language Detection Failed Error")
            }
            _ => println!("    ❌ Unexpected error type: {:?}", e),
        },
    }

    Ok(())
}

/// Demonstrates handling translation failures.
fn translation_failed_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Translation Failed Example");
    println!("---------------------------------------------");

    let translator = Translator::new("en").unwrap();

    match translator.translate("NonexistentKey") {
        Ok(_) => println!("    ✅ Unexpected success"),
        Err(e) => match e {
            I18nError::TranslationFailed(msg) => {
                println!("    ❌ Translation Failed Error: {}", msg)
            }
            _ => println!("    ❌ Unexpected error type: {:?}", e),
        },
    }

    Ok(())
}

/// Demonstrates handling unexpected errors.
fn unexpected_error_example() -> Result<(), Box<dyn std::error::Error>>
{
    println!("\n🦀 Unexpected Error Example");
    println!("---------------------------------------------");

    // Simulating an unexpected error
    let error = I18nError::UnexpectedError(
        "Simulated unexpected error".to_string(),
    );

    // Explicitly specify the Result type
    let result: Result<String, I18nError> = Err(error);

    match result {
        Ok(_) => println!("    ✅ Unexpected success"),
        Err(e) => match e {
            I18nError::UnexpectedError(msg) => {
                println!("    ❌ Unexpected Error: {}", msg)
            }
            _ => println!("    ❌ Unexpected error type: {:?}", e),
        },
    }

    Ok(())
}

/// Demonstrates a successful translation scenario.
fn successful_translation_example(
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Successful Translation Example");
    println!("---------------------------------------------");

    match translate("fr", "Hello") {
        Ok(result) => println!("    ✅ Translated text: {}", result),
        Err(e) => println!("    ❌ Unexpected error: {:?}", e),
    }

    Ok(())
}
