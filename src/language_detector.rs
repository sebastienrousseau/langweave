// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Language Detection Module
//!
//! This module provides mechanisms to detect the language of given text using
//! a combination of custom heuristics and the `whatlang` library. It supports
//! both synchronous and asynchronous detection for a wide range of languages,
//! optimized for short texts and mixed language inputs, including non-Latin scripts.
//!
//! ## Features
//!
//! - Synchronous and asynchronous language detection
//! - Custom pattern matching for common languages
//! - Fallback to statistical detection using `whatlang`
//! - Support for a wide range of languages and scripts
//!
//! ## Examples
//!
//! Synchronous usage:
//!
//! ```
//! use langweave::language_detector::LanguageDetector;
//!
//! let detector = LanguageDetector::new();
//! let result = detector.detect("Hello, world!");
//! assert_eq!(result.unwrap(), "en");
//! ```
//!
//! Asynchronous usage:
//!
//! ```
//! use langweave::language_detector::LanguageDetector;
//!
//! #[tokio::main]
//! async fn main() {
//!     let detector = LanguageDetector::new();
//!     let result = detector.detect_async("Bonjour le monde!").await;
//!     assert_eq!(result.unwrap(), "fr");
//! }
//! ```

use crate::error::I18nError;
use log::{debug, error};
use once_cell::sync::Lazy;
use regex::Regex;
use std::sync::Arc;
use whatlang::{detect, Lang};

/// A thread-safe struct for detecting the language of a given text.
#[derive(Debug, Clone)]
pub struct LanguageDetector {
    patterns: Arc<Vec<(Regex, &'static str)>>,
}

/// A static list of language detection patterns for common languages.
static PATTERNS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        // English
        (
            Regex::new(r"(?i)\b(hello|hi|hey|goodbye|bye|thank you|thanks|please|the|a|an|in|on|at|for|to|of)\b").expect("Failed to compile English regex"),
            "en",
        ),
        // French
        (
            Regex::new(r"(?i)\b(bonjour|salut|au revoir|merci|s'il vous plaît|le|la|les|un|une|des|dans|sur|pour|de)\b").expect("Failed to compile French regex"),
            "fr",
        ),
        // German
        (
            Regex::new(r"(?i)\b(hallo|guten tag|auf wiedersehen|tschüss|danke|bitte|der|die|das|ein|eine|in|auf|für|zu|von)\b").expect("Failed to compile German regex"),
            "de",
        ),
        // Spanish
        (
            Regex::new(r"(?i)\b(hola|adiós|gracias|por favor|el|la|los|las|un|una|unos|unas|en|para|por)\b").expect("Failed to compile Spanish regex"),
            "es",
        ),
        // Portuguese
        (
            Regex::new(r"(?i)\b(olá|adeus|obrigado|obrigada|por favor|o|a|os|as|um|uma|uns|umas|em|para|por)\b").expect("Failed to compile Portuguese regex"),
            "pt",
        ),
        // Russian (includes Cyrillic script detection)
        (
            Regex::new(r"(?i)\b(здравствуйте|привет|до свидания|пока|спасибо|пожалуйста)|[\p{Cyrillic}]+").expect("Failed to compile Russian regex"),
            "ru",
        ),
        // Arabic script detection
        (Regex::new(r"[\p{Arabic}]+").expect("Failed to compile Arabic regex"), "ar"),
        // Japanese (prioritize Hiragana and Katakana)
        (
            Regex::new(r"(?i)\b(こんにちは|さようなら|ありがとう|お願いします)|[\p{Hiragana}\p{Katakana}ー]+").expect("Failed to compile Japanese regex"),
            "ja",
        ),
        // Chinese (Han script detection, but exclude Japanese-specific characters)
        (
            Regex::new(r"(?i)\b(你好|再见|谢谢|请)|(?:[\p{Han}&&[^\p{Hiragana}\p{Katakana}ー]]+)").expect("Failed to compile Chinese regex"),
            "zh",
        ),
        // Hindi (includes Devanagari script detection)
        (
            Regex::new(r"(?i)\b(नमस्ते|अलविदा|धन्यवाद|कृपया)|[\p{Devanagari}]+").expect("Failed to compile Hindi regex"),
            "hi",
        ),
        // Korean (includes Hangul script detection)
        (
            Regex::new(r"(?i)\b(안녕하세요|안녕히 가세요|감사합니다|주세요)|[\p{Hangul}]+").expect("Failed to compile Korean regex"),
            "ko",
        ),
    ]
});

impl LanguageDetector {
    /// Creates a new instance of `LanguageDetector`.
    ///
    /// This constructor initializes the detector with predefined language detection patterns
    /// for multiple languages, allowing for quick detection based on common words and script patterns.
    ///
    /// # Returns
    ///
    /// * `LanguageDetector` - A new instance of the `LanguageDetector` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// let detector = LanguageDetector::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        LanguageDetector {
            patterns: Arc::new(PATTERNS.clone()),
        }
    }

    /// Detects the language of the given text synchronously.
    ///
    /// This method first attempts to detect the language using custom regular expression patterns
    /// for common words. If no match is found, it falls back to `whatlang` for statistical detection.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Result<String, I18nError>` - The detected language code if successful, or an error if detection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// let detector = LanguageDetector::new();
    /// let result = detector.detect("The quick brown fox");
    /// assert_eq!(result.unwrap(), "en");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an `I18nError::LanguageDetectionFailed` if:
    /// - The input text is empty or contains only non-alphabetic characters.
    /// - The language detection process fails to identify a language with sufficient confidence.
    pub fn detect(&self, text: &str) -> Result<String, I18nError> {
        let normalized_text = text.trim();

        // Reject empty or non-alphabetic input
        if normalized_text.is_empty()
            || !normalized_text.chars().any(|c| c.is_alphabetic())
        {
            error!("Empty or non-alphabetic input: {}", text);
            return Err(I18nError::LanguageDetectionFailed);
        }

        // Try custom patterns first
        for (pattern, lang) in self.patterns.iter() {
            if pattern.is_match(normalized_text) {
                debug!("Custom heuristic matched pattern for language '{}'", lang);
                return Ok(lang.to_string());
            }
        }

        // If custom heuristics fail, detect word-by-word using `whatlang`
        for word in normalized_text.split_whitespace() {
            if let Some(info) = detect(word) {
                if info.is_reliable() || info.confidence() > 0.3 {
                    debug!(
                        "Detected language '{}' for word '{}'",
                        info.lang(),
                        word
                    );
                    return Ok(self.convert_lang_code(info.lang()));
                }
            }
        }

        // If no detections succeed, return an error
        error!("Failed to detect language for text: {}", text);
        Err(I18nError::LanguageDetectionFailed)
    }

    /// Detects the language of the given text asynchronously.
    ///
    /// This method provides the same functionality as `detect`, but operates asynchronously,
    /// allowing for non-blocking language detection in concurrent contexts.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Result<String, I18nError>` - The detected language code if successful, or an error if detection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let detector = LanguageDetector::new();
    ///     let result = detector.detect_async("Le chat noir").await;
    ///     assert_eq!(result.unwrap(), "fr");
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an `I18nError::LanguageDetectionFailed` if:
    /// - The input text is empty or contains only non-alphabetic characters.
    /// - The language detection process fails to identify a language with sufficient confidence.
    pub async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError> {
        let text = text.to_string();
        let patterns = Arc::clone(&self.patterns);

        tokio::task::spawn_blocking(move || {
            let detector = LanguageDetector { patterns };
            detector.detect(&text)
        })
        .await
        .map_err(|e| {
            error!("Async language detection task failed: {:?}", e);
            I18nError::LanguageDetectionFailed
        })?
    }

    /// Converts `whatlang`'s language codes to the desired format.
    ///
    /// This function maps `whatlang`'s internal `Lang` enum values to their ISO 639-1
    /// equivalents or other common language codes used by the application.
    ///
    /// # Arguments
    ///
    /// * `lang` - The `Lang` enum from `whatlang`.
    ///
    /// # Returns
    ///
    /// * `String` - The standardized language code (e.g., "en", "fr").
    fn convert_lang_code(&self, lang: Lang) -> String {
        match lang {
            Lang::Eng => "en",
            Lang::Fra => "fr",
            Lang::Deu => "de",
            Lang::Spa => "es",
            Lang::Por => "pt",
            Lang::Jpn => "ja",
            Lang::Cmn => "zh",
            Lang::Ara => "ar",
            Lang::Hin => "hi",
            Lang::Kor => "ko",
            Lang::Rus => "ru",
            _ => lang.code(),
        }
        .to_string()
    }
}

impl Default for LanguageDetector {
    /// Provides a default instance of `LanguageDetector`.
    ///
    /// # Returns
    ///
    /// * `LanguageDetector` - A default instance using predefined patterns for common languages.
    ///
    /// This method allows the `LanguageDetector` to be initialized easily with default patterns.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn test_language_detection() {
        let detector = LanguageDetector::new();
        let test_cases = vec![
            ("The quick brown fox", "en"),
            ("Le chat noir", "fr"),
            ("Der schnelle Fuchs", "de"),
            ("El gato rápido", "es"),
            ("O gato rápido", "pt"),
            ("こんにちは", "ja"),
            ("你好", "zh"),
            ("مرحبا", "ar"),
            ("नमस्ते", "hi"),
            ("안녕하세요", "ko"),
        ];

        for (text, expected_lang) in test_cases {
            match detector.detect(text) {
                Ok(lang) => assert_eq!(
                    lang, expected_lang,
                    "Failed to detect {} for text: {}",
                    expected_lang, text
                ),
                Err(err) => panic!(
                    "Failed to detect language for '{}': {:?}",
                    text, err
                ),
            }
        }
    }

    #[tokio::test]
    async fn test_async_language_detection() {
        let detector = LanguageDetector::new();
        let test_cases = vec![
            ("The quick brown fox", "en"),
            ("Le chat noir", "fr"),
            ("Der schnelle Fuchs", "de"),
            ("El gato rápido", "es"),
            ("O gato rápido", "pt"),
            ("こんにちは", "ja"),
            ("你好", "zh"),
            ("مرحبا", "ar"),
            ("नमस्ते", "hi"),
            ("안녕하세요", "ko"),
        ];

        for (text, expected_lang) in test_cases {
            match detector.detect_async(text).await {
                Ok(lang) => assert_eq!(
                    lang, expected_lang,
                    "Failed to detect {} for text: {}",
                    expected_lang, text
                ),
                Err(err) => panic!(
                    "Failed to detect language asynchronously for '{}': {:?}",
                    text, err
                ),
            }
        }
    }

    #[test]
    fn test_empty_input() {
        let detector = LanguageDetector::new();
        assert!(
            detector.detect("").is_err(),
            "Empty input should return an error."
        );
        assert!(
            detector.detect("   ").is_err(),
            "Whitespace-only input should return an error."
        );
    }

    #[tokio::test]
    async fn test_async_empty_input() {
        let detector = LanguageDetector::new();
        assert!(
            detector.detect_async("").await.is_err(),
            "Empty input should return an error in async detection."
        );
        assert!(
            detector.detect_async("   ").await.is_err(),
            "Whitespace-only input should return an error in async detection."
        );
    }

    #[test]
    fn test_non_linguistic_characters() {
        let detector = LanguageDetector::new();
        assert!(detector.detect("12345 @#$% !").is_err(), "Non-linguistic characters should return a detection error.");
    }

    #[tokio::test]
    async fn test_async_non_linguistic_characters() {
        let detector = LanguageDetector::new();
        assert!(
            detector.detect_async("12345 @#$% !").await.is_err(),
            "Non-linguistic characters should return a detection error in async detection."
        );
    }

    #[test]
    fn test_mixed_language() {
        let detector = LanguageDetector::new();
        let result = detector
            .detect("The quick brown fox Le chat noir")
            .unwrap();
        assert!(
            result == "en" || result == "fr",
            "Should detect either English or French in mixed-language input."
        );
    }

    #[tokio::test]
    async fn test_async_mixed_language() {
        let detector = LanguageDetector::new();
        let result = detector
            .detect_async("The quick brown fox Le chat noir")
            .await
            .unwrap();
        assert!(
            result == "en" || result == "fr",
            "Should detect either English or French in mixed-language input asynchronously."
        );
    }

    #[test]
    fn test_long_input() {
        let detector = LanguageDetector::new();
        let long_text =
            "The quick brown fox jumps over the lazy dog ".repeat(1000);
        assert_eq!(
            detector.detect(&long_text).unwrap(),
            "en",
            "Failed to detect English in long input."
        );
    }

    #[tokio::test]
    async fn test_async_long_input() {
        let detector = LanguageDetector::new();
        let long_text = "Le chat noir est très beau ".repeat(1000);
        assert_eq!(
            detector.detect_async(&long_text).await.unwrap(),
            "fr",
            "Failed to detect French in long input asynchronously."
        );
    }

    #[test]
    fn test_convert_lang_code() {
        let detector = LanguageDetector::new();
        assert_eq!(detector.convert_lang_code(Lang::Eng), "en");
        assert_eq!(detector.convert_lang_code(Lang::Fra), "fr");
        assert_eq!(detector.convert_lang_code(Lang::Deu), "de");
        assert_eq!(detector.convert_lang_code(Lang::Spa), "es");
        assert_eq!(detector.convert_lang_code(Lang::Por), "pt");
        assert_eq!(detector.convert_lang_code(Lang::Jpn), "ja");
        assert_eq!(detector.convert_lang_code(Lang::Cmn), "zh");
        assert_eq!(detector.convert_lang_code(Lang::Ara), "ar");
        assert_eq!(detector.convert_lang_code(Lang::Hin), "hi");
        assert_eq!(detector.convert_lang_code(Lang::Kor), "ko");
        assert_eq!(detector.convert_lang_code(Lang::Rus), "ru");
        // Test a language not explicitly handled
        assert_eq!(detector.convert_lang_code(Lang::Ita), "ita");
    }

    #[test]
    fn test_default_implementation() {
        let detector = LanguageDetector::default();
        assert_eq!(
            detector.detect("Hello").unwrap(),
            "en",
            "Default implementation should work correctly."
        );
    }

    #[tokio::test]
    async fn test_concurrent_detection() {
        let detector = Arc::new(LanguageDetector::new());
        let mut handles = vec![];

        for _ in 0..100 {
            let detector_clone = Arc::clone(&detector);
            handles.push(tokio::spawn(async move {
                detector_clone.detect_async("Hello world").await
            }));
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert_eq!(
                result.unwrap(),
                "en",
                "Concurrent detection should work correctly."
            );
        }
    }
}
