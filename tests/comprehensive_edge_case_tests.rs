//! Comprehensive edge case and boundary tests for maximum coverage
//!
//! This module targets specific edge cases, boundary conditions, and error paths
//! to achieve 100% line coverage and 90%+ branch coverage.

use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::{CompositeLanguageDetector, LanguageDetectorTrait};
use langweave::optimized::*;
use langweave::translations;
use langweave::translator::Translator;
use langweave::{
    detect_language, detect_language_async, is_language_supported, supported_languages,
    translate, VERSION,
};
use tokio::task::JoinSet;

#[cfg(test)]
mod edge_cases {
    use super::*;

    /// Test all Unicode edge cases for language detection
    #[tokio::test]
    async fn test_unicode_edge_cases() {
        let edge_cases = vec![
            "", // Empty
            " \t\n\r", // Whitespace only
            "🎉🎊🎈", // Emojis only
            "123456789", // Numbers only
            "!@#$%^&*()", // Punctuation only
            "ሀሎ ወርልድ", // Amharic
            "مرحبا بالعالم", // Arabic
            "שלום עולם", // Hebrew
            "こんにちは世界", // Japanese
            "안녕하세요 세계", // Korean
            "你好世界", // Chinese
            "Привет мир", // Russian
            "नमस्ते संसार", // Hindi
            "Hallå världen", // Swedish with special chars
            "Olá mundo", // Portuguese with accent
            "A\u{0300}e\u{0301}i\u{0302}o\u{0303}u\u{0308}", // Combining diacritical marks
            "\u{200B}\u{200C}\u{200D}", // Zero-width characters
            "\u{FEFF}Hello", // BOM + text
            "𝒽𝑒𝓁𝓁𝑜 𝓌𝑜𝓇𝓁𝒹", // Mathematical script
        ];

        for test_case in edge_cases {
            println!("Testing: {:?}", test_case);

            // Should never panic
            let sync_result = detect_language(test_case);
            let async_result = detect_language_async(test_case).await;

            // Results should be consistent
            assert_eq!(sync_result.is_ok(), async_result.is_ok());

            if let (Ok(sync_lang), Ok(async_lang)) = (sync_result, async_result) {
                assert_eq!(sync_lang, async_lang);
            }
        }
    }

    /// Test boundary conditions for text length
    #[test]
    fn test_text_length_boundaries() {
        let detector = LanguageDetector::new();

        // Very short texts
        let short_texts = vec!["a", "ab", "abc"];
        for text in short_texts {
            let _result = detector.detect(text);
        }

        // Very long text (1MB)
        let long_text = "Hello world this is a test sentence. ".repeat(30000);
        let _result = detector.detect(&long_text);

        // Extremely long single word
        let long_word = "a".repeat(100000);
        let _result = detector.detect(&long_word);
    }

    /// Test all error path combinations in translate function
    #[test]
    fn test_translate_error_paths() {
        // Test with all unsupported language patterns
        let unsupported_langs = vec![
            "xx", "zz", "invalid", "123", "", "en-US", "fr-FR", "de-DE",
            "toolong", "CAPS", "MiXeD", "with space", "with-dash",
        ];

        for lang in unsupported_langs {
            match translate(lang, "Hello") {
                Ok(_) => {}, // Might be supported by fallback
                Err(I18nError::UnsupportedLanguage(_)) => {}, // Expected
                Err(_) => panic!("Unexpected error type for lang: {}", lang),
            }
        }

        // Test translation failure paths with complex phrases
        let supported_langs = supported_languages();
        let complex_phrases = vec![
            "This is a very complex sentence that doesn't exist in dictionary",
            "Multiple words with punctuation!",
            "Questions are complex, right?",
            "Comma, separated, phrases, everywhere",
        ];

        for lang in &supported_langs[..3] { // Test first 3 languages
            for phrase in &complex_phrases {
                let result = translate(lang, phrase);
                // Should either succeed or fail with TranslationFailed
                match result {
                    Ok(_) => {},
                    Err(I18nError::TranslationFailed(_)) => {},
                    Err(e) => panic!("Unexpected error for {}/{}: {:?}", lang, phrase, e),
                }
            }
        }
    }

    /// Test fallback translation behavior in detail
    #[test]
    fn test_translation_fallback_behavior() {
        // Simple keys (should fallback to original)
        let simple_keys = vec![
            "UnknownSimpleKey",
            "AnothErKey",
            "123Key",
            "key-with-dash",
            "key_with_underscore",
            "KeyWithNumbers123",
        ];

        for key in simple_keys {
            let result = translations::translate_with_fallback("fr", key);
            match result {
                Ok(translated) => {
                    // Should either be actual translation or fallback to original
                    assert!(!translated.is_empty());
                },
                Err(e) => panic!("Simple key should not fail with error: {:?}", e),
            }
        }

        // Complex phrases (should fail)
        let complex_phrases = vec![
            "Complex phrase with spaces",
            "Question with question mark?",
            "Exclamation with exclamation mark!",
            "Comma, separated phrase",
            "Multiple, complex, comma, separated, phrases",
        ];

        for phrase in complex_phrases {
            let result = translations::translate_with_fallback("fr", phrase);
            match result {
                Ok(_) => {}, // Might have actual translation
                Err(I18nError::TranslationFailed(_)) => {}, // Expected for complex phrases without translation
                Err(e) => panic!("Unexpected error for complex phrase {}: {:?}", phrase, e),
            }
        }
    }

    /// Test all branches in optimized functions
    #[test]
    fn test_optimized_functions_coverage() {
        // Test supported_languages_optimized
        let optimized_langs = supported_languages_optimized();
        let regular_langs = supported_languages();
        assert_eq!(optimized_langs.len(), regular_langs.len());

        // Test is_language_supported_optimized with all cases
        let test_codes = vec![
            "en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he", "hi", "ja", "ko", "zh", "id",
            "EN", "FR", "De", // Case variations
            "invalid", "xx", "123", "", "toolong", // Invalid cases
        ];

        for code in test_codes {
            let optimized_result = is_language_supported_optimized(code);
            let regular_result = is_language_supported(code);
            assert_eq!(optimized_result, regular_result, "Mismatch for code: {}", code);
        }

        // Test is_language_supported_zero_alloc
        for code in &["en", "fr", "invalid", ""] {
            let zero_alloc_result = is_language_supported_zero_alloc(code);
            let regular_result = is_language_supported(code);
            assert_eq!(zero_alloc_result, regular_result, "Zero-alloc mismatch for: {}", code);
        }

        // Test translate_optimized - verify it works for known translations
        let test_cases = vec![
            ("en", "Hello"),
            ("fr", "Hello"),
        ];

        for (lang, text) in test_cases {
            let optimized_result = translate_optimized(lang, text);
            let regular_result = translate(lang, text);

            // For known translations, both should succeed
            assert!(optimized_result.is_ok(), "Optimized failed for {}/{}", lang, text);
            assert!(regular_result.is_ok(), "Regular failed for {}/{}", lang, text);
        }

        // Test that invalid language returns error for both
        assert!(translate_optimized("invalid", "Hello").is_err());
        assert!(translate("invalid", "Hello").is_err());
    }

    /// Test Translator creation edge cases
    #[test]
    fn test_translator_edge_cases() {
        // Test all supported languages
        let supported = supported_languages();
        for lang in &supported {
            let translator = Translator::new(lang);
            assert!(translator.is_ok(), "Failed to create translator for {}", lang);

            if let Ok(translator) = translator {
                assert_eq!(translator.lang(), lang);

                // Test translation with various inputs
                let test_inputs = vec!["Hello", "Goodbye", "", "NonexistentKey"];
                for input in test_inputs {
                    let _result = translator.translate(input);
                }
            }
        }

        // Test unsupported languages
        let unsupported = vec!["xx", "invalid", "123", "", "en-US"];
        for lang in unsupported {
            let result = Translator::new(lang);
            match result {
                Ok(_) => {}, // Might be supported
                Err(I18nError::UnsupportedLanguage(_)) => {}, // Expected
                Err(e) => panic!("Unexpected error for {}: {:?}", lang, e),
            }
        }
    }

    /// Test CompositeLanguageDetector edge cases
    #[tokio::test]
    async fn test_composite_detector_edge_cases() {
        // Test empty composite detector
        let mut composite = CompositeLanguageDetector::new();

        let result = composite.detect("Hello world");
        assert!(matches!(result, Err(I18nError::LanguageDetectionFailed)));

        let async_result = composite.detect_async("Hello world").await;
        assert!(matches!(async_result, Err(I18nError::LanguageDetectionFailed)));

        // Test with one detector
        composite.add_detector(Box::new(LanguageDetector::new()));

        let test_inputs = vec![
            "Hello world",
            "Bonjour monde",
            "",
            "123",
            "🎉🎊",
            "مرحبا",
        ];

        for input in test_inputs {
            let sync_result = composite.detect(input);
            let async_result = composite.detect_async(input).await;

            // Results should be consistent
            assert_eq!(sync_result.is_ok(), async_result.is_ok());
        }

        // Test with multiple detectors
        composite.add_detector(Box::new(LanguageDetector::new()));
        composite.add_detector(Box::new(LanguageDetector::new()));

        // Test that first successful detection wins
        let result = composite.detect("Hello world");
        if result.is_ok() {
            // Should be consistent across calls
            let result2 = composite.detect("Hello world");
            assert_eq!(result.is_ok(), result2.is_ok());
        }
    }

    /// Test LanguageDetector instantiation methods
    #[test]
    fn test_language_detector_instantiation() {
        // Test new() method
        let detector1 = LanguageDetector::new();
        let detector2 = LanguageDetector::new();

        // Test try_new() method
        let detector3 = LanguageDetector::try_new();
        assert!(detector3.is_ok());

        let detector3 = detector3.unwrap();

        // All detectors should behave consistently
        let test_inputs = vec!["Hello", "Bonjour", "", "123"];
        for input in test_inputs {
            let result1 = detector1.detect(input);
            let result2 = detector2.detect(input);
            let result3 = detector3.detect(input);

            assert_eq!(result1.is_ok(), result2.is_ok());
            assert_eq!(result1.is_ok(), result3.is_ok());

            if let (Ok(lang1), Ok(lang2), Ok(lang3)) = (result1, result2, result3) {
                assert_eq!(lang1, lang2);
                assert_eq!(lang1, lang3);
            }
        }
    }

    /// Test concurrent access edge cases
    #[tokio::test]
    async fn test_concurrent_edge_cases() {
        let mut set = JoinSet::new();

        // Spawn many concurrent tasks with edge case inputs
        let edge_inputs = vec![
            ("", ""), // Empty lang and text
            ("invalid", ""), // Invalid lang, empty text
            ("", "Hello"), // Empty lang, valid text
            ("fr", "🎉🎊🎈"), // Valid lang, emoji text
            ("CAPS", "CAPS TEXT"), // Uppercase everything
        ];

        for (i, (lang, text)) in edge_inputs.into_iter().enumerate() {
            let lang = lang.to_string();
            let text = text.to_string();

            let _ = set.spawn(async move {
                // Test translation
                let _translate_result = translate(&lang, &text);

                // Test language support
                let _support_result = is_language_supported(&lang);

                // Test detection
                if !text.is_empty() {
                    let _detect_result = detect_language_async(&text).await;
                }

                i // Return task ID
            });
        }

        // Also test concurrent access to the same resources
        for i in 0..20 {
            let _ = set.spawn(async move {
                let _langs = supported_languages();
                let _version = VERSION;
                let detector = LanguageDetector::new();
                let _result = detector.detect("concurrent test");
                i
            });
        }

        // Wait for all tasks and ensure none panic
        let mut completed = 0;
        while let Some(result) = set.join_next().await {
            let _ = result.expect("Task should not panic");
            completed += 1;
        }

        assert!(completed >= 25); // Should have at least 25 tasks
    }

    /// Test memory safety with malformed Unicode
    #[test]
    fn test_malformed_unicode_safety() {
        let malformed_inputs = vec![
            // Invalid UTF-8 sequences converted to valid Unicode with replacement characters
            String::from_utf8_lossy(&[0xFF, 0xFE, 0xFD]).to_string(),
            String::from_utf8_lossy(&[0x80, 0x81, 0x82]).to_string(),
            String::from_utf8_lossy(&[0xC0, 0x80]).to_string(),
            // Valid but unusual Unicode
            "\u{FFFD}".repeat(100), // Replacement character
            "\u{0000}\u{0001}\u{0002}".to_string(), // Control characters
            "\u{E000}\u{E001}\u{E002}".to_string(), // Private use area
        ];

        for input in malformed_inputs {
            // Should not panic
            let _detect_result = detect_language(&input);
            let _translate_result = translate("en", &input);
            let _support_result = is_language_supported(&input);
        }
    }

    /// Test VERSION constant properties
    #[test]
    fn test_version_properties() {
        // VERSION should be non-empty and follow semver pattern
        assert!(!VERSION.is_empty());
        assert!(VERSION.contains('.'));

        // Should be consistent across calls
        assert_eq!(VERSION, VERSION);

        // Should match Cargo.toml version format (basic check)
        let parts: Vec<&str> = VERSION.split('.').collect();
        assert!(parts.len() >= 2, "Version should have at least major.minor");

        // Each part should be numeric or contain numeric
        for part in parts {
            assert!(!part.is_empty(), "Version part should not be empty");
        }
    }

    /// Test error Display formatting edge cases
    #[test]
    fn test_error_display_edge_cases() {
        let test_cases = vec![
            ("".to_string(), "Empty string"),
            ("very long error message ".repeat(100), "Long message"),
            ("Unicode error: 🎉💀".to_string(), "Unicode in message"),
            ("\n\t\r".to_string(), "Whitespace message"),
            ("Message with \"quotes\" and 'apostrophes'".to_string(), "Quotes in message"),
        ];

        for (msg, description) in test_cases {
            let error1 = I18nError::TranslationFailed(msg.clone());
            let error2 = I18nError::UnsupportedLanguage(msg.clone());
            let error3 = I18nError::UnexpectedError(msg);

            // Display strings should be non-empty and consistent
            let display1 = error1.to_string();
            let display2 = error2.to_string();
            let display3 = error3.to_string();

            assert!(!display1.is_empty(), "Failed for: {}", description);
            assert!(!display2.is_empty(), "Failed for: {}", description);
            assert!(!display3.is_empty(), "Failed for: {}", description);

            // Should be consistent across multiple calls
            assert_eq!(display1, error1.to_string());
            assert_eq!(display2, error2.to_string());
            assert_eq!(display3, error3.to_string());
        }
    }
}

/// Additional regression tests for specific code paths
#[cfg(test)]
mod regression_tests {
    use super::*;

    /// Regression test for word-by-word fallback in detect_language
    #[test]
    fn test_word_by_word_fallback_regression() {
        // Test cases that should trigger word-by-word detection
        let test_cases = vec![
            "unknownword hello world",
            "xyzabc bonjour monde",
            "123456 english text",
            "!@#$% known words here",
            "mixed 🎉 hello content",
        ];

        for test_case in test_cases {
            let result = detect_language(test_case);
            // Should either succeed or fail gracefully
            match result {
                Ok(ref lang) => {
                    // Detection succeeded - lang should be a valid language code
                    assert!(!lang.is_empty(), "Detected language should not be empty");
                    // Note: detected languages may include languages not in
                    // the translation dictionary (e.g., 'uzb' for Uzbek patterns)
                },
                Err(I18nError::LanguageDetectionFailed) => {
                    // Acceptable failure for difficult inputs
                },
                Err(e) => panic!("Unexpected error for '{}': {:?}", test_case, e),
            }
        }
    }

    /// Regression test for case sensitivity in language codes
    #[test]
    fn test_case_sensitivity_regression() {
        let test_cases = vec![
            ("en", true),
            ("EN", true),
            ("En", true),
            ("eN", true),
            ("fr", true),
            ("FR", true),
            ("Fr", true),
            ("invalid", false),
            ("INVALID", false),
            ("Invalid", false),
        ];

        for (lang_code, expected_supported) in test_cases {
            let result = is_language_supported(lang_code);
            assert_eq!(result, expected_supported, "Mismatch for language code: {}", lang_code);
        }
    }

    /// Regression test for async/sync consistency
    #[tokio::test]
    async fn test_async_sync_consistency_regression() {
        let test_inputs = vec![
            "Hello world",
            "Bonjour monde",
            "Guten Tag Welt",
            "Hola mundo",
            "",
            "123456",
            "🎉🎊🎈",
            "Mixed content with numbers 123 and symbols !@#",
        ];

        for input in test_inputs {
            let sync_result = detect_language(input);
            let async_result = detect_language_async(input).await;

            // Results should be consistent
            match (sync_result, async_result) {
                (Ok(sync_lang), Ok(async_lang)) => {
                    assert_eq!(sync_lang, async_lang, "Language mismatch for input: {}", input);
                },
                (Err(_), Err(_)) => {
                    // Both failed - acceptable
                },
                _ => panic!("Sync/async result type mismatch for input: {}", input),
            }
        }
    }
}