// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Language Detector Examples
//!
//! This program demonstrates the usage of the LanguageDetector
//! in the LangWeave library, showcasing its capabilities for
//! detecting languages in various scenarios.

use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;

/// Entry point for the LangWeave Language Detector Examples program.
///
/// This program demonstrates the usage of the LanguageDetector
/// in the LangWeave library, showcasing its capabilities for
/// detecting languages in various scenarios.
///
/// # Errors
///
/// This function returns a `Result` containing a `Box<dyn std::error::Error>` if any error occurs during
/// the execution of the examples.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Language Detector Examples\n");

    simple_detection_example().await?;
    multi_language_detection_example().await?;
    short_text_detection_example().await?;
    non_latin_script_detection_example().await?;
    mixed_language_detection_example().await?;
    error_handling_example().await?;

    println!(
        "\n🎉 All language detector examples completed successfully!"
    );

    Ok(())
}

/// Demonstrates simple language detection for common languages.
async fn simple_detection_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Simple Language Detection Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Hello, how are you?",
        "Bonjour, comment allez-vous ?",
        "Hallo, wie geht es dir?",
        "Hola, ¿cómo estás?",
    ];

    for text in texts {
        match detector.detect_async(text).await {
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

    Ok(())
}

/// Demonstrates language detection for multiple languages.
async fn multi_language_detection_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Multi-Language Detection Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Hello world",      // English
        "Bonjour le monde", // French
        "Hallo Welt",       // German
        "Ciao mondo",       // Italian
        "Olá mundo",        // Portuguese
        "Привет, мир",      // Russian
        "你好，世界",       // Chinese
        "こんにちは、世界", // Japanese
        "안녕하세요, 세상", // Korean
        "مرحبا بالعالم",    // Arabic
    ];

    for text in texts {
        match detector.detect_async(text).await {
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

    Ok(())
}

/// Demonstrates language detection for short texts.
async fn short_text_detection_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Short Text Detection Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Hi",
        "Oi",
        "Hej",
        "Salut",
        "Ciao",
        "こんにちは",
        "안녕",
        "你好",
    ];

    for text in texts {
        match detector.detect_async(text).await {
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

    Ok(())
}

/// Demonstrates language detection for non-Latin scripts.
async fn non_latin_script_detection_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Non-Latin Script Detection Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Здравствуй, мир",  // Russian
        "こんにちは、世界", // Japanese
        "안녕하세요, 세상", // Korean
        "你好，世界",       // Chinese
        "مرحبا بالعالم",    // Arabic
        "γειά σου κόσμε",   // Greek
        "שלום עולם",        // Hebrew
        "नमस्ते दुनिया",       // Hindi
    ];

    for text in texts {
        match detector.detect_async(text).await {
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

    Ok(())
}

/// Demonstrates language detection for mixed language texts.
async fn mixed_language_detection_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Mixed Language Detection Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Hello monde",        // English + French
        "Hola world",         // Spanish + English
        "Bonjour 世界",       // French + Chinese
        "こんにちは world",   // Japanese + English
        "Здравствуй world",   // Russian + English
        "Hello नमस्ते Bonjour", // English + Hindi + French
    ];

    for text in texts {
        match detector.detect_async(text).await {
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

    Ok(())
}

/// Demonstrates error handling in language detection.
async fn error_handling_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Error Handling Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "",           // Empty string
        "123456789",  // Only numbers
        "!@#$%^&*()", // Only symbols
        "   ",        // Only whitespace
    ];

    for text in texts {
        match detector.detect_async(text).await {
            Ok(lang) => println!(
                "    ✅ Unexpectedly detected language for '{}': {}",
                text, lang
            ),
            Err(e) => println!(
                "    ❌ Expected error for '{}': {:?}",
                text, e
            ),
        }
    }

    Ok(())
}
