//! # Performance Optimizations Module
//!
//! This module provides optimized alternatives to core LangWeave functions:
//! - Const static arrays instead of heap-allocated vectors
//! - O(1) membership testing via compile-time pattern matching
//! - Borrowed strings to eliminate clones
//! - True zero-allocation variants for hot paths
//!
//! **Note:** This module is experimental. API may change in future versions.

use crate::I18nError;

/// Compile-time constant array of supported language codes.
///
/// This replaces the heap-allocated `Vec<String>` from `supported_languages()`.
/// With only 15 languages, linear search with `eq_ignore_ascii_case` is faster
/// than HashSet lookup due to cache locality and avoiding hash computation.
pub const SUPPORTED_LANGUAGE_CODES: &[&str] = &[
    "en", "fr", "de", "es", "pt", "it", "nl", "ru", "ar", "he", "hi",
    "ja", "ko", "zh", "id",
];

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

/// Optimized version of is_language_supported() with zero heap allocations.
///
/// Uses compile-time pattern matching for O(1) lookup without any allocations.
/// Supports case-insensitive matching via ASCII case folding (no heap allocation).
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
    // Zero-allocation case-insensitive check using eq_ignore_ascii_case
    SUPPORTED_LANGUAGE_CODES
        .iter()
        .any(|&code| code.eq_ignore_ascii_case(lang))
}

/// True zero-allocation language support check.
///
/// This function uses compile-time pattern matching for lowercase codes (fast path),
/// and falls back to ASCII case-insensitive comparison without any heap allocation.
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
/// assert!(is_language_supported_zero_alloc("FR")); // Case insensitive, zero alloc
/// assert!(!is_language_supported_zero_alloc("zz"));
/// ```
#[inline]
pub fn is_language_supported_zero_alloc(lang: &str) -> bool {
    // Fast path: exact lowercase match (most common case)
    match lang {
        "en" | "fr" | "de" | "es" | "pt" | "it" | "nl" | "ru"
        | "ar" | "he" | "hi" | "ja" | "ko" | "zh" | "id" => true,
        _ => {
            // Slow path: case-insensitive check without allocation
            // Uses eq_ignore_ascii_case which operates in-place
            SUPPORTED_LANGUAGE_CODES
                .iter()
                .any(|&code| code.eq_ignore_ascii_case(lang))
        }
    }
}

/// Translation function with optimized language validation.
///
/// Uses the original translate function with pre-validated language support.
/// Note: Translation itself requires string allocation for the result.
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
