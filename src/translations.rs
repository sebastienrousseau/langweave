//! # Translations Module
//!
//! This module provides functionality to load and manage translations from PO (Portable Object)
//! files. PO files are a standard format for storing translatable strings and their translations.
//!
//! The module automatically loads translation files from a `locales/` directory in the project
//! root. Each supported language should have its own `.po` file named with the language code
//! (e.g., `en.po`, `fr.po`, `de.po`).
//!
//! ## PO File Format
//!
//! PO files contain key-value pairs where keys (msgid) map to translations (msgstr):
//!
//! ```po
//! msgid "Hello"
//! msgstr "Bonjour"
//!
//! msgid "Goodbye"
//! msgstr "Au revoir"
//! ```
//!
//! ## Examples
//!
//! ```
//! use langweave::translations::translate;
//!
//! // Translate a key to French
//! let result = translate("fr", "Hello");
//! // Returns Ok("Bonjour") if the translation exists
//!
//! // Handle missing translations
//! match translate("fr", "NonexistentKey") {
//!     Ok(translation) => println!("Found: {}", translation),
//!     Err(error) => println!("Translation failed: {}", error),
//! }
//! ```

use crate::I18nError;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Type alias for the nested HashMap structure that stores translations.
///
/// The outer HashMap maps language codes to translation dictionaries.
/// The inner HashMap maps original text keys to translated strings.
type TranslationMap = HashMap<String, HashMap<String, String>>;

/// Global translation storage loaded at runtime from PO files.
///
/// This static variable is initialized once when first accessed and contains
/// all available translations loaded from the `locales/` directory.
static TRANSLATIONS: Lazy<TranslationMap> =
    Lazy::new(load_all_translations);

/// Loads translations from all PO files in the specified directory.
///
/// This function scans a directory for `.po` files and loads each one as a
/// language-specific translation dictionary. The language code is derived
/// from the filename (e.g., `fr.po` becomes language code "fr").
///
/// # Arguments
///
/// * `dir` - Path to the directory containing PO files
///
/// # Returns
///
/// A `TranslationMap` containing all loaded translations organized by language code.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// use langweave::translations::load_translations_from_dir;
///
/// let translations = load_translations_from_dir(Path::new("locales"));
/// // Returns HashMap with language codes as keys and translation dictionaries as values
/// ```
fn load_translations_from_dir(dir: &Path) -> TranslationMap {
    let mut all_translations = TranslationMap::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "po" {
                    if let Some(lang_code) =
                        path.file_stem().and_then(|s| s.to_str())
                    {
                        match load_translations(&path) {
                            Ok(translations) => {
                                let _ = all_translations.insert(
                                    lang_code.to_lowercase(),
                                    translations,
                                );
                            }
                            Err(e) => eprintln!(
                                "Error loading translations for {:?}: {}",
                                path, e
                            ),
                        }
                    }
                }
            }
        }
    }

    all_translations
}

/// Loads translations from a single PO file.
///
/// This function parses a PO (Portable Object) file and extracts key-value pairs
/// where `msgid` entries become keys and `msgstr` entries become values.
///
/// # Arguments
///
/// * `file_path` - Path to the PO file to load
///
/// # Returns
///
/// * `Ok(HashMap<String, String>)` - A dictionary of translations if successful
/// * `Err(std::io::Error)` - An I/O error if the file cannot be read
///
/// # PO File Format
///
/// The function expects standard PO format:
/// ```po
/// msgid "original text"
/// msgstr "translated text"
/// ```
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// use langweave::translations::load_translations;
///
/// let translations = load_translations(Path::new("locales/fr.po"))?;
/// // Returns HashMap with French translations
/// ```
fn load_translations(
    file_path: &Path,
) -> Result<HashMap<String, String>, std::io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut translations = HashMap::new();
    let mut current_msgid = String::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.starts_with("msgid ") {
            current_msgid = parse_po_string(line, "msgid ");
        } else if line.starts_with("msgstr ") {
            let msgstr = parse_po_string(line, "msgstr ");
            if !current_msgid.is_empty() && !msgstr.is_empty() {
                let _ =
                    translations.insert(current_msgid.clone(), msgstr);
            }
            current_msgid.clear();
        }
    }

    Ok(translations)
}

/// Loads all translations from the default locales directory.
///
/// This function looks for a `locales/` directory in the current working directory
/// and loads all PO files found within it. If the directory doesn't exist,
/// returns an empty translation map.
///
/// # Returns
///
/// A `TranslationMap` containing all loaded translations, or an empty map if
/// no locales directory is found.
///
/// # Directory Structure Expected
///
/// ```text
/// locales/
/// ├── en.po
/// ├── fr.po
/// ├── de.po
/// └── ...
/// ```
fn load_all_translations() -> TranslationMap {
    let locales_dir = env::current_dir().unwrap().join("locales");

    if locales_dir.exists() && locales_dir.is_dir() {
        println!("Locales directory found. Contents:");
        if let Ok(entries) = fs::read_dir(&locales_dir) {
            for entry in entries.flatten() {
                println!("  {:?}", entry.path());
            }
        }
        load_translations_from_dir(&locales_dir)
    } else {
        TranslationMap::new()
    }
}

/// Parses a PO file line to extract the string content.
///
/// This function removes the prefix (`msgid` or `msgstr`) and quotes from a PO file line,
/// and handles escaped quotes within the string.
///
/// # Arguments
///
/// * `line` - The PO file line to parse
/// * `prefix` - The prefix to remove ("msgid " or "msgstr ")
///
/// # Returns
///
/// The extracted string content with quotes removed and escape sequences handled.
///
/// # Examples
///
/// ```ignore
/// use langweave::translations::parse_po_string;
///
/// let result = parse_po_string("msgid \"Hello\"", "msgid ");
/// assert_eq!(result, "Hello");
///
/// let result = parse_po_string("msgstr \"He said \\\"Hi\\\"\"", "msgstr ");
/// assert_eq!(result, "He said \"Hi\"");
/// ```
fn parse_po_string(line: &str, prefix: &str) -> String {
    line.trim_start_matches(prefix)
        .trim_matches('"')
        .replace("\\\"", "\"")
}

/// Translates a given key into the specified language.
///
/// This function looks up a translation for the given key in the specified language.
/// It first tries an exact match, then falls back to a case-insensitive search.
/// The translations are loaded from PO files in the `locales/` directory.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code (e.g., "en", "fr", "de")
/// * `key` - A string slice that holds the key to be translated
///
/// # Returns
///
/// * `Ok(String)` - The translated string if found
/// * `Err(I18nError)` - An error if the translation fails or the language is unsupported
///
/// # Examples
///
/// ```
/// use langweave::translations::translate;
///
/// // Basic translation
/// let result = translate("fr", "Hello");
/// assert!(result.is_ok());
///
/// // Case-insensitive matching
/// let result = translate("fr", "hello");
/// assert!(result.is_ok());
///
/// // Handle unsupported language
/// match translate("zz", "Hello") {
///     Err(langweave::error::I18nError::UnsupportedLanguage(lang)) => {
///         println!("Language {} not supported", lang);
///     }
///     _ => {}
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The specified language is not supported (no PO file found)
/// * The translation key is not found in the language's translation dictionary
pub fn translate(lang: &str, key: &str) -> Result<String, I18nError> {
    let translations =
        TRANSLATIONS.get(lang.to_lowercase().as_str()).ok_or_else(
            || I18nError::UnsupportedLanguage(lang.to_string()),
        )?;

    // Try exact match first
    if let Some(translation) = translations.get(key) {
        return Ok(translation.clone());
    }

    // If not found, try case-insensitive match
    for (k, v) in translations {
        if k.to_lowercase() == key.to_lowercase() {
            return Ok(v.clone());
        }
    }

    Err(I18nError::TranslationFailed(format!("{}:{}", lang, key)))
}

/// Translates a given text with intelligent fallback handling.
///
/// This function attempts to translate text and applies fallback logic for simple keys.
/// For simple keys (single words without complex punctuation), it falls back to the
/// original text when translation is not found. For complex phrases, it returns an error.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code (e.g., "en", "fr", "de")
/// * `text` - A string slice that holds the text to be translated
///
/// # Returns
///
/// * `Ok(String)` - The translated text, or the original text for simple keys when translation fails
/// * `Err(I18nError)` - An error if the language is unsupported or translation fails for complex phrases
///
/// # Fallback Logic
///
/// - **Simple keys** (no spaces, commas, question marks, or exclamation marks): Fall back to original text
/// - **Complex phrases** (containing spaces or punctuation): Return TranslationFailed error
///
/// # Examples
///
/// ```
/// use langweave::translations::translate_with_fallback;
///
/// // Successful translation
/// let result = translate_with_fallback("fr", "Hello");
/// assert_eq!(result.unwrap(), "Bonjour");
///
/// // Simple key fallback (if "SimpleKey" not in dictionary)
/// let result = translate_with_fallback("fr", "SimpleKey");
/// // Returns Ok("SimpleKey") - falls back to original
///
/// // Complex phrase error (if not in dictionary)
/// let result = translate_with_fallback("fr", "Complex phrase with spaces");
/// // Returns Err(I18nError::TranslationFailed(...))
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The specified language is not supported
/// * Translation fails for complex phrases (containing spaces or punctuation)
pub fn translate_with_fallback(
    lang: &str,
    text: &str,
) -> Result<String, I18nError> {
    // Try to translate first
    match translate(lang, text) {
        Ok(translation) => Ok(translation),
        Err(I18nError::UnsupportedLanguage(lang_code)) => {
            Err(I18nError::UnsupportedLanguage(lang_code))
        }
        Err(_) => {
            // Translation failed - apply fallback logic
            // Only fallback for simple keys (single word, no punctuation except basic ones)
            if text.contains(' ')
                || text.contains(',')
                || text.contains('?')
                || text.contains('!')
            {
                Err(I18nError::TranslationFailed(format!(
                    "Complex phrase translation not found: {}",
                    text
                )))
            } else {
                // Simple key - fallback to original text
                Ok(text.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_locales_contents() {
        let locales_dir = env::current_dir().unwrap().join("locales");
        println!("Locales directory: {:?}", locales_dir);
        if locales_dir.exists() && locales_dir.is_dir() {
            println!("Contents:");
            if let Ok(entries) = fs::read_dir(&locales_dir) {
                for entry in entries.flatten() {
                    println!("  {:?}", entry.path());
                }
            }
        } else {
            println!(
                "Locales directory not found or is not a directory."
            );
        }
    }

    #[test]
    fn test_po_files_exist() {
        let locales_dir = env::current_dir().unwrap().join("locales");
        assert!(
            locales_dir.exists(),
            "locales directory does not exist at {:?}",
            locales_dir
        );

        for lang in &["en", "fr", "de"] {
            let po_file = locales_dir.join(format!("{}.po", lang));
            assert!(
                po_file.exists(),
                "{}.po file does not exist at {:?}",
                lang,
                po_file
            );
        }
    }

    #[test]
    fn print_loaded_translations() {
        println!("Loaded translations: {:#?}", *TRANSLATIONS);
    }

    #[test]
    fn test_basic_translations() {
        assert_eq!(translate("en", "Hello").unwrap(), "Hello");
        assert_eq!(translate("fr", "Hello").unwrap(), "Bonjour");
        assert_eq!(translate("de", "Hello").unwrap(), "Hallo");
    }

    #[test]
    fn test_all_languages() {
        let languages = vec!["en", "fr", "de"];
        let test_keys = vec![
            "Hello",
            "Goodbye",
            "Yes",
            "No",
            "Thank you",
            "Please",
        ];

        for lang in languages {
            for key in &test_keys {
                assert!(
                    translate(lang, key).is_ok(),
                    "Failed to translate '{}' in {}",
                    key,
                    lang
                );
            }
        }
    }

    #[test]
    fn test_specific_translations() {
        assert_eq!(translate("en", "Goodbye").unwrap(), "Goodbye");
        assert_eq!(translate("fr", "Thank you").unwrap(), "Merci");
        assert_eq!(translate("de", "Please").unwrap(), "Bitte");
    }

    #[test]
    fn test_logger_messages() {
        assert!(
            translate("en", "main_logger_msg")
                .unwrap()
                .contains("Please run `ssg --help`")
        );
        assert!(
            translate("fr", "lib_banner_log_msg")
                .unwrap()
                .contains("Bannière imprimée")
        );
        assert!(
            translate("de", "lib_server_log_msg")
                .unwrap()
                .contains("Server erfolgreich gestartet")
        );
    }

    #[test]
    fn test_missing_translation() {
        assert!(matches!(
            translate("en", "NonexistentKey"),
            Err(I18nError::TranslationFailed(_))
        ));
    }

    #[test]
    fn test_unsupported_language() {
        assert!(matches!(
            translate("xx", "Hello"),
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[test]
    fn test_translate_with_fallback_unsupported_language() {
        assert!(matches!(
            translate_with_fallback("xx", "Hello"),
            Err(I18nError::UnsupportedLanguage(_))
        ));
    }

    #[test]
    fn test_case_sensitivity() {
        assert_eq!(translate("en", "hello").unwrap(), "Hello");
        assert_eq!(translate("fr", "GOODBYE").unwrap(), "Au revoir");
    }

    #[test]
    fn test_load_translations_nonexistent_file() {
        let result = load_translations(Path::new(
            "/tmp/langweave_test_nonexistent_file.po",
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_translations_from_dir_with_mixed_files() {
        let test_dir = Path::new("/tmp/langweave_test_mixed_dir");
        let _ = fs::create_dir_all(test_dir);
        // Non-.po file (triggers else on extension == "po")
        let _ = fs::write(test_dir.join("readme.txt"), "not a po file");
        // File without extension (triggers None on path.extension())
        let _ = fs::write(test_dir.join("noext"), "no extension");
        let result = load_translations_from_dir(test_dir);
        assert!(result.is_empty());
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_load_translations_from_dir_with_bad_po() {
        let test_dir = Path::new("/tmp/langweave_test_error_dir");
        let _ = fs::create_dir_all(test_dir);
        // Directory named "bad.po" — File::open will fail
        let _ = fs::create_dir(test_dir.join("bad.po"));
        let result = load_translations_from_dir(test_dir);
        assert!(result.is_empty());
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_load_translations_from_nonexistent_dir() {
        let result = load_translations_from_dir(Path::new(
            "/tmp/langweave_test_no_such_dir_exists",
        ));
        assert!(result.is_empty());
    }
}
