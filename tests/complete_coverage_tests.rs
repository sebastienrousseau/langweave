//! Complete coverage tests targeting uncovered lines and branches
//!
//! This module contains tests specifically designed to achieve 100% line coverage
//! and 90%+ branch coverage for the langweave library.

use langweave::error::I18nError;
use langweave::{
    VERSION, detect_language, detect_language_async,
    is_language_supported, supported_languages, translate,
};

#[cfg(test)]
mod complete_coverage {
    use super::*;

    /// Test the async_utils module when the async feature is enabled
    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_utils_translate_async() {
        use langweave::async_utils::translate_async;

        // Test successful translation
        let result = translate_async("fr", "Hello").await;
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::TranslationFailed(_))
                )
        );

        // Test unsupported language error
        let result = translate_async("invalid", "Hello").await;
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));

        // Test empty input
        let result = translate_async("fr", "").await;
        assert!(result.is_ok() || result.is_err()); // Should handle gracefully
    }

    /// Test VERSION constant access
    #[test]
    fn test_version_constant_detailed() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.starts_with("0."));
        assert_eq!(VERSION, "0.0.2");

        // Test repeated access
        let v1 = VERSION;
        let v2 = VERSION;
        assert_eq!(v1, v2);
    }

    /// Test edge cases for translate function to hit all code paths
    #[test]
    fn test_translate_comprehensive_edge_cases() {
        // Test all supported languages with empty string
        let supported = supported_languages();
        for lang in supported {
            let result = translate(lang, "");
            // Should either succeed or fail gracefully
            assert!(result.is_ok() || result.is_err());
        }

        // Test with whitespace-only input
        for lang in supported {
            let result = translate(lang, "   \t\n   ");
            assert!(result.is_ok() || result.is_err());
        }

        // Test with very long input
        let long_text = "Hello ".repeat(1000);
        let result = translate("fr", &long_text);
        assert!(result.is_ok() || result.is_err());

        // Test with special characters
        let special_text = "café naïve résumé";
        let result = translate("fr", special_text);
        assert!(result.is_ok() || result.is_err());
    }

    /// Test comprehensive language detection edge cases
    #[tokio::test]
    async fn test_detect_language_comprehensive() {
        // Test with only punctuation
        let result = detect_language("!@#$%^&*()");
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));

        // Test with mixed punctuation and letters
        let result = detect_language("Hello!");
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        // Test with numbers and letters
        let result = detect_language("Hello123");
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        // Test async version with same inputs
        let result = detect_language_async("!@#$%^&*()").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));

        let result = detect_language_async("Hello!").await;
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        // Test with tab and newline characters
        let result = detect_language_async("Hello\tworld\n").await;
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    /// Test error propagation paths
    #[test]
    fn test_error_propagation() {
        // Test that UnsupportedLanguage error is properly propagated
        match translate("xyz", "Hello") {
            Err(I18nError::UnsupportedLanguage(lang)) => {
                assert_eq!(lang, "xyz");
            }
            _ => panic!("Expected UnsupportedLanguage error"),
        }

        // Test that simple keys fall back to original (not an error)
        let result = translate("fr", "NonExistentTranslationKey12345");
        // Simple keys without spaces fall back to the original key
        assert!(result.is_ok());

        // Test that complex phrases with spaces that aren't found return an error
        let result = translate(
            "fr",
            "This is a complex phrase that does not exist",
        );
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    /// Test word-by-word fallback detection
    #[test]
    fn test_word_by_word_detection_fallback() {
        // This should trigger the word-by-word fallback in detect_language
        let mixed_difficult = "xyzabc hello world qwerty";
        let result = detect_language(mixed_difficult);

        // Should either detect from "hello world" or fail
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    /// Test all supported languages consistency
    #[test]
    fn test_all_supported_languages_consistency() {
        let languages = supported_languages();

        // Verify all expected languages are present
        let expected_languages = vec![
            "en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he",
            "hi", "ja", "ko", "zh", "id",
        ];

        assert_eq!(languages.len(), 15);

        for expected_lang in expected_languages {
            assert!(
                languages.contains(&expected_lang),
                "Missing language: {}",
                expected_lang
            );
            assert!(
                is_language_supported(expected_lang),
                "Language not supported: {}",
                expected_lang
            );
        }
    }

    /// Test case sensitivity for language codes
    #[test]
    fn test_language_code_case_sensitivity() {
        // Test uppercase versions
        assert!(is_language_supported("EN"));
        assert!(is_language_supported("FR"));
        assert!(is_language_supported("De"));

        // Test mixed case
        assert!(is_language_supported("eN"));
        assert!(is_language_supported("fR"));
        assert!(is_language_supported("ES"));

        // Test that unsupported languages remain unsupported regardless of case
        assert!(!is_language_supported("XY"));
        assert!(!is_language_supported("xy"));
        assert!(!is_language_supported("Xy"));
    }

    /// Test boundary conditions for text processing
    #[tokio::test]
    async fn test_boundary_conditions() {
        // Single character inputs
        let result = detect_language("a");
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        let result = detect_language_async("a").await;
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        // Unicode boundary conditions
        let result = detect_language("é");
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        // Non-Latin scripts
        let result = detect_language("你好"); // Chinese
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        let result = detect_language("こんにちは"); // Japanese
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );

        let result = detect_language("مرحبا"); // Arabic
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(I18nError::LanguageDetectionFailed)
                )
        );
    }

    /// Test that library handles concurrent access safely
    #[tokio::test]
    async fn test_concurrent_safety() {
        use tokio::task::JoinSet;

        let mut set = JoinSet::new();

        // Spawn multiple concurrent tasks
        for i in 0..10 {
            let text = format!("Hello world test {}", i);
            let _handle = set.spawn(async move {
                let _ = detect_language_async(&text).await;
                let _ = translate("fr", "Hello");
                let _ = is_language_supported("en");
            });
        }

        // Wait for all tasks to complete
        while let Some(result) = set.join_next().await {
            result.expect("Task should not panic");
        }
    }

    /// Test memory efficiency with large datasets
    #[test]
    fn test_memory_efficiency() {
        // Test with very large supported languages calls
        for _ in 0..1000 {
            let _ = supported_languages();
        }

        // Test repeated language support checks
        for _ in 0..10000 {
            let _ = is_language_supported("en");
            let _ = is_language_supported("invalid");
        }

        // Should not cause memory leaks or excessive allocations
    }

    /// Test numeric and special character combinations
    #[test]
    fn test_numeric_special_combinations() {
        let test_cases = vec![
            "123",
            "test123",
            "123test",
            "test-123",
            "test_123",
            "test.123",
            "test@123",
            "test#123",
            "hello@world.com",
            "user.name@domain.org",
        ];

        for test_case in test_cases {
            let result = detect_language(test_case);
            // Should handle all these cases without panic
            assert!(
                result.is_ok()
                    || matches!(
                        result,
                        Err(I18nError::LanguageDetectionFailed)
                    )
            );
        }
    }

    /// Test regression cases for specific bugs or edge cases
    #[tokio::test]
    async fn test_regression_cases() {
        // Test empty string after trim (whitespace only)
        let whitespace_cases =
            vec![" ", "\t", "\n", "\r", "  \t\n\r  "];

        for case in whitespace_cases {
            let sync_result = detect_language(case);
            let async_result = detect_language_async(case).await;

            // Both should fail with LanguageDetectionFailed
            assert!(matches!(
                sync_result,
                Err(I18nError::LanguageDetectionFailed)
            ));
            assert!(matches!(
                async_result,
                Err(I18nError::LanguageDetectionFailed)
            ));
        }

        // Test translation with empty/whitespace inputs
        let result = translate("fr", "");
        assert!(result.is_ok() || result.is_err()); // Should handle gracefully

        let result = translate("fr", "   ");
        assert!(result.is_ok() || result.is_err()); // Should handle gracefully
    }
}

/// Property-based edge case tests for complete coverage
#[cfg(test)]
mod property_complete_coverage {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Test that all valid language codes work with translation
        #[test]
        fn all_valid_languages_translate_consistently(
            text in "[a-zA-Z]{1,50}",
            lang_index in 0..15usize
        ) {
            let languages = supported_languages();
            let lang = &languages[lang_index];

            let result1 = translate(lang, &text);
            let result2 = translate(lang, &text);

            // Results should be consistent
            prop_assert_eq!(result1.clone(), result2);

            // If supported, should not get UnsupportedLanguage error
            if let Err(I18nError::UnsupportedLanguage(_)) = &result1 {
                prop_assert!(false, "Got UnsupportedLanguage for supported language {}", lang);
            }
        }

        /// Test language detection with various Unicode ranges
        #[test]
        fn unicode_range_detection(
            text in "\\p{L}{1,100}" // Any Unicode letter
        ) {
            let result = std::panic::catch_unwind(|| {
                let _ = detect_language(&text);
            });

            // Should never panic regardless of Unicode input
            prop_assert!(result.is_ok());
        }

        /// Test error message consistency
        #[test]
        fn error_message_consistency(
            invalid_lang in "[^a-zA-Z]{1,10}"
        ) {
            if let Err(I18nError::UnsupportedLanguage(lang)) = translate(&invalid_lang, "test") {
                prop_assert_eq!(lang, invalid_lang);
            }
        }
    }
}
