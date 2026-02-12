# Translation

Translate text between supported languages.

## Overview

LangWeave translation system supports basic text translation between English, French, and German using a predefined dictionary.

## Basic Translation

### Using Global Function

```rust
use langweave::translate;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    // Translate to French
    let result = translate("fr", "Hello")?;
    println!("Translation: {}", result); // "Bonjour"

    // Translate to German
    let result = translate("de", "Goodbye")?;
    println!("Translation: {}", result); // "Auf Wiedersehen"

    Ok(())
}
```

### Using Translator Instance

```rust
use langweave::translator::Translator;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    // Create translator for French
    let translator = Translator::new("fr")?;

    // Translate text
    let result = translator.translate("Hello")?;
    println!("Translation: {}", result); // "Bonjour"

    Ok(())
}
```

### Auto-Detect and Translate

Combine detection with translation:

```rust
use langweave::language_detector::LanguageDetector;
use langweave::translator::Translator;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let detector = LanguageDetector::new();
    let text = "Hello, world!";

    // Detect source language
    let source_lang = detector.detect_async(text).await?;
    println!("Detected language: {}", source_lang);

    // Translate using global function
    let translation = langweave::translate("fr", text)?;
    println!("Translation to French: {}", translation);

    Ok(())
}
```

## Multiple Translations

### Translate Multiple Words

```rust
use langweave::translate;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    let words = vec!["Hello", "Goodbye", "Thank you"];

    for word in words {
        let french = translate("fr", word)?;
        let german = translate("de", word)?;
        println!("{}: {} (French), {} (German)", word, french, german);
    }

    Ok(())
}
```

### Using Different Translator Instances

```rust
use langweave::translator::Translator;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    // Create translators for different languages
    let french_translator = Translator::new("fr")?;
    let german_translator = Translator::new("de")?;

    let text = "Hello";

    // Translate using each instance
    let french = french_translator.translate(text)?;
    let german = german_translator.translate(text)?;

    println!("{}: {} (French), {} (German)", text, french, german);

    Ok(())
}
```

## Supported Languages

LangWeave currently supports basic translation between these languages:

- **en** - English
- **fr** - French
- **de** - German

### Language Codes

Use these exact codes when calling translation functions:

```rust
use langweave::translate;

// Supported language codes
let result1 = translate("fr", "Hello")?; // English to French
let result2 = translate("de", "Hello")?; // English to German
```

### Check Language Support

```rust
use langweave::is_language_supported;

// Check if language is supported
if is_language_supported("fr") {
    println!("French is supported");
}

if !is_language_supported("es") {
    println!("Spanish is not currently supported");
}
```

## Error Handling

### Handle Translation Errors

```rust
use langweave::translate;
use langweave::error::I18nError;

match translate("invalid_code", "Hello") {
    Ok(translation) => println!("Translation: {}", translation),
    Err(I18nError::UnsupportedLanguage(lang)) => {
        println!("Language not supported: {}", lang);
    }
    Err(I18nError::TranslationFailed(reason)) => {
        println!("Translation failed: {}", reason);
    }
    Err(e) => println!("Other error: {}", e),
}
```

### Using Translator with Error Handling

```rust
use langweave::translator::Translator;
use langweave::error::I18nError;

// Handle errors during translator creation
match Translator::new("invalid_lang") {
    Ok(translator) => {
        match translator.translate("Hello") {
            Ok(translation) => println!("Translation: {}", translation),
            Err(e) => println!("Translation error: {}", e),
        }
    }
    Err(I18nError::UnsupportedLanguage(lang)) => {
        println!("Cannot create translator for unsupported language: {}", lang);
    }
    Err(e) => println!("Translator creation error: {}", e),
}
```

### Graceful Fallback

```rust
use langweave::translate;

fn translate_with_fallback(text: &str, target_lang: &str) -> String {
    match translate(target_lang, text) {
        Ok(translation) => translation,
        Err(_) => {
            println!("Translation failed, using original text");
            text.to_string()
        }
    }
}

let result = translate_with_fallback("Hello", "fr");
println!("Result: {}", result);
```

## Integration Examples

### Simple CLI Tool

```rust
use langweave::translate;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <language> <text>", args[0]);
        eprintln!("Example: {} fr Hello", args[0]);
        return;
    }

    let target_lang = &args[1];
    let text = &args[2];

    match translate(target_lang, text) {
        Ok(translation) => println!("Translation: {}", translation),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Web Service Integration

```rust
use langweave::{translate, detect_language};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TranslateRequest {
    text: String,
    target_language: String,
}

#[derive(Serialize)]
struct TranslateResponse {
    original_text: String,
    detected_language: Option<String>,
    target_language: String,
    translation: String,
}

async fn translate_endpoint(
    request: TranslateRequest
) -> Result<TranslateResponse, Box<dyn std::error::Error>> {
    // Try to detect source language
    let detected_lang = detect_language(&request.text).await.ok();

    // Translate using global function
    let translation = translate(&request.target_language, &request.text)?;

    Ok(TranslateResponse {
        original_text: request.text,
        detected_language: detected_lang,
        target_language: request.target_language,
        translation,
    })
}
```

### Batch Processing

```rust
use langweave::translate;
use langweave::error::I18nError;

fn translate_multiple(texts: &[&str], target_lang: &str) -> Result<Vec<String>, I18nError> {
    let mut results = Vec::new();

    for text in texts {
        let translation = translate(target_lang, text)?;
        results.push(translation);
    }

    Ok(results)
}

fn main() -> Result<(), I18nError> {
    let texts = vec!["Hello", "Goodbye", "Thank you"];
    let translations = translate_multiple(&texts, "fr")?;

    for (original, translated) in texts.iter().zip(translations) {
        println!("{} -> {}", original, translated);
    }

    Ok(())
}
```

## Best Practices

### Text Preparation
- Use simple, clear text for best results
- Single words and simple phrases work best with the current dictionary-based approach
- Ensure text is not empty or only whitespace

### Language Support
- Always check language support using `is_language_supported()` before translation
- Use exact language codes: "en", "fr", "de"
- Handle unsupported languages gracefully with fallbacks

### Error Handling
- Always handle `I18nError::UnsupportedLanguage` for unsupported language codes
- Provide fallback content when translation fails
- Use `Result<String, I18nError>` pattern consistently

### Code Organization
- Use the global `translate()` function for simple one-off translations
- Create `Translator` instances when you need multiple translations to the same language
- Keep translator instances for reuse rather than creating them repeatedly

## Troubleshooting

### Common Issues

**UnsupportedLanguage error:**
- Verify you're using supported language codes: "en", "fr", "de"
- Check `supported_languages()` function for current list
- Use `is_language_supported()` to validate codes before translation

**Translation returns original text:**
- This is expected behavior when translation fails
- The library falls back to returning the original text
- Check if the word exists in the translation dictionary

**Empty or unexpected results:**
- Ensure input text is not empty
- The current implementation uses a simple dictionary lookup
- Complex phrases may not have direct translations

### Getting Help

For more comprehensive error handling patterns, see [Error Handling](error-handling.md).