// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use thiserror::Error;

/// Represents errors that can occur during internationalization and translation operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum I18nError {
    /// Indicates that the language detection process failed.
    ///
    /// This error occurs when the library is unable to determine the language of the provided text.
    #[error("Failed to detect language: the provided text does not contain sufficient identifiable language patterns")]
    LanguageDetectionFailed,

    /// Indicates that the translation process failed for the given text.
    ///
    /// This error occurs when the library is unable to translate the provided text,
    /// either due to unsupported words or phrases, or other translation-related issues.
    #[error("Failed to translate text: {0}")]
    TranslationFailed(String),

    /// Indicates that the requested language is not supported by the library.
    ///
    /// This error occurs when attempting to use a language that is not implemented
    /// in the current version of the library.
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    /// Represents any other unexpected errors that may occur during library operations.
    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),
}

impl I18nError {
    /// Returns a string slice describing the error.
    ///
    /// This method provides a short, human-readable description of the error type,
    /// suitable for logging or displaying to users in a simplified format.
    ///
    /// # Returns
    ///
    /// A string slice containing a brief description of the error type.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::error::I18nError;
    ///
    /// let error = I18nError::LanguageDetectionFailed;
    /// assert_eq!(error.as_str(), "language detection failed");
    ///
    /// let error = I18nError::UnsupportedLanguage("xyz".to_string());
    /// assert_eq!(error.as_str(), "unsupported language");
    ///
    /// let error = I18nError::TranslationFailed("missing key".to_string());
    /// assert_eq!(error.as_str(), "translation failed");
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            I18nError::LanguageDetectionFailed => {
                "language detection failed"
            }
            I18nError::TranslationFailed(_) => "translation failed",
            I18nError::UnsupportedLanguage(_) => "unsupported language",
            I18nError::UnexpectedError(_) => "unexpected error",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        assert_eq!(
            I18nError::LanguageDetectionFailed.to_string(),
            "Failed to detect language: the provided text does not contain sufficient identifiable language patterns"
        );
        assert_eq!(
            I18nError::TranslationFailed("test".to_string())
                .to_string(),
            "Failed to translate text: test"
        );
        assert_eq!(
            I18nError::UnsupportedLanguage("xyz".to_string())
                .to_string(),
            "Unsupported language: xyz"
        );
        assert_eq!(
            I18nError::UnexpectedError("test error".to_string())
                .to_string(),
            "An unexpected error occurred: test error"
        );
    }

    #[test]
    fn test_as_str() {
        assert_eq!(
            I18nError::LanguageDetectionFailed.as_str(),
            "language detection failed"
        );
        assert_eq!(
            I18nError::TranslationFailed("test".to_string()).as_str(),
            "translation failed"
        );
        assert_eq!(
            I18nError::UnsupportedLanguage("xyz".to_string()).as_str(),
            "unsupported language"
        );
        assert_eq!(
            I18nError::UnexpectedError("test error".to_string())
                .as_str(),
            "unexpected error"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(
            I18nError::TranslationFailed("error".to_string()),
            I18nError::TranslationFailed("error".to_string())
        );
        assert_ne!(
            I18nError::TranslationFailed("error1".to_string()),
            I18nError::TranslationFailed("error2".to_string())
        );
        assert_eq!(
            I18nError::UnsupportedLanguage("en".to_string()),
            I18nError::UnsupportedLanguage("en".to_string())
        );
        assert_ne!(
            I18nError::UnsupportedLanguage("en".to_string()),
            I18nError::UnsupportedLanguage("fr".to_string())
        );
        assert_eq!(
            I18nError::UnexpectedError("oops".to_string()),
            I18nError::UnexpectedError("oops".to_string())
        );
        assert_ne!(
            I18nError::UnexpectedError("oops1".to_string()),
            I18nError::UnexpectedError("oops2".to_string())
        );
    }

    #[test]
    fn test_error_clone() {
        let error = I18nError::TranslationFailed("test".to_string());
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);

        let error = I18nError::UnsupportedLanguage("xyz".to_string());
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);

        let error =
            I18nError::UnexpectedError("unexpected".to_string());
        let cloned_error = error.clone();
        assert_eq!(error, cloned_error);
    }

    #[test]
    fn test_error_debug() {
        let error = I18nError::TranslationFailed("test".to_string());
        assert_eq!(
            format!("{:?}", error),
            "TranslationFailed(\"test\")"
        );

        let error = I18nError::UnsupportedLanguage("xyz".to_string());
        assert_eq!(
            format!("{:?}", error),
            "UnsupportedLanguage(\"xyz\")"
        );

        let error =
            I18nError::UnexpectedError("unexpected".to_string());
        assert_eq!(
            format!("{:?}", error),
            "UnexpectedError(\"unexpected\")"
        );
    }

    #[test]
    fn test_error_partial_eq() {
        let error1 = I18nError::TranslationFailed("test".to_string());
        let error2 = I18nError::TranslationFailed("test".to_string());
        let error3 =
            I18nError::TranslationFailed("different".to_string());

        assert!(error1 == error2);
        assert!(error1 != error3);

        let error1 = I18nError::UnsupportedLanguage("en".to_string());
        let error2 = I18nError::UnsupportedLanguage("en".to_string());
        let error3 = I18nError::UnsupportedLanguage("fr".to_string());

        assert!(error1 == error2);
        assert!(error1 != error3);

        let error1 = I18nError::UnexpectedError("oops".to_string());
        let error2 = I18nError::UnexpectedError("oops".to_string());
        let error3 =
            I18nError::UnexpectedError("different".to_string());

        assert!(error1 == error2);
        assert!(error1 != error3);
    }

    #[test]
    fn test_error_non_exhaustive() {
        // This test demonstrates that we can match on all current variants
        // without a wildcard pattern, proving that #[non_exhaustive] doesn't
        // affect matching within the same crate.
        fn use_error(error: I18nError) {
            match error {
                I18nError::LanguageDetectionFailed => {}
                I18nError::TranslationFailed(_) => {}
                I18nError::UnsupportedLanguage(_) => {}
                I18nError::UnexpectedError(_) => {}
            }
        }

        // Use the function to avoid unused function warning
        use_error(I18nError::LanguageDetectionFailed);
    }

    #[test]
    fn test_error_variants() {
        // This test ensures that all expected variants are present
        let errors = vec![
            I18nError::LanguageDetectionFailed,
            I18nError::TranslationFailed("test".to_string()),
            I18nError::UnsupportedLanguage("en".to_string()),
            I18nError::UnexpectedError("oops".to_string()),
        ];

        for error in errors {
            match error {
                I18nError::LanguageDetectionFailed => {}
                I18nError::TranslationFailed(_) => {}
                I18nError::UnsupportedLanguage(_) => {}
                I18nError::UnexpectedError(_) => {}
            }
        }
    }
}
