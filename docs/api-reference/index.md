# (c) 2026 Euxis Fleet. All rights reserved.

# API Reference

Welcome to the LangWeave API Reference. This section provides detailed documentation for all public APIs, types, and traits.

## Core Modules

### [`langweave::language_detector`](language_detector.md)
Language detection functionality for all supported languages (English, French, German, Spanish, Portuguese, Italian, Dutch, Russian, Arabic, Hebrew, Hindi, Japanese, Korean, Chinese, and Indonesian).

**Key Types:**
- [`LanguageDetector`](language_detector.md#languagedetector) - Main language detection implementation
- [`LanguageDetectorTrait`](language_detector.md#languagedetectortrait) - Common interface for language detectors

### [`langweave::translator`](translator.md)
Text translation between supported language pairs across all supported languages.

**Key Types:**
- [`Translator`](translator.md#translator) - Main translation implementation

### [`langweave::translations`](translations.md)
Translation functions and utilities for supported languages.

### [`langweave::error`](error.md)
Error handling for all LangWeave operations.

**Key Types:**
- [`I18nError`](error.md#i18nerror) - Main error enumeration

## Quick Reference

### Common Patterns

```rust
use langweave::{translate, detect_language, error::I18nError};

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    // Language Detection using high-level API
    let language = detect_language("Hello, world!").await?;
    println!("Detected: {}", language);

    // Translation using high-level API
    let translation = translate("fr", "Hello")?;
    println!("Translation: {}", translation);

    Ok(())
}

// Error Handling
match translate("invalid_lang", "Hello") {
    Ok(result) => println!("Success: {}", result),
    Err(I18nError::UnsupportedLanguage(lang)) => println!("Unsupported language: {}", lang),
    Err(e) => println!("Error: {}", e),
}
```

### Feature Flags

LangWeave supports optional features that can be enabled in your `Cargo.toml`:

```toml
[dependencies]
langweave = "0.0.2"

# Optional async support
langweave = { version = "0.0.2", features = ["async"] }
```

**Available Features:**
- `async` - Async/await support

## Type Hierarchy

```
langweave
├── language_detector
│   ├── LanguageDetector
│   └── LanguageDetectorTrait
├── translator
│   └── Translator
├── translations
│   └── (translation functions)
└── error
    └── I18nError
```

## Compatibility

### Rust Version Support

LangWeave supports Rust 1.87.0 and later. We follow the Rust release train and support:

- **Current stable** - Full support with all features
- **Previous stable** - Support for core functionality
- **MSRV (1.87.0)** - Basic functionality guaranteed

### Platform Support

LangWeave is tested on:

- **Linux** (x86_64, aarch64)
- **macOS** (x86_64, Apple Silicon)
- **Windows** (x86_64)
- **WebAssembly** (wasm32-unknown-unknown)

### Async Runtime Support

LangWeave is runtime-agnostic and works with:

- **Tokio** (recommended)
- **async-std**
- **smol**
- Any runtime that supports `std::future::Future`

## Performance Characteristics

### Language Detection
- **Small text (< 50 chars)**: ~1-5ms
- **Medium text (50-500 chars)**: ~5-15ms
- **Large text (> 500 chars)**: ~10-50ms

### Translation
- **Single word**: ~50-200ms
- **Sentence (< 100 chars)**: ~100-500ms
- **Paragraph (< 1000 chars)**: ~200-1000ms

*Performance varies based on language pairs, text complexity, and system resources.*

### Memory Usage
- **Base library**: ~2-5 MB
- **Language models**: ~10-50 MB per language
- **Runtime overhead**: ~1-10 MB depending on cache settings

## Examples by Use Case

### Web Application
```rust
// See examples/web_server_example.rs
use langweave::detect_language;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = "Hello, world!";
    let language = detect_language(text).await?;
    println!("Detected: {}", language);
    Ok(())
}
```

### CLI Tool
```rust
// See examples/cli_tool_example.rs
use langweave::translate;
use clap::Parser;

#[derive(Parser)]
struct Args {
    target_lang: String,
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let result = translate(&args.target_lang, &args.text)?;
    println!("Translation: {}", result);
    Ok(())
}
```

### Multiple Translations
```rust
use langweave::translate;

fn translate_multiple(texts: &[&str], target_lang: &str) -> Result<Vec<String>, langweave::error::I18nError> {
    texts.iter()
        .map(|text| translate(target_lang, text))
        .collect()
}
```

## Migration Guides

### From 0.0.1 to 0.0.2
*Coming soon*

## Contributing to API Documentation

Help improve this documentation:

1. **API Coverage**: Ensure all public APIs are documented
2. **Examples**: Add practical examples for complex APIs
3. **Error Cases**: Document when functions return errors
4. **Performance**: Add performance notes where relevant

See the [Contributing Guide](../guides/contributing.md) for details on how to contribute.

## External Resources

- **docs.rs**: [Complete API documentation](https://docs.rs/langweave)
- **GitHub**: [Source code and examples](https://github.com/sebastienrousseau/langweave)
- **Crates.io**: [Package information](https://crates.io/crates/langweave)
