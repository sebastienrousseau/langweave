// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Targeted tests to cover remaining uncovered production code paths.

/// Tests for the streaming module break paths (channel closed before all chunks sent).
#[cfg(feature = "stream")]
mod streaming_coverage {
    use langweave::streaming::{
        detect_language_stream, translate_stream, StreamConfig,
    };
    use tokio_stream::StreamExt;

    #[tokio::test]
    async fn test_detect_stream_drop_early() {
        // Use a very small chunk_size to produce many chunks
        let config = StreamConfig {
            chunk_size: 5,
            buffer_size: 2,
        };
        let text =
            "Hello world this is a test with many words for streaming";
        let mut stream = detect_language_stream(text, &config);
        // Consume only the first chunk, then drop the stream
        let first = stream.next().await;
        assert!(first.is_some());
        assert_eq!(first.as_ref().map(|c| c.chunk_index), Some(0));
        // Drop the stream, which closes the receiver channel.
        // The spawned producer task will see a send error and break.
        drop(stream);
        // Give the producer task a moment to hit the break
        tokio::time::sleep(tokio::time::Duration::from_millis(50))
            .await;
    }

    #[tokio::test]
    async fn test_translate_stream_drop_early() {
        let config = StreamConfig {
            chunk_size: 5,
            buffer_size: 2,
        };
        let text = "Hello Goodbye Yes No Thank you Please";
        let mut stream = translate_stream("fr", text, &config);
        // Consume only the first chunk
        let first = stream.next().await;
        assert!(first.is_some());
        drop(stream);
        tokio::time::sleep(tokio::time::Duration::from_millis(50))
            .await;
    }

    #[tokio::test]
    async fn test_detect_stream_multi_chunk_all_consumed() {
        let config = StreamConfig {
            chunk_size: 10,
            buffer_size: 4,
        };
        let text = "Hello world Bonjour le monde Hola mundo";
        let mut stream = detect_language_stream(text, &config);
        let mut results = Vec::new();
        while let Some(chunk) = stream.next().await {
            results.push(chunk);
        }
        assert!(results.len() >= 3);
        for (i, r) in results.iter().enumerate() {
            assert_eq!(r.chunk_index, i);
            assert!(!r.chunk_text.is_empty());
        }
    }

    #[tokio::test]
    async fn test_translate_stream_multi_chunk() {
        let config = StreamConfig {
            chunk_size: 8,
            buffer_size: 4,
        };
        let text = "Hello Goodbye";
        let mut stream = translate_stream("fr", text, &config);
        let mut results = Vec::new();
        while let Some(chunk) = stream.next().await {
            results.push(chunk);
        }
        assert!(!results.is_empty());
    }
}

/// Tests for translations::translate_with_fallback UnsupportedLanguage path.
mod translations_coverage {
    use langweave::error::I18nError;
    use langweave::translations::translate_with_fallback;

    #[test]
    fn test_translate_with_fallback_unsupported_language() {
        // This hits the UnsupportedLanguage match arm in translate_with_fallback
        let result = translate_with_fallback("zz", "Hello");
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[test]
    fn test_translate_with_fallback_unsupported_language_message() {
        let result = translate_with_fallback("xyz", "anything");
        match result {
            Err(I18nError::UnsupportedLanguage(lang)) => {
                assert_eq!(lang, "xyz");
            }
            other => {
                panic!("Expected UnsupportedLanguage, got {:?}", other)
            }
        }
    }

    #[test]
    fn test_translate_with_fallback_complex_phrase() {
        // Complex phrases (with spaces) should return TranslationFailed
        let result =
            translate_with_fallback("en", "nonexistent phrase here");
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    #[test]
    fn test_translate_with_fallback_simple_key_fallback() {
        // Simple unknown key (no spaces/punctuation) falls back to original text
        let result = translate_with_fallback("en", "unknownkey");
        assert_eq!(result.ok(), Some("unknownkey".to_string()));
    }

    #[test]
    fn test_translate_with_fallback_success() {
        let result = translate_with_fallback("fr", "Hello");
        assert_eq!(result.ok(), Some("Bonjour".to_string()));
    }
}

/// Tests for batch error handling paths.
#[cfg(feature = "batch")]
mod batch_coverage {
    use langweave::batch::{
        detect_batch_async, translate_batch_async, BatchConfig,
    };

    #[tokio::test]
    async fn test_detect_batch_with_invalid_inputs() {
        let texts = vec!["", "   ", "12345", "!!!", "Hello"];
        let config = BatchConfig { max_concurrency: 2 };
        let results = detect_batch_async(&texts, &config).await;
        assert_eq!(results.len(), 5);
        // First 4 should fail, last should succeed
        assert!(results[0].result.is_err());
        assert!(results[1].result.is_err());
        assert!(results[2].result.is_err());
        assert!(results[3].result.is_err());
        assert!(results[4].result.is_ok());
    }

    #[tokio::test]
    async fn test_translate_batch_with_unsupported_language() {
        let texts = vec!["Hello", "Goodbye"];
        let config = BatchConfig::default();
        let results =
            translate_batch_async("zz", &texts, &config).await;
        assert_eq!(results.len(), 2);
        // Both should fail with unsupported language
        assert!(results[0].result.is_err());
        assert!(results[1].result.is_err());
    }

    #[tokio::test]
    async fn test_detect_batch_high_concurrency() {
        let texts: Vec<&str> = (0..50).map(|_| "Hello world").collect();
        let config = BatchConfig {
            max_concurrency: 50,
        };
        let results = detect_batch_async(&texts, &config).await;
        assert_eq!(results.len(), 50);
        for result in &results {
            assert_eq!(
                result.result.as_ref().ok(),
                Some(&"en".to_string())
            );
        }
    }

    #[tokio::test]
    async fn test_translate_batch_mixed_results() {
        // "Hello" and "Goodbye" are in the dictionary, "xyzzy" is not
        let texts = vec!["Hello", "Goodbye", "xyzzy"];
        let config = BatchConfig::default();
        let results =
            translate_batch_async("fr", &texts, &config).await;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].index, 0);
        assert_eq!(results[1].index, 1);
        assert_eq!(results[2].index, 2);
        // "Hello" -> "Bonjour", "Goodbye" -> "Au revoir", "xyzzy" -> fallback
        assert!(results[0].result.is_ok());
        assert!(results[1].result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_config_debug() {
        let config = BatchConfig { max_concurrency: 3 };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("3"));
    }
}

/// Tests for language_detector custom patterns edge cases.
mod language_detector_coverage {
    use langweave::error::I18nError;
    use langweave::language_detector::LanguageDetector;
    use langweave::language_detector_trait::LanguageDetectorTrait;

    #[test]
    fn test_custom_pattern_debug_format() {
        let detector = LanguageDetector::new()
            .with_custom_pattern(r"(?i)\baloha\b", "haw")
            .unwrap();
        let debug_str = format!("{:?}", detector);
        assert!(debug_str.contains("LanguageDetector"));
    }

    #[test]
    fn test_custom_pattern_clone() {
        let detector = LanguageDetector::new()
            .with_custom_pattern(r"(?i)\baloha\b", "haw")
            .unwrap();
        let cloned = detector.clone();
        assert_eq!(cloned.custom_pattern_count(), 1);
        assert_eq!(
            cloned.detect("Aloha").ok(),
            Some("haw".to_string())
        );
    }

    #[test]
    fn test_custom_pattern_various_invalid_regex() {
        let detector = LanguageDetector::new();

        // Various invalid regex patterns
        let result = detector.with_custom_pattern("(unclosed", "xx");
        assert!(matches!(
            result,
            Err(I18nError::PatternOperationFailed(_))
        ));

        let result = detector.with_custom_pattern("[z-a]", "xx");
        assert!(matches!(
            result,
            Err(I18nError::PatternOperationFailed(_))
        ));

        let result =
            detector.with_custom_pattern("(?P<dup>a)(?P<dup>b)", "xx");
        assert!(matches!(
            result,
            Err(I18nError::PatternOperationFailed(_))
        ));
    }

    #[tokio::test]
    async fn test_detect_async_multiple_languages() {
        let detector = LanguageDetector::new();
        let tests = vec![
            ("Hello world", "en"),
            ("Bonjour le monde", "fr"),
            ("Hallo Welt", "de"),
            ("Hola mundo", "es"),
            ("こんにちは", "ja"),
            ("你好", "zh"),
            ("مرحبا", "ar"),
            ("नमस्ते", "hi"),
            ("안녕하세요", "ko"),
        ];
        for (text, expected) in tests {
            let result = detector.detect_async(text).await;
            assert_eq!(
                result.ok(),
                Some(expected.to_string()),
                "Failed for: {}",
                text
            );
        }
    }
}

/// Tests for lib.rs async and sync edge cases.
mod lib_coverage {
    use langweave::error::I18nError;
    use langweave::{
        detect_language, detect_language_async, translate,
    };

    #[test]
    fn test_translate_with_all_supported_languages() {
        let languages = langweave::supported_languages();
        for lang in languages {
            let result = translate(lang, "Hello");
            assert!(
                result.is_ok(),
                "translate should succeed for '{}': {:?}",
                lang,
                result
            );
        }
    }

    #[test]
    fn test_detect_language_various_scripts() {
        // Test multiple languages via the top-level detect_language function
        let tests = vec![
            "The quick brown fox",
            "Le chat noir",
            "Der schnelle Fuchs",
            "El gato rápido",
            "こんにちは世界",
            "你好世界",
            "مرحبا بالعالم",
            "Здравствуйте мир",
            "שלום עולם",
        ];
        for text in tests {
            let result = detect_language(text);
            assert!(
                result.is_ok(),
                "detect_language failed for: {}",
                text
            );
        }
    }

    #[tokio::test]
    async fn test_detect_language_async_various() {
        let tests = vec![
            ("Hello world", "en"),
            ("Bonjour le monde", "fr"),
            ("Hallo Welt", "de"),
        ];
        for (text, expected) in tests {
            let result = detect_language_async(text).await;
            assert_eq!(
                result.ok(),
                Some(expected.to_string()),
                "Failed for: {}",
                text
            );
        }
    }

    #[tokio::test]
    async fn test_detect_language_async_concurrent() {
        let mut handles = Vec::new();
        for _ in 0..30 {
            handles.push(tokio::spawn(async {
                detect_language_async("Hello world").await
            }));
        }
        for handle in handles {
            let result = handle.await.ok().and_then(|r| r.ok());
            assert_eq!(result, Some("en".to_string()));
        }
    }

    #[test]
    fn test_detect_language_numeric_only() {
        let result = detect_language("12345 67890");
        assert!(matches!(
            result,
            Err(I18nError::LanguageDetectionFailed)
        ));
    }

    #[test]
    fn test_detect_language_special_chars_with_word() {
        // Text with special chars but also a word that can be detected
        let result = detect_language("### Hello ###");
        assert!(result.is_ok());
    }
}

/// Tests for error variants additional coverage.
mod error_coverage {
    use langweave::error::I18nError;

    #[test]
    fn test_new_error_variant_equality() {
        assert_ne!(
            I18nError::TaskFailed("a".to_string()),
            I18nError::BatchOperationFailed("a".to_string())
        );
        assert_ne!(
            I18nError::StreamProcessingFailed("a".to_string()),
            I18nError::PatternOperationFailed("a".to_string())
        );
    }

    #[test]
    fn test_new_error_variant_clone() {
        let errors = vec![
            I18nError::TaskFailed("test".to_string()),
            I18nError::BatchOperationFailed("test".to_string()),
            I18nError::StreamProcessingFailed("test".to_string()),
            I18nError::PatternOperationFailed("test".to_string()),
        ];
        for error in errors {
            let cloned = error.clone();
            assert_eq!(error, cloned);
        }
    }

    #[test]
    fn test_new_error_variant_debug() {
        assert_eq!(
            format!("{:?}", I18nError::TaskFailed("x".to_string())),
            "TaskFailed(\"x\")"
        );
        assert_eq!(
            format!(
                "{:?}",
                I18nError::BatchOperationFailed("x".to_string())
            ),
            "BatchOperationFailed(\"x\")"
        );
        assert_eq!(
            format!(
                "{:?}",
                I18nError::StreamProcessingFailed("x".to_string())
            ),
            "StreamProcessingFailed(\"x\")"
        );
        assert_eq!(
            format!(
                "{:?}",
                I18nError::PatternOperationFailed("x".to_string())
            ),
            "PatternOperationFailed(\"x\")"
        );
    }
}

/// Tests for async_utils module.
#[cfg(feature = "async")]
mod async_utils_coverage {
    use langweave::async_utils::translate_async;
    use langweave::error::I18nError;

    #[tokio::test]
    async fn test_translate_async_all_languages() {
        let languages = langweave::supported_languages();
        for lang in languages {
            let result = translate_async(lang, "Hello").await;
            assert!(
                result.is_ok(),
                "translate_async should succeed for '{}': {:?}",
                lang,
                result
            );
        }
    }

    #[tokio::test]
    async fn test_translate_async_missing_key() {
        // translate_async uses Translator which calls translations::translate
        // directly (no fallback), so a missing key returns TranslationFailed
        let result = translate_async("fr", "NonexistentKey99").await;
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }

    #[tokio::test]
    async fn test_translate_async_complex_phrase() {
        let result = translate_async("fr", "How are you?").await;
        assert!(matches!(result, Err(I18nError::TranslationFailed(_))));
    }
}
