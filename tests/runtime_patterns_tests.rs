// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for runtime pattern extensibility.

use langweave::error::I18nError;
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;

#[test]
fn test_custom_pattern_basic() {
    let detector = LanguageDetector::new();
    let custom =
        detector.with_custom_pattern(r"(?i)\b(aloha)\b", "haw").ok();
    assert!(custom.is_some());
    let custom = custom.unwrap();
    assert_eq!(
        custom.detect("Aloha world").ok(),
        Some("haw".to_string())
    );
}

#[test]
fn test_custom_pattern_count() {
    let detector = LanguageDetector::new();
    assert_eq!(detector.custom_pattern_count(), 0);

    let d1 = detector
        .with_custom_pattern(r"(?i)\baloha\b", "haw")
        .unwrap();
    assert_eq!(d1.custom_pattern_count(), 1);

    let d2 = d1.with_custom_pattern(r"(?i)\bmabuhay\b", "tl").unwrap();
    assert_eq!(d2.custom_pattern_count(), 2);
}

#[test]
fn test_custom_pattern_priority_over_builtin() {
    let detector = LanguageDetector::new();
    // "Hello" normally detects as English
    assert_eq!(detector.detect("Hello").ok(), Some("en".to_string()));

    // Override: make "Hello" match a custom language
    let custom = detector
        .with_custom_pattern(r"(?i)\bhello\b", "xx")
        .unwrap();
    assert_eq!(custom.detect("Hello").ok(), Some("xx".to_string()));
}

#[test]
fn test_custom_pattern_invalid_regex() {
    let detector = LanguageDetector::new();
    let result = detector.with_custom_pattern("[invalid", "xx");
    assert!(matches!(
        result,
        Err(I18nError::PatternOperationFailed(_))
    ));
}

#[test]
fn test_original_detector_unchanged() {
    let detector = LanguageDetector::new();
    let _custom = detector
        .with_custom_pattern(r"(?i)\baloha\b", "haw")
        .unwrap();

    // Original detector should not have the custom pattern
    assert_eq!(detector.custom_pattern_count(), 0);
    // "Aloha" should not match anything in the original detector
    // (it falls through to whatlang or fails)
    let result = detector.detect("Aloha");
    // It should not return "haw"
    assert_ne!(result.ok(), Some("haw".to_string()));
}

#[test]
fn test_custom_pattern_with_try_new() {
    let detector = LanguageDetector::try_new().unwrap();
    let custom = detector
        .with_custom_pattern(r"(?i)\bsawubona\b", "zu")
        .unwrap();
    assert_eq!(custom.detect("Sawubona").ok(), Some("zu".to_string()));
}

#[tokio::test]
async fn test_custom_pattern_async() {
    let detector = LanguageDetector::new();
    let custom = detector
        .with_custom_pattern(r"(?i)\baloha\b", "haw")
        .unwrap();
    let result = custom.detect_async("Aloha world").await;
    assert_eq!(result.ok(), Some("haw".to_string()));
}

#[test]
fn test_multiple_custom_patterns() {
    let detector = LanguageDetector::new()
        .with_custom_pattern(r"(?i)\baloha\b", "haw")
        .unwrap()
        .with_custom_pattern(r"(?i)\bsawubona\b", "zu")
        .unwrap();

    assert_eq!(detector.detect("Aloha").ok(), Some("haw".to_string()));
    assert_eq!(
        detector.detect("Sawubona").ok(),
        Some("zu".to_string())
    );
}

#[test]
fn test_custom_pattern_does_not_affect_builtin_detection() {
    let detector = LanguageDetector::new()
        .with_custom_pattern(r"(?i)\baloha\b", "haw")
        .unwrap();

    // Built-in detection should still work for other languages
    assert_eq!(
        detector.detect("Bonjour le monde").ok(),
        Some("fr".to_string())
    );
    assert_eq!(
        detector.detect("こんにちは").ok(),
        Some("ja".to_string())
    );
}
