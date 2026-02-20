// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Language Detector Trait Examples
//!
//! This program demonstrates the usage of the LanguageDetectorTrait
//! in the LangWeave library, including creating a custom detector
//! and using it alongside the built-in detector.

use async_trait::async_trait;
use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::{
    CompositeLanguageDetector, LanguageDetectorTrait,
};

/// The main function for running the LangWeave Language Detector Trait examples.
///
/// This function demonstrates the usage of the LanguageDetectorTrait in the LangWeave library,
/// including creating a custom detector and using it alongside the built-in detector.
///
/// # Arguments
///
/// None.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>`: A result indicating whether the execution was successful.
///   If successful, returns `Ok(())`. If an error occurs, returns `Err(Box<dyn std::error::Error>)`.
#[tokio::main]
pub(crate) async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧪 LangWeave Language Detector Trait Examples\n");

    built_in_detector_example().await?;
    custom_detector_example().await?;
    composite_detector_example().await?;

    println!(
        "\n🎉 All language detector examples completed successfully!"
    );

    Ok(())
}

/// Demonstrates using the built-in LanguageDetector.
async fn built_in_detector_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Built-in Language Detector Example");
    println!("---------------------------------------------");

    let detector = LanguageDetector::new();

    let texts = vec![
        "Hello, world!",
        "Bonjour le monde!",
        "Hallo Welt!",
        "こんにちは世界！",
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

/// A custom language detector that only detects English and Spanish.
struct SimpleDetector;

#[async_trait]
impl LanguageDetectorTrait for SimpleDetector {
    fn detect(&self, text: &str) -> Result<String, I18nError> {
        if text.to_lowercase().contains("hello") {
            Ok("en".to_string())
        } else if text.to_lowercase().contains("hola") {
            Ok("es".to_string())
        } else {
            Err(I18nError::LanguageDetectionFailed)
        }
    }

    async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError> {
        self.detect(text)
    }
}

/// Demonstrates using a custom language detector.
async fn custom_detector_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Custom Language Detector Example");
    println!("---------------------------------------------");

    let detector = SimpleDetector;

    let texts = vec![
        "Hello, how are you?",
        "Hola, ¿cómo estás?",
        "Bonjour, comment allez-vous?",
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

/// Demonstrates using a composite language detector.
async fn composite_detector_example()
-> Result<(), Box<dyn std::error::Error>> {
    println!("\n🦀 Composite Language Detector Example");
    println!("---------------------------------------------");

    let mut composite = CompositeLanguageDetector::new();
    composite.add_detector(Box::new(SimpleDetector));
    composite.add_detector(Box::new(LanguageDetector::new()));

    let texts = vec![
        "Hello, world!",
        "Hola, mundo!",
        "Bonjour le monde!",
        "こんにちは世界！",
    ];

    for text in texts {
        match composite.detect_async(text).await {
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
