# Language Detection

Detect text languages using pattern-based analysis.

## Overview

`LanguageDetector` identifies text languages using async methods based on character patterns and word analysis.

## Basic Detection

### Detect Languages

```rust
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let detector = LanguageDetector::new();

    // Asynchronous detection
    let language = detector.detect_async("Hello world").await?;
    println!("Detected: {}", language); // "en"

    let language = detector.detect_async("Bonjour le monde").await?;
    println!("Detected: {}", language); // "fr"

    let language = detector.detect_async("Guten Tag").await?;
    println!("Detected: {}", language); // "de"

    Ok(())
}
```

### Using Global Function

```rust
use langweave::detect_language;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    // Simple detection using global function
    let language = detect_language("Hello world").await?;
    println!("Detected: {}", language); // "en"

    Ok(())
}
```

## Supported Languages

LangWeave can detect these languages:

- **en** - English
- **fr** - French
- **de** - German

### Detection Method

The detector uses pattern-based analysis:
- Analyzes character patterns and common words
- Falls back to word-by-word detection if full text detection fails
- Uses regex patterns to identify language-specific characteristics

## Multiple Text Detection

### Process Multiple Texts

```rust
use langweave::detect_language;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let texts = vec![
        "Hello world",
        "Bonjour le monde",
        "Guten Tag",
    ];

    for text in texts {
        match detect_language(text).await {
            Ok(language) => println!("{}: {}", text, language),
            Err(e) => println!("{}: Error - {}", text, e),
        }
    }

    Ok(())
}
```

### Using Detector Instance

```rust
use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let detector = LanguageDetector::new();

    let texts = vec![
        "The quick brown fox",
        "Le chat noir",
        "Der schnelle Fuchs"
    ];

    for text in texts {
        let language = detector.detect_async(text).await?;
        println!("{}: {}", text, language);
    }

    Ok(())
}
```

## Error Handling

Handle detection errors:

```rust
use langweave::detect_language;
use langweave::error::I18nError;

#[tokio::main]
async fn main() {
    match detect_language("").await {
        Ok(language) => println!("Detected: {}", language),
        Err(I18nError::LanguageDetectionFailed) => {
            println!("Cannot detect language for this text");
        }
        Err(e) => println!("Detection error: {}", e),
    }
}
```

### Handling Mixed or Ambiguous Text

```rust
use langweave::detect_language;

#[tokio::main]
async fn main() {
    let text = "Hello bonjour";

    match detect_language(text).await {
        Ok(language) => {
            println!("Detected primary language: {}", language);
        }
        Err(_) => {
            println!("Could not reliably detect language");
        }
    }
}
```

## Best Practices

### Input Text Guidelines
- **Minimum** — A few words for basic detection
- **Optimal** — Complete sentences for best accuracy
- **Empty text** — Will return `LanguageDetectionFailed` error

### Text Quality
- Clean text performs better than symbol-heavy text
- Mixed languages return the first detected language
- Numeric-only text may fail detection

### Performance Tips
- Use async methods since detection is async
- Handle errors gracefully for user-facing applications
- Consider fallback strategies for failed detection

## Integration Examples

### Simple CLI Tool

```rust
use langweave::detect_language;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <text>", args[0]);
        return;
    }

    match detect_language(&args[1]).await {
        Ok(language) => {
            println!("Input: {}", args[1]);
            println!("Detected: {}", language);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Web Service Endpoint

```rust
use langweave::detect_language;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct DetectRequest {
    text: String,
}

#[derive(Serialize)]
struct DetectResponse {
    text: String,
    language: Option<String>,
    error: Option<String>,
}

async fn detect_endpoint(request: DetectRequest) -> DetectResponse {
    match detect_language(&request.text).await {
        Ok(language) => DetectResponse {
            text: request.text,
            language: Some(language),
            error: None,
        },
        Err(e) => DetectResponse {
            text: request.text,
            language: None,
            error: Some(e.to_string()),
        },
    }
}
```

## Troubleshooting

### Common Issues

**LanguageDetectionFailed error:**
- Ensure text is not empty or only whitespace
- Try with longer, more representative text samples
- Check that text contains actual words, not just numbers/symbols

**Unexpected language detection:**
- Mixed language text may detect the first recognizable language
- Short text samples may be ambiguous
- Consider the pattern-based nature of detection

**Performance concerns:**
- Detection is async and designed for reasonable performance
- For very large volumes, consider caching results for identical text

For more error handling patterns, see [Error Handling](error-handling.md).