//! # Translator Module
//!
//! This module provides functionality to translate text into different languages.

use crate::error::I18nError;
use crate::translations;
use std::fmt;

/// A struct responsible for translating text into different languages.
#[derive(Debug, Clone)]
pub struct Translator {
    lang: String,
}

impl Translator {
    /// Creates a new `Translator` instance for a specific language.
    ///
    /// # Arguments
    ///
    /// * `lang` - A string slice that holds the language code (e.g., "en", "fr", "de")
    ///
    /// # Returns
    ///
    /// * `Result<Translator, I18nError>` - The translator instance or an error if the language is unsupported
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::translator::Translator;
    ///
    /// let translator = Translator::new("en").unwrap();
    /// assert_eq!(translator.lang(), "en");
    /// ```
    pub fn new(lang: &str) -> Result<Self, I18nError> {
        let lang = lang.to_lowercase();
        // Check if the language is supported by trying to translate a known key
        match translations::translate(&lang, "Hello") {
            Ok(_) => Ok(Translator { lang }),
            Err(I18nError::UnsupportedLanguage(_)) => {
                Err(I18nError::UnsupportedLanguage(lang))
            }
            Err(e) => Err(e),
        }
    }

    /// Translates the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to be translated
    ///
    /// # Returns
    ///
    /// * `Result<String, I18nError>` - The translated string or an error if translation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::translator::Translator;
    ///
    /// let translator = Translator::new("fr").unwrap();
    /// assert_eq!(translator.translate("Hello").unwrap(), "Bonjour");
    /// ```
    pub fn translate(&self, text: &str) -> Result<String, I18nError> {
        translations::translate(&self.lang, text)
    }

    /// Returns the language code of this translator.
    ///
    /// # Returns
    ///
    /// * `&str` - The language code
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::translator::Translator;
    ///
    /// let translator = Translator::new("de").unwrap();
    /// assert_eq!(translator.lang(), "de");
    /// ```
    pub fn lang(&self) -> &str {
        &self.lang
    }
}

impl fmt::Display for Translator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Translator for language: {}", self.lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation_supported_languages() {
        let test_cases =
            vec![("en", "Hello"), ("fr", "Bonjour"), ("de", "Hallo")];
        for (lang, expected) in test_cases {
            let translator = Translator::new(lang).unwrap();
            assert_eq!(
                translator.translate("Hello").unwrap(),
                expected
            );
        }
    }

    #[test]
    fn test_unsupported_language() {
        let result = Translator::new("es");
        assert!(matches!(
            result,
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[test]
    fn test_case_insensitive_language_code() {
        let translator = Translator::new("FR").unwrap();
        assert_eq!(translator.lang(), "fr");
        assert_eq!(translator.translate("Hello").unwrap(), "Bonjour");
    }

    #[test]
    fn test_display_implementation() {
        let translator = Translator::new("en").unwrap();
        assert_eq!(
            format!("{}", translator),
            "Translator for language: en"
        );
    }
}
