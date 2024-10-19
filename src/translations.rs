use crate::I18nError;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

type TranslationMap = HashMap<String, HashMap<String, String>>;

lazy_static! {
    static ref TRANSLATIONS: TranslationMap = load_all_translations();
}

fn load_translations_from_dir(dir: &Path) -> TranslationMap {
    let mut all_translations = TranslationMap::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "po" {
                    if let Some(lang_code) = path.file_stem().and_then(|s| s.to_str()) {
                        match load_translations(&path) {
                            Ok(translations) => {
                                let _ = all_translations.insert(lang_code.to_lowercase(), translations);
                            }
                            Err(e) => eprintln!("Error loading translations for {:?}: {}", path, e),
                        }
                    }
                }
            }
        }
    }

    all_translations
}

fn load_translations(file_path: &Path) -> Result<HashMap<String, String>, std::io::Error> {
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
                let _ = translations.insert(current_msgid.clone(), msgstr);
            }
            current_msgid.clear();
        }
    }

    Ok(translations)
}

fn load_all_translations() -> TranslationMap {
    println!("Current working directory: {:?}", env::current_dir().unwrap());

    let locales_dir = env::current_dir().unwrap().join("locales");
    println!("Looking for locales in: {:?}", locales_dir);

    if locales_dir.exists() && locales_dir.is_dir() {
        println!("Locales directory found. Contents:");
        if let Ok(entries) = fs::read_dir(&locales_dir) {
            for entry in entries.flatten() {
                println!("  {:?}", entry.path());
            }
        }
        load_translations_from_dir(&locales_dir)
    } else {
        println!("Locales directory not found or is not a directory.");
        TranslationMap::new()
    }
}

fn parse_po_string(line: &str, prefix: &str) -> String {
    line.trim_start_matches(prefix)
        .trim_matches('"')
        .replace("\\\"", "\"")
}

/// Translates a given key into the specified language.
///
/// # Arguments
///
/// * `lang` - A string slice that holds the language code (e.g., "en", "fr").
/// * `key` - A string slice that holds the key to be translated.
///
/// # Returns
///
/// * `Ok(String)` - The translated string if found.
/// * `Err(I18nError)` - An error if the translation fails or the language is unsupported.
pub fn translate(lang: &str, key: &str) -> Result<String, I18nError> {
    let translations = TRANSLATIONS.get(lang.to_lowercase().as_str())
        .ok_or_else(|| I18nError::UnsupportedLanguage(lang.to_string()))?;

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
            println!("Locales directory not found or is not a directory.");
        }
    }

    #[test]
    fn test_po_files_exist() {
        let locales_dir = env::current_dir().unwrap().join("locales");
        assert!(locales_dir.exists(), "locales directory does not exist at {:?}", locales_dir);

        for lang in &["en", "fr", "de"] {
            let po_file = locales_dir.join(format!("{}.po", lang));
            assert!(po_file.exists(), "{}.po file does not exist at {:?}", lang, po_file);
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
        let test_keys = vec!["Hello", "Goodbye", "Yes", "No", "Thank you", "Please"];

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
        assert!(translate("en", "main_logger_msg").unwrap().contains("Please run `ssg --help`"));
        assert!(translate("fr", "lib_banner_log_msg").unwrap().contains("Bannière imprimée"));
        assert!(translate("de", "lib_server_log_msg").unwrap().contains("Server erfolgreich gestartet"));
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
    fn test_case_sensitivity() {
        assert_eq!(translate("en", "hello").unwrap(), "Hello");
        assert_eq!(translate("fr", "GOODBYE").unwrap(), "Au revoir");
    }
}
