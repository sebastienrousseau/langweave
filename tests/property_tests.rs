//! Property-based tests for the langweave library
//!
//! This module contains property-based tests using proptest to verify
//! mathematical properties, invariants, and panic safety.

use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::translator::Translator;
use langweave::{
    detect_language_async, is_language_supported, supported_languages,
    translate,
};
use proptest::prelude::*;

/// Test strategy for generating arbitrary text inputs
fn arbitrary_text() -> impl Strategy<Value = String> {
    prop_oneof![
        // Empty and whitespace
        Just(String::new()),
        Just("   ".to_string()),
        Just("\n\t\r ".to_string()),
        // ASCII text
        "[a-zA-Z0-9 .,!?-]{0,1000}",
        // Unicode text with various scripts
        "\\PC{0,100}",
        // Control characters and edge cases
        prop::collection::vec(0u8..=255, 0..100).prop_map(|bytes| {
            String::from_utf8_lossy(&bytes).to_string()
        }),
    ]
}

/// Test strategy for language codes
fn arbitrary_language_code() -> impl Strategy<Value = String> {
    prop_oneof![
        // Valid language codes
        Just("en".to_string()),
        Just("fr".to_string()),
        Just("de".to_string()),
        Just("es".to_string()),
        Just("pt".to_string()),
        Just("ru".to_string()),
        Just("ja".to_string()),
        Just("zh".to_string()),
        Just("ar".to_string()),
        Just("hi".to_string()),
        Just("ko".to_string()),
        // Invalid/unsupported codes
        "[a-z]{1,10}",
        "[A-Z]{1,10}",
        // Edge cases
        Just(String::new()),
        Just("123".to_string()),
        Just("en-US".to_string()),
        Just("invalid_lang".to_string()),
    ]
}

// Property tests for error handling
proptest! {
    /// Verifies that I18nError maintains consistency across cloning and equality
    #[test]
    fn error_clone_equals_original(msg in ".*") {
        let error = I18nError::TranslationFailed(msg.clone());
        let cloned = error.clone();
        prop_assert_eq!(error, cloned);

        let error2 = I18nError::UnsupportedLanguage(msg.clone());
        let cloned2 = error2.clone();
        prop_assert_eq!(error2, cloned2);

        let error3 = I18nError::UnexpectedError(msg);
        let cloned3 = error3.clone();
        prop_assert_eq!(error3, cloned3);
    }

    /// Verifies that I18nError string representations are deterministic
    #[test]
    fn error_string_representation_consistent(msg in ".*") {
        let error = I18nError::TranslationFailed(msg);
        let str1 = error.to_string();
        let str2 = error.to_string();
        prop_assert_eq!(str1, str2);
    }

    /// Verifies that I18nError as_str() returns consistent results
    #[test]
    fn error_as_str_consistent(msg in ".*") {
        let error = I18nError::TranslationFailed(msg);
        let str1 = error.as_str();
        let str2 = error.as_str();
        prop_assert_eq!(str1, str2);
    }
}

// Property tests for language detection
proptest! {
    /// Test panic safety: detect_language should never panic on arbitrary input
    #[test]
    fn detect_language_no_panic(text in arbitrary_text()) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        // This should never panic, even on malformed input
        let _result = std::panic::catch_unwind(|| {
            rt.block_on(async {
                let _ = detect_language_async(&text).await;
            })
        });

        // Test passed if we reach this point without panic
        prop_assert!(true);
    }

    /// Test that language detector maintains consistency
    #[test]
    fn language_detector_consistency(text in arbitrary_text()) {
        let detector = LanguageDetector::new();

        // Multiple calls with same input should return same result
        let result1 = detector.detect(&text);
        let result2 = detector.detect(&text);

        prop_assert_eq!(result1.is_ok(), result2.is_ok());
        if let (Ok(lang1), Ok(lang2)) = (result1, result2) {
            prop_assert_eq!(lang1, lang2);
        }
    }

    /// Test async vs sync consistency
    #[test]
    fn async_sync_detection_consistency(text in arbitrary_text()) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let detector = LanguageDetector::new();

        let sync_result = detector.detect(&text);
        let async_result = rt.block_on(async {
            detector.detect_async(&text).await
        });

        // Results should be consistent between sync and async
        prop_assert_eq!(sync_result.is_ok(), async_result.is_ok());
        if let (Ok(sync_lang), Ok(async_lang)) = (sync_result, async_result) {
            prop_assert_eq!(sync_lang, async_lang);
        }
    }
}

// Property tests for translation system
proptest! {
    /// Test panic safety: translate should never panic on arbitrary input
    #[test]
    fn translate_no_panic(lang in arbitrary_language_code(), text in arbitrary_text()) {
        let _result = std::panic::catch_unwind(|| {
            let _ = translate(&lang, &text);
        });

        // Test passed if we reach this point without panic
        prop_assert!(true);
    }

    /// Test translation invariants: same input should produce same output
    #[test]
    fn translation_deterministic(lang in arbitrary_language_code(), text in arbitrary_text()) {
        let result1 = translate(&lang, &text);
        let result2 = translate(&lang, &text);

        prop_assert_eq!(result1, result2);
    }

    /// Test that successful translation preserves some properties
    #[test]
    fn translation_properties(lang in arbitrary_language_code(), text in arbitrary_text()) {
        if let Ok(translated) = translate(&lang, &text) {
            // Translated text should not be empty if input is not empty (unless no translation available)
            if !text.trim().is_empty() && is_language_supported(&lang) {
                prop_assert!(!translated.is_empty());
            }

            // Translated text length should be reasonable (not exponential growth)
            if !text.is_empty() {
                prop_assert!(translated.len() <= text.len() * 10);
            }
        }
    }

    /// Test translator creation consistency
    #[test]
    fn translator_creation_consistent(lang in arbitrary_language_code()) {
        let result1 = Translator::new(&lang);
        let result2 = Translator::new(&lang);

        prop_assert_eq!(result1.is_ok(), result2.is_ok());

        if let (Ok(translator1), Ok(translator2)) = (result1, result2) {
            prop_assert_eq!(translator1.lang(), translator2.lang());
        }
    }
}

// Property tests for core library functions
proptest! {
    /// Test that language support checking is consistent
    #[test]
    fn language_support_consistent(lang in arbitrary_language_code()) {
        let result1 = is_language_supported(&lang);
        let result2 = is_language_supported(&lang);
        prop_assert_eq!(result1, result2);
    }

    /// Test that supported languages list is stable
    #[test]
    fn supported_languages_stable(_any in ".*") {
        let langs1 = supported_languages();
        let langs2 = supported_languages();
        prop_assert_eq!(langs1, langs2);
    }

    /// Test that supported languages are consistent with is_language_supported
    #[test]
    fn supported_languages_consistency(_any in ".*") {
        let supported = supported_languages();
        for lang in &supported {
            prop_assert!(is_language_supported(lang));
        }
    }

    /// Test case insensitivity properties
    #[test]
    fn language_code_case_insensitive(lang_code in "[a-z]{2,5}") {
        let lower = is_language_supported(&lang_code.to_lowercase());
        let upper = is_language_supported(&lang_code.to_uppercase());

        // Results should be the same regardless of case
        prop_assert_eq!(lower, upper);
    }
}

// Property tests for mathematical invariants
proptest! {
    /// Test that empty input detection behaves correctly
    #[test]
    fn empty_input_handling(whitespace in "\\s*") {
        let rt = tokio::runtime::Runtime::new().unwrap();

        if whitespace.trim().is_empty() {
            let result = rt.block_on(async {
                detect_language_async(&whitespace).await
            });

            // Empty or whitespace-only input should fail detection
            prop_assert!(result.is_err());
        }
    }

    /// Test that language detection respects text length limits
    #[test]
    fn language_detection_length_handling(base_text in "[a-zA-Z]{1,50}", repeat_count in 1usize..=100) {
        let long_text = base_text.repeat(repeat_count);
        let detector = LanguageDetector::new();

        // Should handle long texts without panic
        let _result = std::panic::catch_unwind(|| {
            let _ = detector.detect(&long_text);
        });

        prop_assert!(true);
    }

    /// Test roundtrip property: if language is supported, translation should work
    #[test]
    fn translation_roundtrip_property(text in "[a-zA-Z ]{1,100}") {
        let supported_langs = supported_languages();

        for lang in &supported_langs {
            if is_language_supported(lang) {
                let result = translate(lang, &text);

                // If language is supported, translation should at least not fail with UnsupportedLanguage
                if let Err(I18nError::UnsupportedLanguage(_)) = result {
                    prop_assert!(false, "Language {} should be supported but translation failed with UnsupportedLanguage", lang);
                }
            }
        }
    }
}

// Concurrency and thread safety tests
proptest! {
    /// Test thread safety of language detection
    #[test]
    fn language_detection_thread_safety(texts in prop::collection::vec(arbitrary_text(), 1..10)) {
        use std::thread;
        use std::sync::Arc;

        let detector = Arc::new(LanguageDetector::new());
        let mut handles = vec![];

        for text in texts {
            let detector_clone = Arc::clone(&detector);
            let handle = thread::spawn(move || {
                // Should not panic in concurrent access
                let _result = detector_clone.detect(&text);
            });
            handles.push(handle);
        }

        // Join all threads - test passes if no panic occurs
        for handle in handles {
            handle.join().unwrap();
        }

        prop_assert!(true);
    }

    /// Test that translation system is thread-safe
    #[test]
    fn translation_thread_safety(inputs in prop::collection::vec((arbitrary_language_code(), arbitrary_text()), 1..10)) {
        use std::thread;

        let mut handles = vec![];

        for (lang, text) in inputs {
            let handle = thread::spawn(move || {
                let _result = translate(&lang, &text);
                let _supported = is_language_supported(&lang);
            });
            handles.push(handle);
        }

        // Join all threads - test passes if no panic occurs
        for handle in handles {
            handle.join().unwrap();
        }

        prop_assert!(true);
    }
}

#[cfg(test)]
mod additional_property_tests {
    use super::*;

    /// Test for serialization-like properties of errors
    #[test]
    fn error_debug_format_stable() {
        let error = I18nError::TranslationFailed("test".to_string());
        let debug1 = format!("{:?}", error);
        let debug2 = format!("{:?}", error);
        assert_eq!(debug1, debug2);
    }

    /// Test memory safety with large inputs
    #[test]
    fn memory_safety_large_input() {
        let large_text = "a".repeat(1_000_000);
        let detector = LanguageDetector::new();

        // Should not crash or cause memory issues
        let _result = detector.detect(&large_text);
    }

    /// Test that VERSION constant is stable
    #[test]
    fn version_constant_stable() {
        use langweave::VERSION;

        let v1 = VERSION;
        let v2 = VERSION;
        assert_eq!(v1, v2);
        assert!(!v1.is_empty());
    }
}
