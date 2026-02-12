// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests verifying true async behavior via spawn_blocking.

use langweave::detect_language_async;
use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use std::sync::Arc;

#[tokio::test]
async fn test_detect_async_returns_correct_language() {
    let detector = LanguageDetector::new();
    let result = detector.detect_async("Hello world").await;
    assert_eq!(result.ok(), Some("en".to_string()));
}

#[tokio::test]
async fn test_detect_async_french() {
    let detector = LanguageDetector::new();
    let result = detector.detect_async("Bonjour le monde").await;
    assert_eq!(result.ok(), Some("fr".to_string()));
}

#[tokio::test]
async fn test_detect_async_empty_input() {
    let detector = LanguageDetector::new();
    let result = detector.detect_async("").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_detect_async_non_alphabetic() {
    let detector = LanguageDetector::new();
    let result = detector.detect_async("12345 @#$%").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_detect_async() {
    let detector = Arc::new(LanguageDetector::new());
    let mut handles = Vec::new();

    for _ in 0..50 {
        let d = Arc::clone(&detector);
        handles.push(tokio::spawn(async move {
            d.detect_async("Hello world").await
        }));
    }

    for handle in handles {
        let result = handle.await.ok().and_then(|r| r.ok());
        assert_eq!(result, Some("en".to_string()));
    }
}

#[tokio::test]
async fn test_detect_language_async_function() {
    let result = detect_language_async("Hello world").await;
    assert_eq!(result.ok(), Some("en".to_string()));
}

#[tokio::test]
async fn test_detect_language_async_empty() {
    let result = detect_language_async("").await;
    assert!(matches!(result, Err(I18nError::LanguageDetectionFailed)));
}

#[tokio::test]
async fn test_detect_language_async_whitespace_only() {
    let result = detect_language_async("   ").await;
    assert!(matches!(result, Err(I18nError::LanguageDetectionFailed)));
}

#[cfg(feature = "async")]
mod async_feature_tests {
    use langweave::async_utils::translate_async;
    use langweave::error::I18nError;

    #[tokio::test]
    async fn test_translate_async_basic() {
        let result = translate_async("fr", "Hello").await;
        assert_eq!(result.ok(), Some("Bonjour".to_string()));
    }

    #[tokio::test]
    async fn test_translate_async_unsupported_language() {
        let result = translate_async("zz", "Hello").await;
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[tokio::test]
    async fn test_translate_async_concurrent() {
        let mut handles = Vec::new();
        for _ in 0..20 {
            handles.push(tokio::spawn(async {
                translate_async("fr", "Hello").await
            }));
        }

        for handle in handles {
            let result = handle.await.ok().and_then(|r| r.ok());
            assert_eq!(result, Some("Bonjour".to_string()));
        }
    }
}
