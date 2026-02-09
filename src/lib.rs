// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/langweave/images/favicon.ico",
    html_logo_url = "https://kura.pro/langweave/images/logos/langweave.svg",
    html_root_url = "https://docs.rs/langweave"
)]
#![crate_name = "langweave"]
#![crate_type = "lib"]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use language_detector_trait::LanguageDetectorTrait;
use log::debug;
use once_cell::sync::Lazy;

use crate::error::I18nError;
use crate::language_detector::LanguageDetector;
use crate::translator::Translator;

/// The `error` module contains error types used by the library.
pub mod error;
/// The `language_detector` module contains a simple regex-based language detector.
pub mod language_detector;
/// The `language_detector_trait` module contains the `LanguageDetectorTrait` trait for extensibility.
pub mod language_detector_trait;
/// The `translations` module contains translation functions for different languages.
pub mod translations;
/// The `translator` module contains a simple translation service using a predefined dictionary.
pub mod translator;
/// The `optimized` module contains zero-cost abstraction performance optimizations.
pub mod optimized;
/// Performance testing and comparison module.
pub mod performance_test;

/// A module that re-exports commonly used items for convenience.
pub mod prelude {
    pub use crate::detect_language;
    pub use crate::error::I18nError;
    pub use crate::is_language_supported;
    pub use crate::supported_languages;
    pub use crate::translate;
    pub use crate::translator::Translator;
}

/// The current version of the langweave library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A lazy-initialized instance of the LanguageDetector.
static LANGUAGE_DETECTOR: Lazy<LanguageDetector> =
    Lazy::new(LanguageDetector::new);

/// Translates a given text to a specified language.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the target language code (e.g., "en", "fr").
/// * `text` - A string slice that holds the text to be translated.
///
/// # Returns
///
/// * `Ok(String)` - The translated text.
/// * `Err(I18nError)` - An error if the translation fails.
///
/// # Examples
///
/// ```
/// use langweave::translate;
///
/// let result = translate("fr", "Hello");
/// assert_eq!(result.unwrap(), "Bonjour");
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The specified language is not supported.
/// * The translation key is not found in the language's translation dictionary.
/// * The translation process fails for any other reason.
pub fn translate(lang: &str, text: &str) -> Result<String, I18nError> {
    if !is_language_supported(lang) {
        return Err(I18nError::UnsupportedLanguage(lang.to_string()));
    }

    let translator = Translator::new(lang)?;

    // Try to translate, but fallback to original text if translation fails
    // Only fallback for simple keys (single word, no punctuation except basic ones)
    match translator.translate(text) {
        Ok(translation) => Ok(translation),
        Err(_) => {
            if text.contains(' ')
                || text.contains(',')
                || text.contains('?')
                || text.contains('!')
            {
                Err(I18nError::TranslationFailed(format!(
                    "Complex phrase translation not found: {}",
                    text
                )))
            } else {
                Ok(text.to_string())
            }
        }
    }
}

/// Detects the language of a given text using the composite language detector.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to analyze
///
/// # Returns
///
/// * `Result<String, I18nError>` - The detected language code if successful, or an error if detection fails
///
/// # Examples
///
/// ```
/// use langweave::detect_language;
/// use langweave::error::I18nError;
///
/// async fn example() -> Result<(), I18nError> {
///     // Detect language using the high-level function
///     let lang = detect_language("Hello, world!").await?;
///     println!("Detected language: {}", lang);
///
///     // Detect language for French text
///     let lang = detect_language("Bonjour le monde!").await?;
///     println!("Detected language: {}", lang);
///
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The input text is empty or contains only non-alphabetic characters.
/// * The language detection process fails to identify a language with sufficient confidence.
pub async fn detect_language(text: &str) -> Result<String, I18nError> {
    debug!("Detecting language for: {}", text);

    if text.trim().is_empty() {
        return Err(I18nError::LanguageDetectionFailed);
    }

    // Delegate to the composite language detector, which handles
    // regex pattern matching and word-by-word whatlang detection internally
    let detected_lang =
        LANGUAGE_DETECTOR.detect_async(text).await?;
    debug!("Detected language: {}", detected_lang);
    Ok(detected_lang)
}

/// Returns a list of supported language codes.
///
/// This function returns all 15 languages that LangWeave supports:
/// en (English), fr (French), de (German), es (Spanish), pt (Portuguese),
/// it (Italian), nl (Dutch), ru (Russian), ar (Arabic), he (Hebrew),
/// hi (Hindi), ja (Japanese), ko (Korean), zh (Chinese), id (Indonesian).
///
/// # Returns
///
/// A vector of strings representing the supported language codes.
///
/// # Examples
///
/// ```
/// use langweave::supported_languages;
///
/// let languages = supported_languages();
/// assert_eq!(languages.len(), 15);
/// assert!(languages.contains(&"en".to_string()));
/// assert!(languages.contains(&"fr".to_string()));
/// assert!(languages.contains(&"de".to_string()));
/// assert!(languages.contains(&"es".to_string()));
/// ```
pub fn supported_languages() -> Vec<String> {
    vec![
        "en".to_string(),
        "fr".to_string(),
        "de".to_string(),
        "es".to_string(),
        "pt".to_string(),
        "it".to_string(),
        "nl".to_string(),
        "ru".to_string(),
        "ar".to_string(),
        "he".to_string(),
        "hi".to_string(),
        "ja".to_string(),
        "ko".to_string(),
        "zh".to_string(),
        "id".to_string(),
    ]
}

/// Validates if a given language code is supported.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code to validate.
///
/// # Returns
///
/// `true` if the language is supported, `false` otherwise.
///
/// # Examples
///
/// ```
/// use langweave::is_language_supported;
///
/// assert!(is_language_supported("en"));
/// assert!(!is_language_supported("zz"));
/// ```
pub fn is_language_supported(lang: &str) -> bool {
    supported_languages().contains(&lang.to_lowercase())
}

/// Asynchronous utilities for language processing.
#[cfg(feature = "async")]
pub mod async_utils {
    use super::*;

    /// Asynchronously translates a given text to a specified language.
    ///
    /// # Arguments
    ///
    /// * `lang` - A string slice that holds the target language code (e.g., "en", "fr").
    /// * `text` - A string slice that holds the text to be translated.
    ///
    /// # Returns
    ///
    /// A Future that resolves to:
    /// * `Ok(String)` - The translated text.
    /// * `Err(I18nError)` - An error if the translation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::async_utils::translate_async;
    /// use langweave::error::I18nError;
    ///
    /// async fn example() -> Result<(), I18nError> {
    ///     let result = translate_async("fr", "Hello").await?;
    ///     assert_eq!(result, "Bonjour");
    ///     Ok(())
    /// }
    ///
    /// // Note: In a real application, you would run this async function
    /// // using an async runtime like tokio or async-std.
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The specified language is not supported.
    /// * The translation process fails for any reason.
    pub async fn translate_async(
        lang: &str,
        text: &str,
    ) -> Result<String, I18nError> {
        if !is_language_supported(lang) {
            return Err(I18nError::UnsupportedLanguage(
                lang.to_string(),
            ));
        }
        let translator = Translator::new(lang)?;
        translator.translate(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate() {
        assert_eq!(translate("fr", "Hello").unwrap(), "Bonjour");
        assert_eq!(
            translate("de", "Goodbye").unwrap(),
            "Auf Wiedersehen"
        );
    }

    #[test]
    fn test_translate_error() {
        assert!(matches!(
            translate("invalid_lang", "Hello"),
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[tokio::test]
    async fn test_detect_language() {
        assert_eq!(
            detect_language("The quick brown fox").await.unwrap(),
            "en"
        );
        assert_eq!(
            detect_language("Le chat noir").await.unwrap(),
            "fr"
        );
        assert_eq!(
            detect_language("Der schnelle Fuchs").await.unwrap(),
            "de"
        );
    }

    #[tokio::test]
    async fn test_detect_language_error() {
        assert!(matches!(
            detect_language("").await,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[test]
    fn test_translate_complex() {
        let text = "Hello, how are you today?";
        let result = translate("fr", text);
        // Complex phrases not in the dictionary should return TranslationFailed error
        assert!(matches!(
            result,
            Err(I18nError::TranslationFailed(_))
        ));
    }

    #[tokio::test]
    async fn test_detect_language_mixed() {
        let result = detect_language("Hello bonjour").await;
        assert!(
            result.is_ok(),
            "Language detection failed for mixed input"
        );

        let detected_lang = result.unwrap();
        // Expect either "en" or "fr"
        assert!(
            detected_lang == "en" || detected_lang == "fr",
            "Detected language '{}' is neither 'en' nor 'fr'",
            detected_lang
        );
    }

    #[tokio::test]
    async fn test_detect_language_fallback() {
        // Test with a string that might be hard to detect
        let result = detect_language("1234567890").await;
        // It should either detect a language or return LanguageDetectionFailed
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    #[test]
    fn test_supported_languages() {
        let languages = supported_languages();
        assert!(languages.contains(&"en".to_string()));
        assert!(languages.contains(&"fr".to_string()));
        assert!(languages.contains(&"de".to_string()));
    }

    #[test]
    fn test_is_language_supported() {
        assert!(is_language_supported("en"));
        assert!(is_language_supported("fr"));
        assert!(is_language_supported("de"));
        assert!(!is_language_supported("zz"));
    }
}
