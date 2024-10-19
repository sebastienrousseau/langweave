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
}
