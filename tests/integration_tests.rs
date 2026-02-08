//! Integration tests for the langweave library
//!
//! This module contains integration tests to verify the correct functionality
//! of the langweave library's public API including translation, language detection,
//! and error handling.

use langweave::error::I18nError;
use langweave::translator::Translator;
use langweave::{detect_language, translate};

#[test]
fn test_translation() {
    assert_eq!(translate("fr", "Hello").unwrap(), "Bonjour");
    assert_eq!(translate("de", "Goodbye").unwrap(), "Auf Wiedersehen");
    assert!(translate("invalid", "Hello").is_err());
}

#[tokio::test]
async fn test_language_detection() {
    assert_eq!(
        detect_language("The quick brown fox").await.unwrap(),
        "en"
    );
    assert_eq!(detect_language("Le chat noir").await.unwrap(), "fr");
    assert_eq!(
        detect_language("Der schnelle Fuchs").await.unwrap(),
        "de"
    );
}

#[test]
fn test_translator() {
    let translator = Translator::new("en").unwrap();
    assert_eq!(translator.translate("Hello").unwrap(), "Hello");

    let result = Translator::new("invalid");
    assert!(matches!(result, Err(I18nError::UnsupportedLanguage(_))));
}
