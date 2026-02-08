# LangWeave Documentation

Master LangWeave internationalization and localization.

## Structure

Documentation follows **Topic-Task-Reference** hierarchy:

### Essentials
Core concepts and setup.

- **[Overview](essentials/overview.md)** — Capabilities and components
- **[Quick Start](essentials/quick-start.md)** — Install and run

### Guides
Implementation and best practices.

- **[Language Detection](guides/language-detection.md)** — Detect text languages
- **[Translation](guides/translation.md)** — Translate between languages
- **[Error Handling](guides/error-handling.md)** — Handle failures
- **[Contributing](guides/contributing.md)** — Contribute code

### API Reference
Technical documentation.

- **[API Reference](api-reference/index.md)** — Complete APIs
- **[Language Detector](api-reference/language_detector.md)** — Detection APIs
- **[Translator](api-reference/translator.md)** — Translation APIs
- **[Error Types](api-reference/error.md)** — Error handling

## Quick Access

### Common Tasks

**Start now** → [Quick Start](essentials/quick-start.md)
**Learn features** → [Overview](essentials/overview.md)
**Detect language** → [Language Detection](guides/language-detection.md)
**Translate text** → [Translation](guides/translation.md)
**Handle errors** → [Error Handling](guides/error-handling.md)
**Contribute** → [Contributing](guides/contributing.md)
**Browse APIs** → [API Reference](api-reference/index.md)

### Project Types

**Web applications** → [Web Integration](guides/translation.md#web-application-integration)
**CLI tools** → [CLI Integration](guides/language-detection.md#cli-tool-integration)
**Batch processing** → [Batch Translation](guides/translation.md#batch-translation)
**Error recovery** → [Recovery Strategies](guides/error-handling.md#error-recovery-strategies)

## Workflows

### Language Detection
1. [Install LangWeave](essentials/quick-start.md#installation)
2. [Create detector](guides/language-detection.md#basic-detection)
3. [Configure thresholds](guides/language-detection.md#advanced-configuration)
4. [Handle errors](guides/error-handling.md#language-detection-errors)

### Translation
1. [Setup translator](guides/translation.md#basic-translation)
2. [Configure options](guides/translation.md#translation-configuration)
3. [Add error handling](guides/error-handling.md#translation-errors)
4. [Optimize performance](guides/translation.md#performance-optimization)

### Integration
1. [Review APIs](api-reference/index.md)
2. [Add error handling](guides/error-handling.md)
3. [Build features](guides/)
4. [Test code](guides/contributing.md#writing-tests)

## Code Examples

### Basic Usage
```rust
use langweave::{
    language_detector::{LanguageDetector, LanguageDetectorTrait},
    translator::Translator,
    error::I18nError
};

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    // Detect language
    let detector = LanguageDetector::new();
    let language = detector.detect_async("Bonjour le monde!").await?;
    println!("Detected: {}", language); // "fr"

    // Translate text
    let translator = Translator::new("fr")?;
    let translation = translator.translate("Hello")?;
    println!("Translation: {}", translation); // "Bonjour"

    Ok(())
}
```

### Error Handling
```rust
use langweave::{language_detector::LanguageDetector, error::I18nError};

async fn safe_detection(text: &str) -> Result<String, String> {
    let detector = LanguageDetector::new();

    match detector.detect_async(text).await {
        Ok(language) => Ok(language),
        Err(I18nError::EmptyInput) => {
            Err("Cannot detect language of empty text".to_string())
        }
        Err(I18nError::ConfidenceTooLow) => {
            Err("Detection confidence too low".to_string())
        }
        Err(e) => Err(format!("Detection failed: {}", e)),
    }
}
```

## External Resources

- **[GitHub Repository](https://github.com/sebastienrousseau/langweave)** — Source and issues
- **[crates.io](https://crates.io/crates/langweave)** — Package registry
- **[docs.rs](https://docs.rs/langweave)** — API reference
- **[Examples](https://github.com/sebastienrousseau/langweave/tree/main/examples)** — Code samples

## Get Help

Find answers:

1. **Read guides** — Common questions answered
2. **Check examples** — Working code samples
3. **Search issues** — Previous questions
4. **File issue** — New problems

### Report Problems

Report issues:

- **Bugs** → [GitHub Issues](https://github.com/sebastienrousseau/langweave/issues)
- **Features** → [GitHub Discussions](https://github.com/sebastienrousseau/langweave/discussions)
- **Questions** → [GitHub Discussions](https://github.com/sebastienrousseau/langweave/discussions)

## Contributing

Help build LangWeave. Read the [Contributing Guide](guides/contributing.md):

- **Report bugs** — Find and fix issues
- **Request features** — Suggest improvements
- **Submit code** — Add fixes and features
- **Update docs** — Improve guides
- **Add languages** — Expand support

---

🎨 Designed by Sebastien Rousseau — https://sebastienrousseau.com/
🚀 Engineered with Euxis — Enterprise Unified eXecution Intelligence System — https://euxis.co/