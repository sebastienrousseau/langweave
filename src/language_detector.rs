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
//! - Implements the `LanguageDetectorTrait` for extensibility
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```
//! use langweave::language_detector::LanguageDetector;
//! use langweave::language_detector_trait::LanguageDetectorTrait;
//!
//! let detector = LanguageDetector::new();
//!
//! // Detect English
//! let result = detector.detect("Hello, world!");
//! assert_eq!(result.unwrap(), "en");
//!
//! // Detect French
//! let result = detector.detect("Bonjour le monde!");
//! assert_eq!(result.unwrap(), "fr");
//!
//! // Detect German
//! let result = detector.detect("Hallo Welt!");
//! assert_eq!(result.unwrap(), "de");
//! ```
//!
//! ### Asynchronous Detection
//!
//! ```
//! use langweave::language_detector::LanguageDetector;
//! use langweave::language_detector_trait::LanguageDetectorTrait;
//!
//! #[tokio::main]
//! async fn main() {
//!     let detector = LanguageDetector::new();
//!
//!     // Detect Spanish asynchronously
//!     let result = detector.detect_async("¡Hola mundo!").await;
//!     assert_eq!(result.unwrap(), "es");
//! }
//! ```

use crate::error::I18nError;
use crate::language_detector_trait::LanguageDetectorTrait;
use async_trait::async_trait;
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

/// Safely compiles all language detection patterns without panicking.
///
/// This function attempts to compile all regex patterns for language detection.
/// If any pattern fails to compile, it returns an error instead of panicking.
///
/// # Returns
///
/// * `Result<Vec<(Regex, &'static str)>, I18nError>` - Compiled patterns on success, or an error on failure.
///
/// # Errors
///
/// Returns `I18nError::UnexpectedError` if any pattern compilation fails.
fn compile_language_patterns(
) -> Result<Vec<(Regex, &'static str)>, I18nError> {
    let pattern_specs = vec![
        // English
        (
            r"(?i)\b(hello|hi|hey|goodbye|bye|thank you|thanks|please|the|a|an|in|on|at|for|to|of)\b",
            "en",
        ),
        // French
        (
            r"(?i)\b(bonjour|salut|au revoir|merci|s'il vous plaît|le|la|les|un|une|des|dans|sur|pour|de)\b",
            "fr",
        ),
        // German
        (
            r"(?i)\b(hallo|guten tag|auf wiedersehen|tschüss|danke|bitte|der|die|das|ein|eine|in|auf|für|zu|von)\b",
            "de",
        ),
        // Spanish
        (
            r"(?i)\b(hola|adiós|gracias|por favor|el|la|los|las|un|una|unos|unas|en|para|por)\b",
            "es",
        ),
        // Portuguese
        (
            r"(?i)\b(olá|adeus|obrigado|obrigada|por favor|o|a|os|as|um|uma|uns|umas|em|para|por)\b",
            "pt",
        ),
        // Russian (includes Cyrillic script detection)
        (
            r"(?i)\b(здравствуйте|привет|до свидания|пока|спасибо|пожалуйста)|[\p{Cyrillic}]+",
            "ru",
        ),
        // Arabic script detection
        (r"[\p{Arabic}]+", "ar"),
        // Japanese (prioritize Hiragana and Katakana)
        (
            r"(?i)\b(こんにちは|さようなら|ありがとう|お願いします)|[\p{Hiragana}\p{Katakana}ー]+",
            "ja",
        ),
        // Chinese (Han script detection, but exclude Japanese-specific characters)
        (
            r"(?i)\b(你好|再见|谢谢|请)|(?:[\p{Han}&&[^\p{Hiragana}\p{Katakana}ー]]+)",
            "zh",
        ),
        // Hindi (includes Devanagari script detection)
        (r"(?i)\b(नमस्ते|अलविदा|धन्यवाद|कृपया)|[\p{Devanagari}]+", "hi"),
        // Korean (includes Hangul script detection)
        (
            r"(?i)\b(안녕하세요|안녕히 가세요|감사합니다|주세요)|[\p{Hangul}]+",
            "ko",
        ),
        // Italian
        (
            r"(?i)\b(ciao|buongiorno|grazie|prego|arrivederci|il|la|lo|gli|le|un|una|in|di|per|con)\b",
            "it",
        ),
        // Dutch
        (
            r"(?i)\b(hallo|goedemorgen|dank|alstublieft|tot ziens|de|het|een|van|in|op|voor|met)\b",
            "nl",
        ),
        // Hebrew (includes Hebrew script detection)
        (r"(?i)\b(שלום|להתראות|תודה|בבקשה)|[\p{Hebrew}]+", "he"),
        // Indonesian
        (
            r"(?i)\b(halo|selamat|terima kasih|tolong|sampai jumpa|yang|dan|atau|ini|itu|dengan|untuk)\b",
            "id",
        ),
    ];

    compile_language_patterns_from_specs(&pattern_specs)
}

fn compile_language_patterns_from_specs(
    pattern_specs: &[(&str, &'static str)],
) -> Result<Vec<(Regex, &'static str)>, I18nError> {
    let mut compiled_patterns = Vec::with_capacity(pattern_specs.len());
    for &(pattern_str, lang_code) in pattern_specs {
        match Regex::new(pattern_str) {
            Ok(regex) => compiled_patterns.push((regex, lang_code)),
            Err(err) => {
                error!(
                    "Failed to compile regex for language '{}': {}",
                    lang_code, err
                );
                return Err(I18nError::UnexpectedError(format!(
                    "Failed to compile regex for language '{}': {}",
                    lang_code, err
                )));
            }
        }
    }

    Ok(compiled_patterns)
}

fn patterns_or_empty(
    result: Result<Vec<(Regex, &'static str)>, I18nError>,
) -> Vec<(Regex, &'static str)> {
    match result {
        Ok(patterns) => patterns,
        Err(err) => {
            error!("Failed to compile language detection patterns, falling back to empty patterns: {}", err);
            Vec::new()
        }
    }
}

/// A static list of language detection patterns for common languages.
///
/// This uses lazy initialization with graceful error handling.
/// Returns empty patterns if compilation fails, allowing the system to fall back to whatlang.
/// For explicit error handling, use `LanguageDetector::try_new()` instead.
static PATTERNS: Lazy<Vec<(Regex, &'static str)>> =
    Lazy::new(|| patterns_or_empty(compile_language_patterns()));

impl LanguageDetector {
    /// Creates a new instance of `LanguageDetector`.
    ///
    /// This constructor initializes the detector with predefined language detection patterns
    /// for multiple languages, allowing for quick detection based on common words and script patterns.
    ///
    /// **Note**: This method will panic if any regex pattern fails to compile. For non-panicking
    /// initialization, use [`LanguageDetector::try_new()`] instead.
    ///
    /// # Returns
    ///
    /// * `LanguageDetector` - A new instance of the `LanguageDetector` struct.
    ///
    /// # Panics
    ///
    /// Panics if any language detection regex pattern fails to compile during static initialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// let detector = LanguageDetector::new();
    /// let result = detector.detect("Hello, world!");
    /// assert_eq!(result.unwrap(), "en");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        LanguageDetector {
            patterns: Arc::new(PATTERNS.clone()),
        }
    }

    /// Creates a new instance of `LanguageDetector` without panicking on regex compilation failure.
    ///
    /// This constructor safely compiles all language detection patterns and returns an error
    /// if any pattern fails to compile, instead of panicking at startup.
    ///
    /// # Returns
    ///
    /// * `Result<LanguageDetector, I18nError>` - A new instance on success, or an error on failure.
    ///
    /// # Errors
    ///
    /// Returns `I18nError::UnexpectedError` if any language detection pattern fails to compile.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// let detector = LanguageDetector::try_new()?;
    /// let result = detector.detect("Hello, world!");
    /// assert_eq!(result.unwrap(), "en");
    /// # Ok::<(), langweave::error::I18nError>(())
    /// ```
    pub fn try_new() -> Result<Self, I18nError> {
        let patterns = compile_language_patterns()?;
        Ok(LanguageDetector {
            patterns: Arc::new(patterns),
        })
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
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use whatlang::Lang;
    ///
    /// let detector = LanguageDetector::new();
    /// assert_eq!(detector.convert_lang_code(Lang::Eng), "en");
    /// assert_eq!(detector.convert_lang_code(Lang::Fra), "fr");
    /// assert_eq!(detector.convert_lang_code(Lang::Deu), "de");
    /// ```
    pub fn convert_lang_code(&self, lang: Lang) -> String {
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
            Lang::Ita => "it",
            Lang::Nld => "nl",
            Lang::Heb => "he",
            Lang::Ind => "id",
            _ => lang.code(),
        }
        .to_string()
    }
}

#[async_trait]
impl LanguageDetectorTrait for LanguageDetector {
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
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// let detector = LanguageDetector::new();
    ///
    /// // Detect English
    /// let result = detector.detect("The quick brown fox");
    /// assert_eq!(result.unwrap(), "en");
    ///
    /// // Detect French
    /// let result = detector.detect("Le chat noir");
    /// assert_eq!(result.unwrap(), "fr");
    ///
    /// // Detect German
    /// let result = detector.detect("Der schnelle braune Fuchs");
    /// assert_eq!(result.unwrap(), "de");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an `I18nError::LanguageDetectionFailed` if:
    /// - The input text is empty or contains only non-alphabetic characters.
    /// - The language detection process fails to identify a language with sufficient confidence.
    fn detect(&self, text: &str) -> Result<String, I18nError> {
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
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let detector = LanguageDetector::new();
    ///
    ///     // Detect Spanish asynchronously
    ///     let result = detector.detect_async("El gato negro").await;
    ///     assert_eq!(result.unwrap(), "es");
    ///
    ///     // Detect Portuguese asynchronously
    ///     let result = detector.detect_async("O gato preto").await;
    ///     assert_eq!(result.unwrap(), "pt");
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an `I18nError::LanguageDetectionFailed` if:
    /// - The input text is empty or contains only non-alphabetic characters.
    /// - The language detection process fails to identify a language with sufficient confidence.
    async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError> {
        self.detect(text)
    }
}

impl Default for LanguageDetector {
    /// Provides a default instance of `LanguageDetector`.
    ///
    /// **Note**: This method will panic if any regex pattern fails to compile. This behavior
    /// maintains backward compatibility with the existing API. For non-panicking initialization,
    /// use [`LanguageDetector::try_new()`] instead.
    ///
    /// # Returns
    ///
    /// * `LanguageDetector` - A default instance using predefined patterns for common languages.
    ///
    /// # Panics
    ///
    /// Panics if any language detection regex pattern fails to compile during static initialization.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// let detector = LanguageDetector::default();
    /// let result = detector.detect("Hello, world!");
    /// assert_eq!(result.unwrap(), "en");
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

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
            ("Здравствуйте", "ru"),
            ("Ciao, buongiorno", "it"),
            ("Dank je wel, alstublieft", "nl"),
            ("שלום, תודה", "he"),
            ("Halo, selamat pagi", "id"),
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
            ("Здравствуйте", "ru"),
            ("Ciao, buongiorno", "it"),
            ("Dank je wel, alstublieft", "nl"),
            ("שלום, תודה", "he"),
            ("Halo, selamat pagi", "id"),
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
        // Test the newly added languages
        assert_eq!(detector.convert_lang_code(Lang::Ita), "it");
        assert_eq!(detector.convert_lang_code(Lang::Nld), "nl");
        assert_eq!(detector.convert_lang_code(Lang::Heb), "he");
        assert_eq!(detector.convert_lang_code(Lang::Ind), "id");
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

    #[test]
    fn test_try_new_success() {
        let detector = LanguageDetector::try_new();
        assert!(
            detector.is_ok(),
            "try_new should succeed with valid regex patterns"
        );

        let detector = detector.unwrap();
        assert_eq!(
            detector.detect("Hello").unwrap(),
            "en",
            "Detector created with try_new should work correctly."
        );
    }

    #[test]
    fn test_compile_language_patterns() {
        let patterns = compile_language_patterns();
        assert!(
            patterns.is_ok(),
            "compile_language_patterns should succeed with valid patterns"
        );

        let patterns = patterns.unwrap();
        assert_eq!(
            patterns.len(),
            15,
            "Should compile all 15 language patterns"
        );

        // Verify all expected language codes are present
        let lang_codes: std::collections::HashSet<&str> =
            patterns.iter().map(|(_, code)| *code).collect();
        let expected_codes = vec![
            "en", "fr", "de", "es", "pt", "ru", "ar", "ja", "zh", "hi",
            "ko", "it", "nl", "he", "id",
        ];
        for code in expected_codes {
            assert!(
                lang_codes.contains(code),
                "Missing language code: {}",
                code
            );
        }
    }

    #[test]
    fn test_try_new_vs_new_equivalence() {
        let detector_new = LanguageDetector::new();
        let detector_try = LanguageDetector::try_new().unwrap();

        let test_texts = vec![
            "Hello world",
            "Bonjour le monde",
            "Hola mundo",
            "こんにちは",
            "你好",
        ];

        for text in test_texts {
            let result_new = detector_new.detect(text);
            let result_try = detector_try.detect(text);
            assert_eq!(
                result_new, result_try,
                "Results should be equivalent for text: {}",
                text
            );
        }
    }

    #[test]
    fn test_compile_language_patterns_invalid_regex() {
        let invalid_specs = vec![("(", "xx")];
        let result =
            compile_language_patterns_from_specs(&invalid_specs);

        match result {
            Err(I18nError::UnexpectedError(msg)) => {
                assert!(msg.contains("xx"));
                assert!(msg.contains("Failed to compile regex"));
            }
            _ => panic!("Expected UnexpectedError for invalid regex"),
        }
    }

    #[test]
    fn test_patterns_or_empty_fallback_on_error() {
        let result = Err(I18nError::UnexpectedError(
            "forced compilation error".to_string(),
        ));

        let patterns = patterns_or_empty(result);
        assert!(patterns.is_empty());
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
