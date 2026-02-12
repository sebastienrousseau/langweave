//! Panic safety verification tests for LangWeave

use langweave::{
    detect_language, detect_language_async, error::I18nError,
    is_language_supported, supported_languages, translate,
};
use std::panic;

/// Panic safety tests for core functions
mod panic_safety {
    use super::*;

    #[test]
    fn detect_language_never_panics() {
        let dangerous_inputs: Vec<String> = vec![
            String::new(),
            "\0".to_string(),
            "\0\0\0\0\0".to_string(),
            "a".repeat(100_000),
            "\u{1F980}".repeat(10_000),
            "\u{FEFF}".repeat(1000),
            "\u{200B}".repeat(1000),
            "\u{0301}".repeat(1000),
            "\x00\x01\x02\x03\x04\x05".to_string(),
            "\u{FFFD}".repeat(1000),
            "\u{E000}".repeat(100),
            "\u{10FFFF}".to_string(),
        ];

        for input in &dangerous_inputs {
            let result = panic::catch_unwind(|| detect_language(input));
            assert!(
                result.is_ok(),
                "detect_language panicked on input len: {}",
                input.len()
            );
        }
    }

    #[tokio::test]
    async fn detect_language_async_never_panics() {
        let dangerous_inputs: Vec<String> = vec![
            String::new(),
            "\0".repeat(10),
            "\u{1F980}\u{1F480}\u{1F525}".repeat(1000),
            "a".repeat(50_000),
            "\u{200B}\u{FEFF}\u{061C}".repeat(1000),
        ];

        for input in &dangerous_inputs {
            let async_result = detect_language_async(input).await;
            let _ = async_result;
        }
    }

    #[test]
    fn translate_never_panics() {
        let dangerous_lang_codes: Vec<String> = vec![
            String::new(),
            "\0".to_string(),
            "\0\0".to_string(),
            "\u{1F980}".to_string(),
            "en\0".to_string(),
            "fr\u{1F480}".to_string(),
            "\u{FFFD}".to_string(),
            "\x00\x01".to_string(),
            "a".repeat(1000),
            "\u{E000}\u{E001}".to_string(),
        ];

        let dangerous_texts: Vec<String> = vec![
            String::new(),
            "\0".repeat(100),
            "\u{1F980}".repeat(5000),
            "\u{200B}".repeat(2000),
            "a".repeat(100_000),
            "\u{FFFD}".repeat(1000),
        ];

        for lang in &dangerous_lang_codes {
            for text in &dangerous_texts {
                let result =
                    panic::catch_unwind(|| translate(lang, text));
                assert!(
                    result.is_ok(),
                    "translate panicked on lang len: {}, text len: {}",
                    lang.len(),
                    text.len()
                );
            }
        }
    }

    #[test]
    fn is_language_supported_never_panics() {
        let dangerous_inputs: Vec<String> = vec![
            String::new(),
            "\0".to_string(),
            "\0\0\0".to_string(),
            "\u{1F980}\u{1F480}".to_string(),
            "\u{FFFD}".to_string(),
            "en\0fr".to_string(),
            "\x00\x01\x02".to_string(),
            "a".repeat(10_000),
            "\u{200B}\u{FEFF}".to_string(),
            "\u{E000}".to_string(),
            "\u{10FFFF}".to_string(),
            String::from_utf8_lossy(&[0xFF; 100]).to_string(),
        ];

        for input in &dangerous_inputs {
            let result =
                panic::catch_unwind(|| is_language_supported(input));
            assert!(
                result.is_ok(),
                "is_language_supported panicked on input len: {}",
                input.len()
            );
        }
    }

    #[test]
    fn supported_languages_never_panics() {
        for _ in 0..1000 {
            let result = panic::catch_unwind(supported_languages);
            assert!(result.is_ok(), "supported_languages panicked");
        }
    }

    #[cfg(feature = "batch")]
    #[tokio::test]
    async fn batch_operations_never_panic() {
        use langweave::batch::{
            detect_batch_async, translate_batch_async, BatchConfig,
        };

        let dangerous_text_sets: Vec<Vec<String>> = vec![
            vec![String::new()],
            vec!["\0".to_string(), "\0\0".to_string()],
            vec!["\u{1F980}".repeat(1000)],
            vec!["a".repeat(10_000), "b".repeat(10_000)],
            vec!["\u{FFFD}".repeat(100); 50],
        ];

        let config = BatchConfig::default();

        for texts in &dangerous_text_sets {
            let text_refs: Vec<&str> =
                texts.iter().map(String::as_str).collect();
            let _ = detect_batch_async(&text_refs, &config).await;
            let _ =
                translate_batch_async("en", &text_refs, &config).await;
        }
    }

    #[cfg(feature = "stream")]
    #[tokio::test]
    async fn streaming_operations_never_panic() {
        use langweave::streaming::{
            chunk_text, detect_language_stream, StreamConfig,
        };
        use tokio_stream::StreamExt;

        let dangerous_inputs: Vec<(String, usize)> = vec![
            (String::new(), 10),
            ("\0".repeat(100), 5),
            ("\u{1F980}".repeat(1000), 50),
            ("\u{FFFD}".repeat(500), 25),
            ("a".repeat(10_000), 100),
        ];

        for (text, chunk_size) in &dangerous_inputs {
            // Test chunk_text
            let result =
                panic::catch_unwind(|| chunk_text(text, *chunk_size));
            assert!(
                result.is_ok(),
                "chunk_text panicked on text len: {}, chunk_size: {}",
                text.len(),
                chunk_size
            );

            // Test detect_language_stream
            let config = StreamConfig {
                chunk_size: *chunk_size,
                buffer_size: 4,
            };
            let mut stream = detect_language_stream(text, &config);
            while let Some(_chunk) = stream.next().await {}
        }
    }
}

/// Memory safety tests
mod memory_safety {
    use super::*;

    #[test]
    fn large_input_handling() {
        let large_text = "a".repeat(1_000_000);

        let result =
            panic::catch_unwind(|| detect_language(&large_text));
        assert!(
            result.is_ok(),
            "detect_language panicked on 1MB input"
        );

        let result =
            panic::catch_unwind(|| translate("en", &large_text));
        assert!(result.is_ok(), "translate panicked on 1MB input");
    }

    #[test]
    fn concurrent_safety() {
        use std::sync::Arc;
        use std::thread;

        let text = Arc::new("Hello world test text".to_string());
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let text = Arc::clone(&text);
                thread::spawn(move || {
                    let _ = detect_language(&text);
                    let _ = translate("en", &text);
                    let _ = is_language_supported("fr");
                    supported_languages()
                })
            })
            .collect();

        for handle in handles {
            let result = handle.join();
            assert!(
                result.is_ok(),
                "Thread panicked during concurrent operations"
            );
        }
    }

    #[test]
    fn error_handling_robustness() {
        let error_cases = vec![
            I18nError::LanguageDetectionFailed,
            I18nError::TranslationFailed(
                "\0\u{FFFD}\u{1F980}".to_string(),
            ),
            I18nError::UnsupportedLanguage("\u{1F480}".repeat(1000)),
            I18nError::UnexpectedError("\x00\x01".to_string()),
            I18nError::TaskFailed("\u{E000}".to_string()),
            I18nError::BatchOperationFailed(String::new()),
            I18nError::StreamProcessingFailed("\u{10FFFF}".to_string()),
            I18nError::PatternOperationFailed("\u{200B}".repeat(1000)),
        ];

        for error in error_cases {
            let result = panic::catch_unwind(|| {
                let _ = error.clone();
                let _ = format!("{error}");
                let _ = format!("{error:?}");
                let _ = error.as_str();
                error
            });
            assert!(result.is_ok(), "Error handling panicked");
        }
    }
}
