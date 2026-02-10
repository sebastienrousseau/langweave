//! Comprehensive test suite to achieve 100% code coverage for langweave library
//!
//! This test suite covers edge cases, error conditions, and code paths not covered
//! by existing tests to ensure 100% test coverage.

use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::{
    CompositeLanguageDetector, LanguageDetectorTrait,
};
use langweave::translator::Translator;
use langweave::{
    detect_language_async, is_language_supported, supported_languages,
    translate,
};

#[cfg(feature = "async")]
use langweave::async_utils::translate_async;

/// Test async utilities module when async feature is enabled
#[cfg(feature = "async")]
mod async_utils_tests {
    use super::*;

    #[tokio::test]
    async fn test_translate_async_success() {
        let result = translate_async("fr", "Hello").await;
        assert_eq!(result.unwrap(), "Bonjour");
    }

    #[tokio::test]
    async fn test_translate_async_unsupported_language() {
        let result = translate_async("invalid_lang", "Hello").await;
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[tokio::test]
    async fn test_translate_async_case_insensitive() {
        let result = translate_async("FR", "Hello").await;
        assert_eq!(result.unwrap(), "Bonjour");
    }

    #[tokio::test]
    async fn test_translate_async_german() {
        let result = translate_async("de", "Goodbye").await;
        assert_eq!(result.unwrap(), "Auf Wiedersehen");
    }

    #[tokio::test]
    async fn test_translate_async_english() {
        let result = translate_async("en", "Hello").await;
        assert_eq!(result.unwrap(), "Hello");
    }
}

/// Test error handling in translation fallback
#[test]
fn test_translate_fallback_to_original() {
    // Test case where translation fails but fallback to original text works
    let result = translate("en", "NonExistentKey");
    // Should return original text when translation fails
    assert_eq!(result.unwrap(), "NonExistentKey");
}

/// Test VERSION constant
#[test]
fn test_version_constant() {
    assert!(!langweave::VERSION.is_empty());
    assert!(langweave::VERSION.chars().any(|c| c.is_ascii_digit()));
}

/// Test language detector trait edge cases
mod language_detector_trait_tests {
    use super::*;
    use async_trait::async_trait;

    /// A failing mock detector to test error cases
    struct FailingDetector;

    #[async_trait]
    impl LanguageDetectorTrait for FailingDetector {
        fn detect(&self, _text: &str) -> Result<String, I18nError> {
            Err(I18nError::LanguageDetectionFailed)
        }

        async fn detect_async(
            &self,
            _text: &str,
        ) -> Result<String, I18nError> {
            Err(I18nError::LanguageDetectionFailed)
        }
    }

    #[test]
    fn test_composite_detector_no_detectors() {
        let composite = CompositeLanguageDetector::new();
        let result = composite.detect("Hello world");
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[tokio::test]
    async fn test_composite_detector_no_detectors_async() {
        let composite = CompositeLanguageDetector::new();
        let result = composite.detect_async("Hello world").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[test]
    fn test_composite_detector_all_fail() {
        let mut composite = CompositeLanguageDetector::new();
        composite.add_detector(Box::new(FailingDetector));
        composite.add_detector(Box::new(FailingDetector));

        let result = composite.detect("Hello world");
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[tokio::test]
    async fn test_composite_detector_all_fail_async() {
        let mut composite = CompositeLanguageDetector::new();
        composite.add_detector(Box::new(FailingDetector));
        composite.add_detector(Box::new(FailingDetector));

        let result = composite.detect_async("Hello world").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[test]
    fn test_composite_detector_debug() {
        let composite = CompositeLanguageDetector::default();
        let debug_str = format!("{:?}", composite);
        assert_eq!(
            debug_str,
            "CompositeLanguageDetector with 0 detectors"
        );
    }
}

/// Test translator error handling edge cases
mod translator_tests {
    use super::*;

    #[test]
    fn test_translator_new_with_other_error() {
        // This test simulates the case where translations::translate returns an error other than UnsupportedLanguage
        // In the current implementation, this would be hard to trigger directly, but we test the error propagation
        let result = Translator::new(
            "invalid_really_long_unsupported_language_code",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_translator_translation_error_propagation() {
        let translator = Translator::new("en").unwrap();
        // Test with a key that doesn't exist to test error propagation
        let result =
            translator.translate("SomeVerySpecificNonExistentKey");
        // Should return an error for non-existent keys
        assert!(result.is_err());
    }
}

/// Test language detection edge cases
mod language_detection_tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_language_whitespace_only() {
        let result = detect_language_async("   \t\n   ").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[tokio::test]
    async fn test_detect_language_empty_string() {
        let result = detect_language_async("").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[tokio::test]
    async fn test_detect_language_numbers_only() {
        let result = detect_language_async("123456789").await;
        // This should either detect a language or fail
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    #[tokio::test]
    async fn test_detect_language_special_characters() {
        let result =
            detect_language_async("!@#$%^&*()_+-=[]{}|;':\",./<>?`~")
                .await;
        // This should either detect a language or fail
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    #[tokio::test]
    async fn test_detect_language_mixed_scripts() {
        let result = detect_language_async("Hello мир world").await;
        // Should detect one of the languages
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detect_language_single_word_fallback() {
        // Test the word-by-word fallback logic
        let result = detect_language_async("xyz Hello abc").await;
        // Should detect English from "Hello"
        assert!(result.is_ok());
        let lang = result.unwrap();
        assert_eq!(lang, "en");
    }
}

/// Test translations module edge cases
mod translations_tests {
    use super::*;
    use langweave::translations;

    #[test]
    fn test_translate_case_insensitive_fallback() {
        // Test the case-insensitive fallback in translate function
        let result = translations::translate("en", "hello"); // lowercase
        assert!(result.is_ok());
    }

    #[test]
    fn test_translate_nonexistent_key() {
        let result = translations::translate(
            "en",
            "NonExistentTranslationKey12345",
        );
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    #[test]
    fn test_translate_unsupported_language_error_message() {
        let result =
            translations::translate("unsupported_lang", "Hello");
        if let Err(I18nError::UnsupportedLanguage(lang)) = result {
            assert_eq!(lang, "unsupported_lang");
        } else {
            panic!("Expected UnsupportedLanguage error");
        }
    }

    #[test]
    fn test_translate_failed_error_message() {
        let result = translations::translate("en", "NonExistentKey");
        if let Err(I18nError::TranslationFailed(msg)) = result {
            assert!(msg.contains("en"));
            assert!(msg.contains("NonExistentKey"));
        } else {
            panic!("Expected TranslationFailed error");
        }
    }
}

/// Test language detector internal methods
mod language_detector_internals {
    use super::*;

    #[test]
    fn test_convert_lang_code_edge_cases() {
        let detector = LanguageDetector::new();

        // Test unknown language code conversion
        let result = detector.convert_lang_code(whatlang::Lang::Kor);
        assert_eq!(result, "ko");

        // Test various language codes to ensure full coverage
        let test_langs = vec![
            (whatlang::Lang::Eng, "en"),
            (whatlang::Lang::Fra, "fr"),
            (whatlang::Lang::Deu, "de"),
            (whatlang::Lang::Spa, "es"),
            (whatlang::Lang::Por, "pt"),
            (whatlang::Lang::Jpn, "ja"),
            (whatlang::Lang::Cmn, "zh"),
            (whatlang::Lang::Ara, "ar"),
            (whatlang::Lang::Hin, "hi"),
            (whatlang::Lang::Rus, "ru"),
        ];

        for (lang, expected) in test_langs {
            let result = detector.convert_lang_code(lang);
            assert_eq!(result, expected);
        }
    }

    #[tokio::test]
    async fn test_async_language_detection_task_failure() {
        let detector = LanguageDetector::new();

        // Test with extremely long input that might cause issues
        let very_long_text = "a".repeat(1_000_000);
        let result = detector.detect_async(&very_long_text).await;

        // Should either succeed or fail gracefully
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }
}

/// Test supported languages function edge cases
#[test]
fn test_supported_languages_content() {
    let languages = supported_languages();
    assert_eq!(languages.len(), 15);
    assert!(languages.contains(&"en".to_string()));
    assert!(languages.contains(&"fr".to_string()));
    assert!(languages.contains(&"de".to_string()));
    assert!(languages.contains(&"es".to_string()));
    assert!(languages.contains(&"pt".to_string()));
    assert!(languages.contains(&"it".to_string()));
    assert!(languages.contains(&"nl".to_string()));
    assert!(languages.contains(&"ru".to_string()));
    assert!(languages.contains(&"ar".to_string()));
    assert!(languages.contains(&"he".to_string()));
    assert!(languages.contains(&"hi".to_string()));
    assert!(languages.contains(&"ja".to_string()));
    assert!(languages.contains(&"ko".to_string()));
    assert!(languages.contains(&"zh".to_string()));
    assert!(languages.contains(&"id".to_string()));
}

/// Test is_language_supported with various inputs
#[test]
fn test_is_language_supported_edge_cases() {
    // Test case sensitivity
    assert!(is_language_supported("EN"));
    assert!(is_language_supported("Fr"));
    assert!(is_language_supported("DE"));

    // Test supported languages
    assert!(is_language_supported("es"));
    assert!(is_language_supported("pt"));
    assert!(is_language_supported("ja"));

    // Test truly unsupported languages
    assert!(!is_language_supported(""));
    assert!(!is_language_supported("invalid"));
    assert!(!is_language_supported("zz"));
    assert!(!is_language_supported("xx"));
}

/// Test error types completeness
mod error_tests {
    use super::*;

    #[test]
    fn test_error_display_implementations() {
        let unsupported_err =
            I18nError::UnsupportedLanguage("test".to_string());
        let detection_err = I18nError::LanguageDetectionFailed;
        let translation_err =
            I18nError::TranslationFailed("test error".to_string());

        // Test Display implementations
        assert!(format!("{}", unsupported_err).contains("test"));
        assert!(!format!("{}", detection_err).is_empty());
        assert!(format!("{}", translation_err).contains("test error"));
    }

    #[test]
    fn test_error_source_methods() {
        let unsupported_err =
            I18nError::UnsupportedLanguage("test".to_string());
        let detection_err = I18nError::LanguageDetectionFailed;
        let translation_err =
            I18nError::TranslationFailed("test error".to_string());

        // Test source methods (for Error trait)
        assert!(std::error::Error::source(&unsupported_err).is_none());
        assert!(std::error::Error::source(&detection_err).is_none());
        assert!(std::error::Error::source(&translation_err).is_none());
    }
}

/// Integration test for full workflow coverage
#[tokio::test]
async fn test_full_workflow_integration() {
    // Test the complete workflow from detection to translation
    let text = "Hello world, this is a test";

    // Detect language
    let detected_lang = detect_language_async(text).await;
    assert!(detected_lang.is_ok());
    let lang = detected_lang.unwrap();

    // Check if language is supported
    assert!(is_language_supported(&lang));

    // Translate using the detected language
    let translation = translate(&lang, "Hello");
    assert!(translation.is_ok());

    // Verify supported languages list
    let supported = supported_languages();
    assert!(supported.contains(&lang));
}

/// Test prelude module imports
#[test]
fn test_prelude_imports() {
    use langweave::prelude::*;

    // Test that all prelude items are accessible
    let _version = langweave::VERSION;
    let _supported = supported_languages();
    let _is_supported = is_language_supported("en");

    // Test error type from prelude
    let _error = I18nError::LanguageDetectionFailed;

    // Test translator from prelude
    let translator = Translator::new("en");
    assert!(translator.is_ok());
}
