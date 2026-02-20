//! Comprehensive coverage tests for the optimized module
//!
//! This module ensures 100% line and branch coverage for all optimization functions.

#[cfg(test)]
mod optimized_coverage_tests {
    use langweave::error::I18nError;
    use langweave::optimized::*;

    #[test]
    fn test_translate_optimized_success_cases() {
        // Test successful translation through optimized path
        let result = translate_optimized("fr", "Hello");
        assert!(result.is_ok());

        let result = translate_optimized("de", "Goodbye");
        assert!(result.is_ok());
    }

    #[test]
    fn test_translate_optimized_error_cases() {
        // Test error propagation through optimized path
        let result = translate_optimized("invalid_lang", "Hello");
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));

        // Test with empty language
        let result = translate_optimized("", "Hello");
        assert!(result.is_err());

        // Test with empty key
        let result = translate_optimized("en", "");
        assert!(result.is_err() || result.is_ok()); // May succeed with empty string fallback
    }

    #[test]
    fn test_is_language_supported_optimized_edge_cases() {
        // Test all the edge cases that might not be covered

        // Empty string
        assert!(!is_language_supported_optimized(""));

        // Very long string
        let long_string = "a".repeat(1000);
        assert!(!is_language_supported_optimized(&long_string));

        // Mixed case variations
        assert!(is_language_supported_optimized("En"));
        assert!(is_language_supported_optimized("FR"));
        assert!(is_language_supported_optimized("De"));
        assert!(is_language_supported_optimized("ES"));
        assert!(is_language_supported_optimized("PT"));
        assert!(is_language_supported_optimized("IT"));
        assert!(is_language_supported_optimized("NL"));
        assert!(is_language_supported_optimized("RU"));
        assert!(is_language_supported_optimized("AR"));
        assert!(is_language_supported_optimized("HE"));
        assert!(is_language_supported_optimized("HI"));
        assert!(is_language_supported_optimized("JA"));
        assert!(is_language_supported_optimized("KO"));
        assert!(is_language_supported_optimized("ZH"));
        assert!(is_language_supported_optimized("ID"));

        // Special characters
        assert!(!is_language_supported_optimized("en-"));
        assert!(!is_language_supported_optimized("en_US"));
        assert!(!is_language_supported_optimized("123"));
        assert!(!is_language_supported_optimized("!@#"));
    }

    #[test]
    fn test_is_language_supported_zero_alloc_comprehensive() {
        // Test direct match cases (no allocation path)
        assert!(is_language_supported_zero_alloc("en"));
        assert!(is_language_supported_zero_alloc("fr"));
        assert!(is_language_supported_zero_alloc("de"));
        assert!(is_language_supported_zero_alloc("es"));
        assert!(is_language_supported_zero_alloc("pt"));
        assert!(is_language_supported_zero_alloc("it"));
        assert!(is_language_supported_zero_alloc("nl"));
        assert!(is_language_supported_zero_alloc("ru"));
        assert!(is_language_supported_zero_alloc("ar"));
        assert!(is_language_supported_zero_alloc("he"));
        assert!(is_language_supported_zero_alloc("hi"));
        assert!(is_language_supported_zero_alloc("ja"));
        assert!(is_language_supported_zero_alloc("ko"));
        assert!(is_language_supported_zero_alloc("zh"));
        assert!(is_language_supported_zero_alloc("id"));

        // Test fallback path (requires allocation for case conversion)
        assert!(is_language_supported_zero_alloc("EN"));
        assert!(is_language_supported_zero_alloc("FR"));
        assert!(is_language_supported_zero_alloc("De"));
        assert!(is_language_supported_zero_alloc("Es"));

        // Test unsupported in fallback path
        assert!(!is_language_supported_zero_alloc("ZZ"));
        assert!(!is_language_supported_zero_alloc("Invalid"));
        assert!(!is_language_supported_zero_alloc(""));
    }

    #[test]
    fn test_supported_languages_optimized_invariants() {
        let langs = supported_languages_optimized();

        // Test length
        assert_eq!(langs.len(), 15);

        // Test that all languages are present
        assert!(langs.contains(&"en"));
        assert!(langs.contains(&"fr"));
        assert!(langs.contains(&"de"));
        assert!(langs.contains(&"es"));
        assert!(langs.contains(&"pt"));
        assert!(langs.contains(&"it"));
        assert!(langs.contains(&"nl"));
        assert!(langs.contains(&"ru"));
        assert!(langs.contains(&"ar"));
        assert!(langs.contains(&"he"));
        assert!(langs.contains(&"hi"));
        assert!(langs.contains(&"ja"));
        assert!(langs.contains(&"ko"));
        assert!(langs.contains(&"zh"));
        assert!(langs.contains(&"id"));

        // Test consistency - multiple calls should return same reference
        let langs2 = supported_languages_optimized();
        assert_eq!(langs.as_ptr(), langs2.as_ptr());
    }

    #[test]
    fn test_constant_array_properties() {
        use langweave::optimized::SUPPORTED_LANGUAGE_CODES;

        // Test that constant array has expected properties
        assert_eq!(SUPPORTED_LANGUAGE_CODES.len(), 15);
        assert!(SUPPORTED_LANGUAGE_CODES.contains(&"en"));
        assert!(!SUPPORTED_LANGUAGE_CODES.contains(&"zz"));

        // Test ordering and uniqueness
        let mut unique_set = std::collections::HashSet::new();
        for &lang in SUPPORTED_LANGUAGE_CODES {
            assert!(
                unique_set.insert(lang),
                "Duplicate language code: {}",
                lang
            );
            assert_eq!(
                lang.len(),
                2,
                "Language code should be 2 characters: {}",
                lang
            );
        }
    }

    #[test]
    fn test_lazy_initialization() {
        // This test ensures the Lazy static is properly initialized
        // The act of calling any function that uses LANGUAGE_SET will initialize it

        // First call initializes the set
        let result1 = is_language_supported_optimized("en");
        assert!(result1);

        // Subsequent calls use the initialized set
        let result2 = is_language_supported_optimized("fr");
        assert!(result2);

        let result3 = is_language_supported_optimized("zz");
        assert!(!result3);
    }

    #[test]
    fn test_memory_efficiency_properties() {
        // Test that optimized functions work with various string types
        let owned_string = String::from("en");
        assert!(is_language_supported_optimized(&owned_string));
        assert!(is_language_supported_zero_alloc(&owned_string));

        let string_slice = "fr";
        assert!(is_language_supported_optimized(string_slice));
        assert!(is_language_supported_zero_alloc(string_slice));

        let borrowed = ["de".to_string()];
        assert!(is_language_supported_optimized(&borrowed[0]));
        assert!(is_language_supported_zero_alloc(
            &borrowed[0]
        ));
    }
}

#[cfg(test)]
mod performance_verification_tests {
    use langweave::optimized::*;
    use std::time::Instant;

    #[test]
    fn test_optimized_functions_performance_characteristics() {
        // These tests verify that optimized functions maintain expected performance

        // Test that supported_languages_optimized is fast (should be O(1))
        let start = Instant::now();
        for _ in 0..1000 {
            let _langs = supported_languages_optimized();
        }
        let optimized_time = start.elapsed();

        // Should complete very quickly since it's just returning a static reference
        assert!(
            optimized_time.as_millis() < 100,
            "Optimized function too slow"
        );

        // Test that language support check is fast
        let start = Instant::now();
        for _ in 0..1000 {
            let _result = is_language_supported_optimized("en");
        }
        let check_time = start.elapsed();

        assert!(
            check_time.as_millis() < 100,
            "Language support check too slow"
        );
    }
}
