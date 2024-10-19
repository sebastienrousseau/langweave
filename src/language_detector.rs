// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Language Detection Module
//!
//! This module provides a mechanism to detect the language of a given
//! text using the `whatlang` library combined with custom heuristics.
//! It supports detection for a wide range of languages and is optimized
//! for short texts and mixed language inputs, including non-Latin scripts.

use crate::error::I18nError;
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;
use whatlang::{detect, Lang};

/// Struct for detecting the language of a given text.
#[derive(Debug, Clone)]
pub struct LanguageDetector {
    patterns: Vec<(Regex, &'static str)>,
}

/// A static list of language detection patterns for common languages.
static PATTERNS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        // English
        (
            Regex::new(r"(?i)\b(hello|hi|hey|goodbye|bye|thank you|thanks|please|the|a|an|in|on|at|for|to|of)\b").unwrap(),
            "en",
        ),
        // French
        (
            Regex::new(r"(?i)\b(bonjour|salut|au revoir|merci|s'il vous plaît|le|la|les|un|une|des|dans|sur|pour|de)\b").unwrap(),
            "fr",
        ),
        // German
        (
            Regex::new(r"(?i)\b(hallo|guten tag|auf wiedersehen|tschüss|danke|bitte|der|die|das|ein|eine|in|auf|für|zu|von)\b").unwrap(),
            "de",
        ),
        // Spanish
        (
            Regex::new(r"(?i)\b(hola|adiós|gracias|por favor|el|la|los|las|un|una|unos|unas|en|para|por)\b").unwrap(),
            "es",
        ),
        // Portuguese
        (
            Regex::new(r"(?i)\b(olá|adeus|obrigado|obrigada|por favor|o|a|os|as|um|uma|uns|umas|em|para|por)\b").unwrap(),
            "pt",
        ),
        // Russian (includes Cyrillic script detection)
        (
            Regex::new(r"(?i)\b(здравствуйте|привет|до свидания|пока|спасибо|пожалуйста)|[\p{Cyrillic}]+").unwrap(),
            "ru",
        ),
        // Arabic script detection
        (Regex::new(r"[\p{Arabic}]+").unwrap(), "ar"),
        // Japanese (prioritize Hiragana and Katakana)
        (
            Regex::new(r"(?i)\b(こんにちは|さようなら|ありがとう|お願いします)|[\p{Hiragana}\p{Katakana}ー]+").unwrap(),
            "ja",
        ),
        // Chinese (Han script detection, but exclude Japanese-specific characters)
        (
            Regex::new(r"(?i)\b(你好|再见|谢谢|请)|(?:[\p{Han}&&[^\p{Hiragana}\p{Katakana}ー]]+)").unwrap(),
            "zh",
        ),
        // Hindi (includes Devanagari script detection)
        (
            Regex::new(r"(?i)\b(नमस्ते|अलविदा|धन्यवाद|कृपया)|[\p{Devanagari}]+").unwrap(),
            "hi",
        ),
        // Korean (includes Hangul script detection)
        (
            Regex::new(r"(?i)\b(안녕하세요|안녕히 가세요|감사합니다|주세요)|[\p{Hangul}]+").unwrap(),
            "ko",
        ),
    ]
});

impl LanguageDetector {
    /// Creates a new instance of `LanguageDetector`.
    ///
    /// # Returns
    ///
    /// * `LanguageDetector` - A new instance of the `LanguageDetector` struct.
    ///
    /// This struct contains predefined language detection patterns for multiple languages,
    /// allowing for quick detection based on common words and script patterns.
    #[must_use]
    pub fn new() -> Self {
        LanguageDetector {
            patterns: PATTERNS.clone(),
        }
    }

    /// Detects the language of the given text.
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
    /// The function first tries to detect the language using custom regular expression patterns for common words.
    /// If no match is found, it falls back to `whatlang` for statistical detection.
    pub fn detect(&self, text: &str) -> Result<String, I18nError> {
        let normalized_text = text.trim();

        // Reject empty or non-alphabetic input
        if normalized_text.is_empty()
            || !normalized_text.chars().any(|c| c.is_alphabetic())
        {
            return Err(I18nError::LanguageDetectionFailed);
        }

        // Try custom patterns first
        for (pattern, lang) in &self.patterns {
            if pattern.is_match(normalized_text) {
                debug!("Custom heuristic matched pattern '{}' for language '{}'", pattern, lang);
                return Ok(lang.to_string());
            }
        }

        // If custom heuristics fail, detect word-by-word using `whatlang`
        for word in normalized_text.split_whitespace() {
            match detect(word) {
                Some(info) => {
                    if info.is_reliable() || info.confidence() > 0.3 {
                        debug!(
                            "Detected language '{}' for word '{}'",
                            info.lang(),
                            word
                        );
                        return Ok(self.convert_lang_code(info.lang()));
                    }
                }
                None => continue, // Try the next word
            }
        }

        // If no detections succeed, return an error
        Err(I18nError::LanguageDetectionFailed)
    }

    /// Converts `whatlang`'s language codes to the desired format.
    ///
    /// # Arguments
    ///
    /// * `lang` - The `Lang` enum from `whatlang`.
    ///
    /// # Returns
    ///
    /// * `String` - The standardized language code (e.g., "en", "fr").
    ///
    /// This function maps `whatlang`'s internal `Lang` enum values to their ISO 639-1
    /// equivalents or other common language codes used by the application. If the language
    /// isn't explicitly handled, the `Lang::code()` method is used to get the code.
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

    // Instance of the default language detector for testing
    static LANGUAGE_DETECTOR: Lazy<LanguageDetector> =
        Lazy::new(LanguageDetector::new);

    /// Tests direct language detection for mixed input.
    #[test]
    fn test_direct_language_detection() {
        let result = LANGUAGE_DETECTOR.detect("Hello bonjour");
        assert!(result.is_ok(), "Detection failed for 'Hello bonjour'");
        println!("Detection result for 'Hello bonjour': {:?}", result);

        let result_hello = LANGUAGE_DETECTOR.detect("Hello");
        println!("Detection result for 'Hello': {:?}", result_hello);
        assert!(result_hello.is_ok(), "Detection failed for 'Hello'");

        let result_bonjour = LANGUAGE_DETECTOR.detect("bonjour");
        println!(
            "Detection result for 'bonjour': {:?}",
            result_bonjour
        );
        assert!(
            result_bonjour.is_ok(),
            "Detection failed for 'bonjour'"
        );
    }

    /// Tests custom pattern matching for English and French.
    #[test]
    fn test_custom_patterns_direct() {
        let patterns = &LANGUAGE_DETECTOR.patterns;

        let hello_match = patterns
            .iter()
            .any(|(pattern, _)| pattern.is_match("Hello"));
        let bonjour_match = patterns
            .iter()
            .any(|(pattern, _)| pattern.is_match("bonjour"));
        let mixed_match = patterns
            .iter()
            .any(|(pattern, _)| pattern.is_match("Hello bonjour"));

        println!("Does any pattern match 'Hello'? {}", hello_match);
        println!("Does any pattern match 'bonjour'? {}", bonjour_match);
        println!(
            "Does any pattern match 'Hello bonjour'? {}",
            mixed_match
        );

        assert!(
            hello_match || bonjour_match,
            "No pattern matched either 'Hello' or 'bonjour'."
        );
    }

    /// Tests the fallback `whatlang` detection for individual words and mixed input.
    #[test]
    fn test_whatlang_direct() {
        let hello_result = detect("Hello");
        let bonjour_result = detect("bonjour");
        let mixed_result = detect("Hello bonjour");

        println!("Whatlang result for 'Hello': {:?}", hello_result);
        println!("Whatlang result for 'bonjour': {:?}", bonjour_result);
        println!(
            "Whatlang result for 'Hello bonjour': {:?}",
            mixed_result
        );

        assert!(
            hello_result.is_some(),
            "Whatlang failed to detect 'Hello'"
        );
        assert!(
            bonjour_result.is_some(),
            "Whatlang failed to detect 'bonjour'"
        );
    }

    /// Tests general language detection for various languages and scripts.
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

    /// Tests language detection specifically for Russian (Cyrillic script).
    #[test]
    fn test_russian_detection() {
        let detector = LanguageDetector::new();
        assert_eq!(
            detector.detect("Здравствуйте").unwrap(),
            "ru",
            "Should detect Russian."
        );
    }

    /// Tests detection of mixed-language input (English and French).
    #[test]
    fn test_mixed_language() {
        let detector = LanguageDetector::new();
        let result = detector
            .detect("The quick brown fox Le chat noir")
            .unwrap();
        assert!(result == "en" || result == "fr", "Should detect either English or French in mixed-language input.");
    }

    /// Tests that empty input returns an error.
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

    /// Tests language detection for long input.
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

    /// Tests that non-linguistic characters return a detection error.
    #[test]
    fn test_non_linguistic_characters() {
        let detector = LanguageDetector::new();
        assert!(detector.detect("12345 @#$% !").is_err(), "Non-linguistic characters should return a detection error.");
    }

    /// Tests detection failure for gibberish input that doesn't match any language.
    #[test]
    fn test_non_language_gibberish() {
        let detector = LanguageDetector::new();
        assert!(
            detector.detect("asdfghjkl").is_err(),
            "Should not detect non-linguistic gibberish."
        );
    }
}
