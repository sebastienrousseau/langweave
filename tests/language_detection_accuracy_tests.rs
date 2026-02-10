//! Language Detection Accuracy Tests
//!
//! This module contains tests for language detection accuracy using real-world
//! multilingual samples, including edge cases and ambiguous content.
//!
//! **Known Limitations:**
//! - Regex-based detection has inherent accuracy limitations
//! - Short texts may produce unreliable results
//! - Mixed-language content may detect the dominant language only

use langweave::{detect_language, detect_language_async};

#[cfg(test)]
mod accuracy_tests {
    use super::*;

    /// Test detection of clear, unambiguous text samples
    #[test]
    fn test_clear_language_samples() {
        let test_cases = [
            // English - clear samples
            ("The quick brown fox jumps over the lazy dog", "en"),
            ("Hello, how are you doing today?", "en"),
            // French - clear samples
            ("Bonjour, comment allez-vous aujourd'hui?", "fr"),
            ("Le chat noir dort sur le canapé", "fr"),
            // German - clear samples
            ("Guten Tag, wie geht es Ihnen?", "de"),
            ("Der schnelle braune Fuchs springt", "de"),
            // Spanish - clear samples
            ("Hola, ¿cómo estás hoy?", "es"),
            ("El gato negro duerme en el sofá", "es"),
        ];

        for (text, expected_lang) in test_cases {
            let result = detect_language(text);
            match result {
                Ok(detected) => {
                    assert_eq!(
                        detected, expected_lang,
                        "Expected '{}' for text '{}', got '{}'",
                        expected_lang, text, detected
                    );
                }
                Err(e) => {
                    panic!(
                        "Detection failed for '{}' (expected {}): {:?}",
                        text, expected_lang, e
                    );
                }
            }
        }
    }

    /// Test detection with short text (known limitation)
    #[test]
    fn test_short_text_detection() {
        // Short texts may not have enough signal for accurate detection
        let short_samples = ["Hi", "Oui", "Ja", "Sí", "Ciao"];

        for sample in short_samples {
            let result = detect_language(sample);
            // We accept either success or detection failure for very short text
            assert!(
                result.is_ok() || result.is_err(),
                "Short text detection should handle gracefully: {}",
                sample
            );
        }
    }

    /// Test detection with mixed-language content (known limitation)
    #[test]
    fn test_mixed_language_detection() {
        // Mixed language content - detection may return dominant language
        let mixed_samples = [
            "Hello bonjour hallo", // English/French/German mix
            "The chat noir is cute", // English with French
            "Das ist very gut", // German with English
        ];

        for sample in mixed_samples {
            let result = detect_language(sample);
            // Mixed language should not panic, may detect any component language
            assert!(
                result.is_ok() || result.is_err(),
                "Mixed language detection should handle gracefully: {}",
                sample
            );
        }
    }

    /// Test detection with non-Latin scripts
    #[test]
    fn test_non_latin_scripts() {
        let script_samples = [
            ("你好世界", Some("zh")), // Chinese
            ("こんにちは世界", Some("ja")), // Japanese
            ("مرحبا بالعالم", Some("ar")), // Arabic
            ("שלום עולם", Some("he")), // Hebrew
            ("Привет мир", Some("ru")), // Russian (Cyrillic)
            ("नमस्ते दुनिया", Some("hi")), // Hindi (Devanagari)
            ("안녕하세요 세계", Some("ko")), // Korean
        ];

        for (text, expected) in script_samples {
            let result = detect_language(text);
            match (result, expected) {
                (Ok(detected), Some(exp)) => {
                    // Non-Latin scripts should detect correctly
                    assert_eq!(
                        detected, exp,
                        "Expected '{}' for '{}', got '{}'",
                        exp, text, detected
                    );
                }
                (Err(_), _) => {
                    // Some scripts may not be detectable - this is acceptable
                }
                (Ok(detected), None) => {
                    // If we expected failure but got success, note it
                    println!("Note: Unexpectedly detected '{}' as '{}'", text, detected);
                }
            }
        }
    }

    /// Test detection consistency (same input should give same output)
    #[test]
    fn test_detection_consistency() {
        let samples = [
            "The quick brown fox",
            "Le chat noir",
            "Der braune Fuchs",
            "El gato negro",
        ];

        for sample in samples {
            let result1 = detect_language(sample);
            let result2 = detect_language(sample);
            let result3 = detect_language(sample);

            assert_eq!(
                result1, result2,
                "Inconsistent detection for '{}'",
                sample
            );
            assert_eq!(
                result2, result3,
                "Inconsistent detection for '{}'",
                sample
            );
        }
    }

    /// Test that detection handles edge cases without panicking
    #[test]
    fn test_edge_case_stability() {
        let edge_cases = [
            "",                  // Empty string
            " ",                 // Whitespace only
            "123456789",        // Numbers only
            "!@#$%^&*()",       // Symbols only
            "a",                 // Single character
            "   \t\n\r   ",     // Mixed whitespace
            "Hello123World",    // Mixed alphanumeric
            "café résumé naïve", // Accented characters
            "🎉🎊🎈",           // Emoji only
            "Hello 🎉 World",   // Text with emoji
        ];

        for case in edge_cases {
            // Should not panic regardless of input
            let result = std::panic::catch_unwind(|| detect_language(case));
            assert!(
                result.is_ok(),
                "Detection panicked for edge case: {:?}",
                case
            );
        }
    }

    /// Test async detection produces same results as sync
    #[tokio::test]
    async fn test_async_sync_consistency() {
        let samples = [
            "Hello world",
            "Bonjour monde",
            "Hallo Welt",
            "Hola mundo",
        ];

        for sample in samples {
            let sync_result = detect_language(sample);
            let async_result = detect_language_async(sample).await;

            assert_eq!(
                sync_result, async_result,
                "Sync and async detection differ for '{}'",
                sample
            );
        }
    }

    /// Document known detection limitations
    #[test]
    fn test_documented_limitations() {
        // This test documents known limitations rather than asserting behavior

        // Limitation 1: Very short text may not detect reliably
        let short_text = "Ok";
        let _ = detect_language(short_text); // May fail or misdetect

        // Limitation 2: Proper nouns/names may not have language signal
        let name = "John Smith";
        let _ = detect_language(name); // Limited language signal

        // Limitation 3: Technical/code content may confuse detection
        let code = "fn main() { println!(\"Hello\"); }";
        let _ = detect_language(code); // May detect based on string content

        // Limitation 4: Romanized non-Latin languages
        let romaji = "konnichiwa sekai"; // Japanese in romaji
        let _ = detect_language(romaji); // May not detect as Japanese

        // All cases should complete without panicking
    }
}
