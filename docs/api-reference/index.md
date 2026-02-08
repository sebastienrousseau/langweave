# (c) 2026 Euxis Fleet. All rights reserved.

# API Reference

Welcome to the LangWeave API Reference. This section provides detailed documentation for all public APIs, types, and traits.

## Core Modules

### [`langweave::language_detector`](language_detector.md)
Language detection functionality with configurable confidence thresholds and multiple detection strategies.

**Key Types:**
- [`LanguageDetector`](language_detector.md#languagedetector) - Main language detection implementation
- [`LanguageDetectorTrait`](language_detector.md#languagedetectortrait) - Common interface for language detectors

### [`langweave::translator`](translator.md)
Text translation between multiple language pairs with quality assessment and batch processing.

**Key Types:**
- [`Translator`](translator.md#translator) - Main translation implementation
- [`TranslationOptions`](translator.md#translationoptions) - Configuration for translation behavior
- [`TranslationContext`](translator.md#translationcontext) - Contextual hints for better translations

### [`langweave::translations`](translations.md)
Localized content management and retrieval system.

**Key Types:**
- [`Translations`](translations.md#translations) - Content management system
- [`LocalizedContent`](translations.md#localizedcontent) - Individual localized content items

### [`langweave::error`](error.md)
Comprehensive error handling for all LangWeave operations.

**Key Types:**
- [`I18nError`](error.md#i18nerror) - Main error enumeration
- [`ErrorKind`](error.md#errorkind) - Error categorization

## Quick Reference

### Common Patterns

```rust
use langweave::{
    language_detector::{LanguageDetector, LanguageDetectorTrait},
    translator::Translator,
    translate,
    detect_language,
    error::I18nError
};

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    // Language Detection
    let detector = LanguageDetector::new();
    let language = detector.detect_async("Hello, world!").await?;

    // Or use global function
    let language = detect_language("Hello, world!").await?;

    // Translation using global function
    let translation = translate("fr", "Hello")?;

    // Translation using Translator instance
    let translator = Translator::new("fr")?;
    let translation = translator.translate("Hello")?;

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
langweave = { version = "0.0.1", features = ["full"] }

# Or enable specific features:
langweave = { version = "0.0.1", features = ["translation", "caching"] }
```

**Available Features:**
- `translation` - Translation functionality (enabled by default)
- `detection` - Language detection (enabled by default)
- `caching` - Result caching for better performance
- `batch` - Batch processing capabilities
- `async` - Async/await support (enabled by default)
- `full` - All features enabled

## Type Hierarchy

```
langweave
├── language_detector
│   ├── LanguageDetector
│   └── LanguageDetectorTrait
├── translator
│   ├── Translator
│   ├── TranslationOptions
│   └── TranslationContext
├── translations
│   ├── Translations
│   └── LocalizedContent
└── error
    ├── I18nError
    └── ErrorKind
```

## Compatibility

### Rust Version Support

LangWeave supports Rust 1.70.0 and later. We follow the Rust release train and support:

- **Current stable** - Full support with all features
- **Previous stable** - Support for core functionality
- **MSRV (1.70.0)** - Basic functionality guaranteed

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

### Batch Processing
```rust
// See examples/batch_processing_example.rs
use langweave::translate;

fn translate_multiple(texts: &[&str], target_lang: &str) -> Result<Vec<String>, langweave::error::I18nError> {
    texts.iter()
        .map(|text| translate(target_lang, text))
        .collect()
}
```

## Migration Guides

### From 0.0.1 to 0.1.0
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