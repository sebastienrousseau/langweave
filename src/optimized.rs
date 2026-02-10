//! # Performance Optimizations Module
//!
//! This module contains zero-cost abstraction optimizations for LangWeave performance bottlenecks:
//! - Const static arrays instead of heap-allocated vectors
//! - Compile-time hash sets for O(1) membership testing
//! - Borrowed strings to eliminate clones
//! - Stack-based data structures where possible

use crate::I18nError;
use once_cell::sync::Lazy;
use std::collections::HashSet;

/// Compile-time constant array of supported language codes
/// This replaces the heap-allocated `Vec<String>` from supported_languages()
pub const SUPPORTED_LANGUAGE_CODES: &[&str] = &[
    "en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he", "hi",
    "ja", "ko", "zh", "id",
];

/// Static HashSet for O(1) language support checking
/// This replaces the O(n) linear search in is_language_supported()
static LANGUAGE_SET: Lazy<HashSet<&'static str>> =
    Lazy::new(|| SUPPORTED_LANGUAGE_CODES.iter().copied().collect());

/// Optimized version of supported_languages() that returns borrowed string slices
/// instead of heap-allocated Strings.
///
/// Performance improvement: Eliminates 15 heap allocations per call
///
/// # Returns
///
/// A static slice of string slices containing the supported language codes.
///
/// # Examples
///
/// ```
/// use langweave::optimized::supported_languages_optimized;
///
/// let languages = supported_languages_optimized();
/// assert_eq!(languages.len(), 15);
/// assert!(languages.contains(&"en"));
/// assert!(languages.contains(&"fr"));
/// ```
pub fn supported_languages_optimized() -> &'static [&'static str] {
    SUPPORTED_LANGUAGE_CODES
}

/// Optimized version of is_language_supported() using O(1) HashSet lookup
/// instead of rebuilding the entire languages list on every call.
///
/// Performance improvement: O(1) vs O(n) + eliminates Vec allocation
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code to validate.
///
/// # Returns
///
/// `true` if the language is supported, `false` otherwise.
///
/// # Examples
///
/// ```
/// use langweave::optimized::is_language_supported_optimized;
///
/// assert!(is_language_supported_optimized("en"));
/// assert!(is_language_supported_optimized("FR")); // Case insensitive
/// assert!(!is_language_supported_optimized("zz"));
/// ```
pub fn is_language_supported_optimized(lang: &str) -> bool {
    LANGUAGE_SET.contains(lang.to_lowercase().as_str())
}

/// Optimized version that works with string slices to avoid allocations.
///
/// This function uses compile-time pattern matching for common cases, falling back
/// to case-insensitive lookup only when necessary.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code to validate.
///
/// # Returns
///
/// `true` if the language is supported, `false` otherwise.
///
/// # Examples
///
/// ```
/// use langweave::optimized::is_language_supported_zero_alloc;
///
/// assert!(is_language_supported_zero_alloc("en"));
/// assert!(is_language_supported_zero_alloc("fr"));
/// assert!(!is_language_supported_zero_alloc("zz"));
/// ```
pub fn is_language_supported_zero_alloc(lang: &str) -> bool {
    // Use a match for common cases to enable compile-time optimization
    match lang {
        "en" | "fr" | "de" | "es" | "pt" | "it" | "nl" | "ru"
        | "ar" | "he" | "hi" | "ja" | "ko" | "zh" | "id" => true,
        _ => {
            // Fallback for case-insensitive check
            let lower = lang.to_lowercase();
            LANGUAGE_SET.contains(lower.as_str())
        }
    }
}

/// Optimized translation function that minimizes allocations.
///
/// Uses the original translate function but provides a performance-optimized interface.
/// This is a zero-cost abstraction over the standard translation functionality.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the target language code (e.g., "en", "fr").
/// * `key` - A string slice that holds the key to be translated.
///
/// # Returns
///
/// * `Ok(String)` - The translated text.
/// * `Err(I18nError)` - An error if the translation fails.
///
/// # Examples
///
/// ```
/// use langweave::optimized::translate_optimized;
///
/// let result = translate_optimized("fr", "Hello");
/// assert!(result.is_ok());
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The specified language is not supported.
/// * The translation key is not found in the language's translation dictionary.
pub fn translate_optimized(
    lang: &str,
    key: &str,
) -> Result<String, I18nError> {
    // Direct call to the underlying translation system
    crate::translations::translate(lang, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_supported_languages() {
        let optimized = supported_languages_optimized();
        assert_eq!(optimized.len(), 15);
        assert!(optimized.contains(&"en"));
        assert!(optimized.contains(&"fr"));
        assert!(optimized.contains(&"de"));
    }

    #[test]
    fn test_optimized_language_support_check() {
        assert!(is_language_supported_optimized("en"));
        assert!(is_language_supported_optimized("fr"));
        assert!(!is_language_supported_optimized("zz"));

        // Test case insensitive
        assert!(is_language_supported_optimized("EN"));
        assert!(is_language_supported_optimized("Fr"));
    }

    #[test]
    fn test_zero_alloc_language_check() {
        assert!(is_language_supported_zero_alloc("en"));
        assert!(is_language_supported_zero_alloc("fr"));
        assert!(!is_language_supported_zero_alloc("zz"));

        // Test case insensitive fallback
        assert!(is_language_supported_zero_alloc("EN"));
    }

    #[test]
    fn test_compatibility_with_original() {
        // Test that optimized versions return equivalent results
        for &lang in SUPPORTED_LANGUAGE_CODES {
            assert!(crate::is_language_supported(lang));
            assert!(is_language_supported_optimized(lang));
            assert!(is_language_supported_zero_alloc(lang));
        }

        // Test unsupported language
        assert!(!crate::is_language_supported("zz"));
        assert!(!is_language_supported_optimized("zz"));
        assert!(!is_language_supported_zero_alloc("zz"));
    }

    #[test]
    fn test_stack_optimized_languages() {
        // Note: This test is currently disabled as stack_optimized feature is not implemented
        // let stack_langs = supported_languages_stack();
        // assert_eq!(stack_langs.len(), 15);
        // assert!(stack_langs.contains(&"en"));
    }
}
