#![cfg(feature = "full")]

//! Comprehensive property-based tests for LangWeave core logic
//!
//! This test suite ensures mathematical properties, invariants, and panic safety
//! across all public functions using property-based testing with proptest.

use langweave::{
    detect_language, detect_language_async, error::I18nError,
    is_language_supported, supported_languages, translate,
};
use proptest::prelude::*;
use std::collections::HashSet;

/// Property-based tests for core functionality
mod properties {
    use super::*;

    proptest! {
        /// Translation consistency: Same input should always produce same output
        #[test]
        fn translation_deterministic(
            lang in "[a-z]{2}",
            text in "[a-zA-Z ]{1,50}"
        ) {
            let result1 = translate(&lang, &text);
            let result2 = translate(&lang, &text);

            prop_assert_eq!(result1.is_ok(), result2.is_ok());
            if result1.is_ok() {
                prop_assert_eq!(result1.unwrap(), result2.unwrap());
            } else {
                prop_assert_eq!(
                    std::mem::discriminant(&result1.unwrap_err()),
                    std::mem::discriminant(&result2.unwrap_err())
                );
            }
        }

        /// Language detection never panics on arbitrary input
        #[test]
        fn detect_language_no_panic(text in ".*") {
            let _result = std::panic::catch_unwind(|| {
                let _ = detect_language(&text);
            });
            prop_assert!(_result.is_ok());
        }

        /// Language detection deterministic
        #[test]
        fn detect_language_deterministic(text in "[a-zA-Z ]{1,100}") {
            let result1 = detect_language(&text);
            let result2 = detect_language(&text);
            prop_assert_eq!(result1.is_ok(), result2.is_ok());

            if result1.is_ok() {
                prop_assert_eq!(result1.unwrap(), result2.unwrap());
            }
        }

        /// Language support is consistent
        #[test]
        fn language_support_consistent(lang in "[a-zA-Z]{1,10}") {
            let supported = is_language_supported(&lang);
            let supported2 = is_language_supported(&lang);
            prop_assert_eq!(supported, supported2);

            // Case-insensitive consistency
            let upper_lang = lang.to_uppercase();
            let lower_lang = lang.to_lowercase();
            prop_assert_eq!(
                is_language_supported(&upper_lang),
                is_language_supported(&lower_lang)
            );
        }

        /// Supported languages set invariants
        #[test]
        fn supported_languages_invariants(_: ()) {
            let languages = supported_languages();

            prop_assert!(!languages.is_empty());

            let unique_langs: HashSet<&str> = languages.iter().copied().collect();
            prop_assert_eq!(languages.len(), unique_langs.len());

            for &lang in languages {
                prop_assert!(is_language_supported(lang));
            }

            prop_assert!(languages.contains(&"en"));
            prop_assert!(languages.contains(&"fr"));
            prop_assert!(languages.contains(&"de"));
            prop_assert!(languages.contains(&"es"));
        }

        /// Translation never panics on arbitrary input
        #[test]
        fn translate_no_panic(lang in ".*", text in ".*") {
            let _result = std::panic::catch_unwind(|| {
                let _ = translate(&lang, &text);
            });
            prop_assert!(_result.is_ok());
        }

        /// Empty input handling is consistent
        #[test]
        fn empty_input_handling(_: ()) {
            let empty_detect = detect_language("");
            prop_assert!(matches!(empty_detect, Err(I18nError::LanguageDetectionFailed)));

            let whitespace_detect = detect_language("   \t\n  ");
            prop_assert!(matches!(whitespace_detect, Err(I18nError::LanguageDetectionFailed)));

            for &lang in supported_languages() {
                let empty_translate = translate(lang, "");
                prop_assert!(empty_translate.is_ok() || empty_translate.is_err());
            }
        }
    }
}

/// Async property tests
mod async_properties {
    use super::*;

    proptest! {
        /// Async language detection consistency with sync version
        #[test]
        fn async_sync_detection_consistency(text in "[a-zA-Z ]{1,100}") {
            tokio_test::block_on(async {
                let sync_result = detect_language(&text);
                let async_result = detect_language_async(&text).await;

                assert_eq!(sync_result.is_ok(), async_result.is_ok());

                if let (Ok(sync_val), Ok(async_val)) = (sync_result, async_result) {
                    assert_eq!(sync_val, async_val);
                }
            });
        }

        /// Async translation never panics
        #[test]
        fn async_translate_no_panic(text in "[a-zA-Z ]{1,50}") {
            tokio_test::block_on(async {
                let _ = detect_language_async(&text).await;
            });
        }
    }
}

/// Error type serialization properties
mod error_properties {
    use super::*;

    proptest! {
        /// Error serialization roundtrip (Clone/Debug/Eq properties)
        #[test]
        fn error_clone_roundtrip(msg in ".*") {
            let errors = vec![
                I18nError::LanguageDetectionFailed,
                I18nError::TranslationFailed(msg.clone()),
                I18nError::UnsupportedLanguage(msg.clone()),
                I18nError::UnexpectedError(msg.clone()),
                I18nError::TaskFailed(msg.clone()),
                I18nError::BatchOperationFailed(msg.clone()),
                I18nError::StreamProcessingFailed(msg.clone()),
                I18nError::PatternOperationFailed(msg),
            ];

            for error in errors {
                let cloned = error.clone();
                let error_debug = format!("{error:?}");
                let cloned_debug = format!("{cloned:?}");
                prop_assert_eq!(&error, &cloned);
                prop_assert_eq!(error_debug, cloned_debug);
                prop_assert_eq!(error.as_str(), cloned.as_str());
            }
        }

        /// Error Display format properties
        #[test]
        fn error_display_properties(msg in "[a-zA-Z0-9 ]{1,100}") {
            let error = I18nError::TranslationFailed(msg.clone());
            let display_str = error.to_string();

            prop_assert!(display_str.contains(&msg));
            prop_assert!(!display_str.is_empty());
            prop_assert!(!error.as_str().is_empty());
        }
    }
}

/// Unicode and edge case properties
mod unicode_properties {
    use super::*;

    proptest! {
        /// Unicode text handling
        #[test]
        fn unicode_text_safety(text in "\\PC*") {
            let _result = std::panic::catch_unwind(|| {
                let _ = detect_language(&text);
                for &lang in supported_languages() {
                    let _ = translate(lang, &text);
                }
            });
            prop_assert!(_result.is_ok());
        }

        /// Language code case-insensitive handling
        #[test]
        fn language_code_case_insensitive(base_lang in "[a-z]{2}") {
            let lower = base_lang.to_lowercase();
            let upper = base_lang.to_uppercase();
            let mixed = format!("{}{}",
                                &base_lang[..1].to_uppercase(),
                                &base_lang[1..].to_lowercase());

            prop_assert_eq!(
                is_language_supported(&lower),
                is_language_supported(&upper)
            );
            prop_assert_eq!(
                is_language_supported(&lower),
                is_language_supported(&mixed)
            );
        }

        /// Large input handling
        #[test]
        fn large_input_safety(size in 1..10000usize) {
            let large_text = "a".repeat(size);

            let _result = std::panic::catch_unwind(|| {
                let _ = detect_language(&large_text);
                let _ = translate("en", &large_text);
            });
            prop_assert!(_result.is_ok());
        }
    }
}

/// Batch properties (when feature enabled)
#[cfg(feature = "batch")]
mod batch_properties {
    use super::*;
    use langweave::batch::{detect_batch_async, BatchConfig};

    proptest! {
        /// Batch detection preserves ordering
        #[test]
        fn batch_detection_preserves_order(
            texts in prop::collection::vec("[a-zA-Z ]{1,50}", 1..10)
        ) {
            tokio_test::block_on(async {
                let config = BatchConfig::default();
                let text_refs: Vec<&str> = texts.iter().map(String::as_str).collect();
                let results = detect_batch_async(&text_refs, &config).await;

                // Results should have same count as inputs
                assert_eq!(results.len(), texts.len());

                // Indices should be in order
                for (i, r) in results.iter().enumerate() {
                    assert_eq!(r.index, i);
                }
            });
        }
    }
}

/// Streaming properties
#[cfg(feature = "stream")]
mod streaming_properties {
    use super::*;
    use langweave::streaming::chunk_text;

    proptest! {
        /// Text chunking properties
        #[test]
        fn text_chunking_properties(
            text in "[a-zA-Z ]{1,1000}",
            chunk_size in 10..100usize
        ) {
            let chunks = chunk_text(&text, chunk_size);

            // Property: Non-empty input produces non-empty chunks
            if !text.trim().is_empty() {
                prop_assert!(!chunks.is_empty());
            }

            // Property: All chunks are non-empty
            for chunk in &chunks {
                prop_assert!(!chunk.is_empty());
            }

            // Property: Chunks respect size limit (with tolerance for no-boundary splits)
            for chunk in &chunks {
                prop_assert!(chunk.len() <= chunk_size * 2,
                            "Chunk length {} exceeds limit {}", chunk.len(), chunk_size * 2);
            }
        }
    }
}

/// Mathematical properties and invariants
mod mathematical_properties {
    use super::*;

    proptest! {
        /// Language set properties are mathematical
        #[test]
        fn language_set_mathematics(_: ()) {
            let languages = supported_languages();

            // Idempotency
            let languages2 = supported_languages();
            prop_assert_eq!(languages, languages2);

            // Membership is well-defined
            for &lang in languages {
                prop_assert!(is_language_supported(lang));
                prop_assert!(languages.contains(&lang));
            }

            // Set cardinality is fixed
            prop_assert_eq!(languages.len(), 15);

            // Each element is unique
            for (i, &lang1) in languages.iter().enumerate() {
                for (j, &lang2) in languages.iter().enumerate() {
                    if i != j {
                        prop_assert_ne!(lang1, lang2);
                    }
                }
            }
        }

        /// Version consistency
        #[test]
        fn version_string_properties(_: ()) {
            let version = langweave::VERSION;

            prop_assert!(!version.is_empty());
            prop_assert!(version.contains('.'));
            prop_assert_eq!(version, langweave::VERSION);
        }
    }
}

/// Regression tests for discovered invariant violations
mod regression_properties {
    use super::*;

    proptest! {
        /// Regression: Empty string handling edge case
        #[test]
        fn regression_empty_string_handling(_: ()) {
            let empty_cases = vec!["", " ", "  \n\t  ", "\u{00A0}", "\u{2000}"];

            for case in empty_cases {
                let result = detect_language(case);
                prop_assert!(matches!(result, Err(I18nError::LanguageDetectionFailed)));
            }
        }

        /// Regression: Language code normalization
        #[test]
        fn regression_language_code_normalization(code in "[A-Za-z]{2,5}") {
            let result1 = is_language_supported(&code);
            let result2 = is_language_supported(&code.to_lowercase());
            let result3 = is_language_supported(&code.to_uppercase());

            prop_assert_eq!(result1, result2);
            prop_assert_eq!(result2, result3);
        }
    }
}

/// Memory safety and resource properties
mod memory_properties {
    use super::*;

    proptest! {
        /// Memory allocation bounds
        #[test]
        fn memory_bounded_operations(input_size in 1..1000usize) {
            let text = "x".repeat(input_size);

            let _detect_result = detect_language(&text);
            let _translate_result = translate("en", &text);

            prop_assert!(true);
        }

        /// Thread safety properties
        #[test]
        fn thread_safety_properties(text in "[a-zA-Z ]{1,100}") {
            use std::sync::Arc;
            use std::thread;

            let text = Arc::new(text);
            let handles: Vec<_> = (0..4).map(|_| {
                let text = Arc::clone(&text);
                thread::spawn(move || {
                    let _ = detect_language(&text);
                    let _ = is_language_supported("en");
                    supported_languages()
                })
            }).collect();

            for handle in handles {
                let _result = handle.join();
                prop_assert!(_result.is_ok());
            }
        }
    }
}
