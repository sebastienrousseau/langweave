# LangWeave

<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/langweave/images/logos/langweave.svg"
alt="LangWeave logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

Detect text languages. Translate between language pairs. Build internationalized Rust applications.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][09]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Status

**Experimental** — This library is under active development. API may change in future versions.

| Version | Status | Notes |
|---------|--------|-------|
| 0.0.x | Experimental | API subject to change |
| 0.1.x | Beta (planned) | API stabilization |
| 1.0.x | Stable (planned) | Stable API, semver guarantees |

## Features

- **Language Detection** — Identify text languages across 15 supported languages including English, French, German, Spanish, Portuguese, Italian, Dutch, Russian, Arabic, Hebrew, Hindi, Japanese, Korean, Chinese, and Indonesian
- **Translation Engine** — Translate between supported language pairs
- **Error Handling** — Comprehensive error types for robust applications
- **Async Support** — Non-blocking language detection and translation
- **Simple API** — Easy-to-use functions for common tasks
- **Safety** — Built with `#![forbid(unsafe_code)]` in library code

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
langweave = "0.0.2"
```

## Requirements

- **MSRV**: Rust 1.85.0 or later

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `default` | No optional features enabled | ✅ |
| `async` | Enable async utilities for non-blocking operations | ❌ |

Enable features in `Cargo.toml`:

```toml
[dependencies]
langweave = { version = "0.0.2", features = ["async"] }
```

## Quick Start

Detect language and translate text:

```rust
use langweave::{detect_language, translate};
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    // Detect language using the high-level API
    let lang = detect_language("Hello, world!")?;
    println!("Detected: {}", lang);

    // Translate text
    let translated = translate("fr", "Hello")?;
    println!("Translated: {}", translated);

    Ok(())
}
```

## Examples

Run examples:

```shell
cargo run --example <example_name>
```

## Known Limitations

- **Short text detection** — Very short texts (< 10 characters) may not have enough signal for accurate language detection
- **Mixed-language content** — Text containing multiple languages may only detect the dominant language
- **Romanized scripts** — Languages written in non-native scripts (e.g., Japanese in romaji) may not be detected correctly
- **Translation coverage** — Not all phrases have translations; the library falls back to the original text for unknown keys

## Documentation

Browse complete API reference at [docs.rs/langweave][04].

## Contributing

Read [Contributing Guidelines][05] before submitting changes.

## License

Choose either [Apache 2.0][10] or [MIT][11] license.

---

🎨 Designed by Sebastien Rousseau — <https://sebastienrousseau.com/>
🚀 Engineered with Euxis — Enterprise Unified eXecution Intelligence System — <https://euxis.co/>


[00]: https://langweave.com
[01]: https://lib.rs/crates/langweave
[02]: https://github.com/sebastienrousseau/langweave/issues
[03]: https://crates.io/crates/langweave
[04]: https://docs.rs/langweave
[05]: https://github.com/sebastienrousseau/langweave/blob/main/CONTRIBUTING.md
[06]: https://codecov.io/gh/sebastienrousseau/langweave
[07]: https://github.com/sebastienrousseau/langweave/actions?query=branch%3Amain
[08]: https://www.rust-lang.org/
[09]: https://github.com/sebastienrousseau/langweave
[10]: https://www.apache.org/licenses/LICENSE-2.0
[11]: https://opensource.org/licenses/MIT

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/langweave/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/langweave?style=for-the-badge&token=CfYfWg8UHf&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/langweave.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-langweave-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/langweave-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
