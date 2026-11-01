# Quick Start

Install and run LangWeave.

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
langweave = "0.0.4"
```

## Basic Usage

### Detect Language

```rust
use langweave::detect_language;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let lang = detect_language("Hello, world!").await?;
    println!("Detected language: {}", lang);

    Ok(())
}
```

### Translate Text

```rust
use langweave::translate;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    let translation = translate("fr", "Hello")?;
    println!("Translation: {}", translation);

    Ok(())
}
```

### Complete Example

```rust
use langweave::{detect_language, translate};
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let text = "Hello";

    // Detect language
    let detected = detect_language(text).await?;
    println!("Detected language: {}", detected);

    // Translate to French
    let french = translate("fr", text)?;
    println!("French: {}", french);

    // Translate to German
    let german = translate("de", text)?;
    println!("German: {}", german);

    Ok(())
}
```

## Next Steps

Explore LangWeave:

- **[Language Detection](../guides/language-detection.md)** — Advanced detection
- **[Translation](../guides/translation.md)** — Text translation
- **[Error Handling](../guides/error-handling.md)** — Error management
- **[Examples](../../examples/)** — Working code
- **[API Reference](../api-reference/)** — Complete documentation

## Support

- **Issues**: [Report bugs](https://github.com/sebastienrousseau/langweave/issues)
- **Contributing**: [Development guide](../guides/contributing.md)

---

🎨 Designed by Sebastien Rousseau — https://sebastienrousseau.com/
🚀 Engineered with Euxis — Enterprise Unified eXecution Intelligence System — https://euxis.co/
