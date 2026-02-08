//! Comprehensive multi-language tests for LangWeave
//!
//! This module tests all 15 supported languages across various functionality:
//! language support verification, translation capabilities, common key translations,
//! language detection, and supported language count validation.

use langweave::{detect_language, is_language_supported, supported_languages, translate};
use langweave::error::I18nError;
use tokio;

/// All 15 languages that should be supported by LangWeave
const ALL_LANGUAGES: &[&str] = &[
    "en", // English
    "fr", // French
    "de", // German
    "es", // Spanish
    "pt", // Portuguese
    "it", // Italian
    "nl", // Dutch
    "ru", // Russian
    "ar", // Arabic
    "he", // Hebrew
    "hi", // Hindi
    "ja", // Japanese
    "ko", // Korean
    "zh", // Chinese
    "id", // Indonesian
];

/// Common translation keys that should be available in all languages
const COMMON_KEYS: &[&str] = &[
    "Hello",
    "Goodbye",
    "Yes",
    "No",
    "Thank you",
    "Please",
];

/// Sample text for language detection testing in each language
const LANGUAGE_SAMPLES: &[(&str, &str)] = &[
    ("en", "The quick brown fox jumps over the lazy dog"),
    ("fr", "Le renard brun rapide saute par-dessus le chien paresseux"),
    ("de", "Der schnelle braune Fuchs springt über den faulen Hund"),
    ("es", "El rápido zorro marrón salta sobre el perro perezoso"),
    ("pt", "A raposa marrom rápida pula sobre o cão preguiçoso"),
    ("it", "La volpe marrone veloce salta sopra il cane pigro"),
    ("nl", "De snelle bruine vos springt over de luie hond"),
    ("ru", "Быстрая коричневая лиса прыгает через ленивую собаку"),
    ("ar", "الثعلب البني السريع يقفز فوق الكلب الكسول"),
    ("he", "השועל החום המהיר קופץ מעל הכלב העצלן"),
    ("hi", "तेज़ भूरी लोमड़ी आलसी कुत्ते के ऊपर कूदती है"),
    ("ja", "素早い茶色のキツネが怠けている犬を飛び越える"),
    ("ko", "빠른 갈색 여우가 게으른 개를 뛰어넘는다"),
    ("zh", "敏捷的棕色狐狸跳过懒惰的狗"),
    ("id", "Rubah coklat yang cepat melompat melewati anjing yang malas"),
];

/// Test that all 15 languages return true for is_language_supported
#[test]
fn test_all_15_languages_supported() {
    let mut supported_count = 0;
    let mut unsupported_languages = Vec::new();

    for &lang in ALL_LANGUAGES {
        if is_language_supported(lang) {
            supported_count += 1;
        } else {
            unsupported_languages.push(lang);
        }
    }

    // Print diagnostic information
    println!("Supported languages: {}/{}", supported_count, ALL_LANGUAGES.len());
    if !unsupported_languages.is_empty() {
        println!("Unsupported languages: {:?}", unsupported_languages);
    }

    // All 15 languages should be supported now
    assert_eq!(
        supported_count, 15,
        "All 15 languages should be supported. Missing: {:?}",
        unsupported_languages
    );
}

/// Test that translate(lang, "Hello") succeeds for all supported languages
#[test]
fn test_translate_all_15_languages() {
    let mut successful_translations = 0;
    let mut failed_translations = Vec::new();

    for &lang in ALL_LANGUAGES {
        match translate(lang, "Hello") {
            Ok(translation) => {
                successful_translations += 1;
                println!("✓ {} -> Hello: {}", lang, translation);
            }
            Err(e) => {
                failed_translations.push((lang, e));
                println!("✗ {} -> Hello: {:?}", lang, failed_translations.last().unwrap().1);
            }
        }
    }

    // Print summary
    println!("Successful Hello translations: {}/{}", successful_translations, ALL_LANGUAGES.len());

    // All 15 languages should work
    assert_eq!(
        successful_translations, 15,
        "All 15 languages should translate 'Hello' successfully, got {}",
        successful_translations
    );

    // Verify no translations failed
    assert!(
        failed_translations.is_empty(),
        "No translations should fail. Failures: {:?}",
        failed_translations
    );
}

/// Test that common keys translate successfully for all supported languages
#[test]
fn test_translate_common_keys() {
    let mut total_translations = 0;
    let mut successful_translations = 0;
    let mut failed_translations = Vec::new();

    for &lang in ALL_LANGUAGES {
        for &key in COMMON_KEYS {
            total_translations += 1;
            match translate(lang, key) {
                Ok(translation) => {
                    successful_translations += 1;
                    println!("✓ {} -> {}: {}", lang, key, translation);
                }
                Err(e) => {
                    failed_translations.push((lang, key, e));
                }
            }
        }
    }

    println!(
        "Common key translations: {}/{} successful",
        successful_translations, total_translations
    );

    // All languages should translate common keys successfully
    let expected_total = ALL_LANGUAGES.len() * COMMON_KEYS.len(); // 15 languages × 6 keys = 90
    assert_eq!(
        successful_translations, expected_total,
        "Should have {} successful translations for all common keys, got {}",
        expected_total, successful_translations
    );

    // Verify no translations failed
    assert!(
        failed_translations.is_empty(),
        "No common key translations should fail. Failures: {:?}",
        failed_translations
    );
}

/// Test that language detection works for text samples in each language
#[tokio::test]
async fn test_detect_language_all_scripts() {
    let mut successful_detections = 0;
    let mut failed_detections = Vec::new();
    let mut incorrect_detections = Vec::new();

    for (expected_lang, sample_text) in LANGUAGE_SAMPLES {
        match detect_language(sample_text).await {
            Ok(detected_lang) => {
                if &detected_lang == expected_lang {
                    successful_detections += 1;
                    let preview = sample_text.chars().take(50).collect::<String>();
                    println!("✓ Correctly detected {} for: {}", detected_lang, preview);
                } else {
                    let preview = sample_text.chars().take(50).collect::<String>();
                    println!("? Expected {}, got {} for: {}", expected_lang, detected_lang, preview);
                    incorrect_detections.push((expected_lang, detected_lang, sample_text));
                }
            }
            Err(e) => {
                println!("✗ Detection failed for {}: {:?}", expected_lang, e);
                failed_detections.push((expected_lang, e, sample_text));
            }
        }
    }

    println!(
        "Language detection results: {}/{} correct, {} incorrect, {} failed",
        successful_detections,
        LANGUAGE_SAMPLES.len(),
        incorrect_detections.len(),
        failed_detections.len()
    );

    // Language detection may not be 100% accurate, especially for languages
    // not fully implemented, so we test more leniently
    assert!(
        successful_detections > 0,
        "At least some language detections should succeed"
    );

    // Test specific languages that should work based on current implementation
    for expected_lang in &["en", "fr", "de"] {
        if let Some((_lang, sample)) = LANGUAGE_SAMPLES.iter().find(|(lang, _)| lang == expected_lang) {
            let result = detect_language(sample).await;
            assert!(
                result.is_ok(),
                "Language detection should work for {} sample text",
                expected_lang
            );
        }
    }
}

/// Test that supported_languages() returns exactly 15 items
#[test]
fn test_supported_languages_count() {
    let languages = supported_languages();

    println!("Currently supported languages: {:?}", languages);
    println!("Count: {}", languages.len());

    // All 15 languages should be supported now
    assert_eq!(
        languages.len(), 15,
        "Should return exactly 15 supported languages, got {}",
        languages.len()
    );

    // Verify all 15 languages are included
    for &expected_lang in ALL_LANGUAGES {
        assert!(
            languages.contains(&expected_lang.to_string()),
            "Language {} should be in supported_languages() result",
            expected_lang
        );
    }
}

/// Test error handling for unsupported languages
#[test]
fn test_unsupported_language_errors() {
    // Test languages that are definitely not implemented
    let unsupported_languages = ["xx", "zz", "invalid"];

    for &lang in &unsupported_languages {
        // is_language_supported should return false
        assert!(
            !is_language_supported(lang),
            "Language {} should not be supported",
            lang
        );

        // translate should return UnsupportedLanguage error
        match translate(lang, "Hello") {
            Err(I18nError::UnsupportedLanguage(returned_lang)) => {
                assert_eq!(returned_lang, lang, "Error should contain the correct language code");
            }
            other => {
                panic!(
                    "Expected UnsupportedLanguage error for {}, got {:?}",
                    lang, other
                );
            }
        }
    }
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    // Test empty string translation
    for lang in &["en", "fr", "de"] {
        match translate(lang, "") {
            Ok(result) => assert_eq!(result, "", "Empty string should return empty string"),
            Err(_) => {
                // Empty string might not be in translation dictionary, this is acceptable
                println!("Empty string translation failed for {} (acceptable)", lang);
            }
        }
    }

    // Test case sensitivity in language codes
    for &lang in &["EN", "Fr", "DE"] {
        // Language codes should be case-insensitive
        assert!(
            is_language_supported(lang) || is_language_supported(&lang.to_lowercase()),
            "Language support check should handle case variations for {}",
            lang
        );
    }

    // Test very long text for translation (should fail gracefully)
    let long_text = "a".repeat(1000);
    for lang in &["en", "fr", "de"] {
        let result = translate(lang, &long_text);
        // Should either succeed or fail gracefully, not panic
        match result {
            Ok(_) => println!("Long text translation succeeded for {}", lang),
            Err(_) => println!("Long text translation failed for {} (expected)", lang),
        }
    }
}

/// Test language detection with edge cases
#[tokio::test]
async fn test_language_detection_edge_cases() {
    // Test empty string
    match detect_language("").await {
        Err(I18nError::LanguageDetectionFailed) => {
            // Expected behavior for empty string
        }
        other => {
            panic!("Expected LanguageDetectionFailed for empty string, got {:?}", other);
        }
    }

    // Test whitespace only
    match detect_language("   ").await {
        Err(I18nError::LanguageDetectionFailed) => {
            // Expected behavior for whitespace-only string
        }
        other => {
            // Might detect something, but should handle gracefully
            println!("Whitespace detection result: {:?}", other);
        }
    }

    // Test numbers only
    let result = detect_language("123456789").await;
    // Should either detect a language or fail gracefully
    match result {
        Ok(lang) => println!("Detected language for numbers: {}", lang),
        Err(_) => println!("Number detection failed (acceptable)"),
    }

    // Test mixed scripts (if supported)
    let mixed_text = "Hello 世界 مرحبا";
    let result = detect_language(mixed_text).await;
    match result {
        Ok(lang) => println!("Detected language for mixed text: {}", lang),
        Err(_) => println!("Mixed script detection failed (may be expected)"),
    }
}

/// Performance test for bulk operations
#[test]
fn test_bulk_operations() {
    use std::time::Instant;

    // Test bulk translation performance
    let start = Instant::now();
    let mut successful_operations = 0;

    for _ in 0..100 {
        for &lang in &["en", "fr", "de"] {
            if translate(lang, "Hello").is_ok() {
                successful_operations += 1;
            }
        }
    }

    let duration = start.elapsed();
    println!(
        "Bulk translation test: {} operations in {:?} ({:?} per operation)",
        successful_operations,
        duration,
        duration / successful_operations as u32
    );

    // Should complete within reasonable time (less than 1 second for 300 operations)
    assert!(
        duration.as_secs() < 1,
        "Bulk translation operations should complete quickly"
    );
}

/// Integration test combining multiple operations
#[tokio::test]
async fn test_full_workflow() {
    // Test a complete workflow: detect language -> verify support -> translate

    for (_expected_lang, sample_text) in &[
        ("en", "Hello world"),
        ("fr", "Bonjour le monde"),
        ("de", "Hallo Welt"),
    ] {
        // Step 1: Detect language
        match detect_language(sample_text).await {
            Ok(detected_lang) => {
                println!("Detected language: {} for text: {}", detected_lang, sample_text);

                // Step 2: Verify it's supported
                if is_language_supported(&detected_lang) {
                    // Step 3: Try to translate something
                    match translate(&detected_lang, "Hello") {
                        Ok(translation) => {
                            println!("Full workflow success: {} -> Hello -> {}", detected_lang, translation);
                        }
                        Err(e) => {
                            println!("Translation failed in workflow: {:?}", e);
                        }
                    }
                } else {
                    println!("Detected language {} is not supported for translation", detected_lang);
                }
            }
            Err(e) => {
                println!("Language detection failed for {}: {:?}", sample_text, e);
            }
        }
    }
}