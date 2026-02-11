//! Final coverage validation tests for LangWeave library.
//!
//! This test suite provides comprehensive validation testing to ensure
//! all code paths are properly covered and validated.

use langweave::{detect_language_async, error::I18nError, translate};

#[cfg(test)]
mod lib_final_coverage {
    use super::*;

    #[test]
    fn test_translate_simple_fallback_scenarios() {
        // Test simple key fallback for unknown words
        // These should trigger the fallback path (lines 100-102)
        let result = translate("fr", "unknownkey");
        assert!(result.is_ok()); // Should fallback to original
        assert_eq!(result.unwrap(), "unknownkey");

        let result = translate("de", "simpleword");
        assert!(result.is_ok()); // Should fallback to original
        assert_eq!(result.unwrap(), "simpleword");
    }

    #[test]
    fn test_translate_complex_phrase_no_fallback() {
        // Test complex phrases that should NOT fallback (lines 97-99)
        let result = translate("fr", "unknown phrase with spaces");
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));

        let result = translate("fr", "unknown,comma");
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));

        let result = translate("fr", "unknown?question");
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));

        let result = translate("fr", "unknown!exclamation");
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    #[test]
    fn test_translate_non_translation_failed_errors() {
        // Test other error types being propagated (line 104)
        // This requires triggering UnsupportedLanguage from translator, not from lang check
        let result = translate("", "test"); // Empty language should trigger UnsupportedLanguage
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[tokio::test]
    async fn test_detect_language_word_by_word_fallback() {
        // Test word-by-word detection fallback (lines 158-168)
        // Use text where full text detection fails but individual words succeed
        let result = detect_language_async("xyz123 hello abc789").await;
        // Should detect "hello" as English in word-by-word fallback
        assert!(result.is_ok());
        let lang = result.unwrap();
        assert_eq!(lang, "en");
    }

    #[tokio::test]
    async fn test_detect_language_no_detection_possible() {
        // Test case where no language is detected at all (line 171)
        let result = detect_language_async("123456 !@#$%^ 789").await;
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));

        let result = detect_language_async("   ").await; // Only whitespace
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[tokio::test]
    async fn test_detect_language_debug_logging_paths() {
        // Test debug logging paths (lines 143, 153, 162-165)
        let result =
            detect_language_async("The quick brown fox jumps").await;
        assert!(result.is_ok()); // Should trigger debug log at line 153

        let result = detect_language_async("xyz hello").await;
        assert!(result.is_ok()); // Should trigger debug log at lines 162-165
    }
}

#[cfg(feature = "async")]
#[cfg(test)]
mod async_utils_final_coverage {
    use langweave::async_utils::translate_async;
    use langweave::error::I18nError;

    #[tokio::test]
    async fn test_translate_async_translator_creation_failure() {
        // Test translator creation error path (lines 262-267)
        let result =
            translate_async("invalid_lang_for_creation", "test").await;
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[tokio::test]
    async fn test_translate_async_translation_failure() {
        // Test translation failure error path
        let result =
            translate_async("fr", "nonexistent_translation_key_xyz")
                .await;
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    #[tokio::test]
    async fn test_translate_async_unsupported_language() {
        // Test unsupported language error path (lines 257-261)
        let result = translate_async("xyz", "Hello").await;
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));

        if let Err(I18nError::UnsupportedLanguage(lang)) = result {
            assert_eq!(lang, "xyz");
        }
    }
}

#[cfg(test)]
mod edge_case_coverage {
    use langweave::error::I18nError;
    use langweave::{
        is_language_supported, supported_languages, translate,
    };

    #[test]
    fn test_translate_error_propagation_paths() {
        // Test various error propagation scenarios
        let result = translate("zz", "test");
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[test]
    fn test_language_support_edge_cases() {
        // Test case sensitivity in language support
        assert!(is_language_supported("EN")); // Should convert to lowercase
        assert!(is_language_supported("Fr"));
        assert!(is_language_supported("DE"));

        // Test unsupported languages
        assert!(!is_language_supported(""));
        assert!(!is_language_supported("xyz"));
        assert!(!is_language_supported("unknown"));
    }

    #[test]
    fn test_supported_languages_content() {
        let languages = supported_languages();
        assert_eq!(languages.len(), 15);

        // Test all 15 required languages
        let expected_languages = vec![
            "en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he",
            "hi", "ja", "ko", "zh", "id",
        ];

        for lang in expected_languages {
            assert!(languages.contains(&lang),
                   "Expected language '{}' not found in supported languages", lang);
        }
    }
}
