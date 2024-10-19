<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/langweave/images/logos/langweave.svg"
alt="LangWeave logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# `LangWeave`

A powerful Rust library for seamless internationalization and localization.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][08] [![Crates.io][crates-badge]][03] [![lib.rs][libs-badge]][01] [![Docs.rs][docs-badge]][04] [![Codecov][codecov-badge]][06] [![Build Status][build-badge]][07] [![GitHub][github-badge]][09]

• [Website][00] • [Documentation][04] • [Report Bug][02] • [Request Feature][02] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

`langweave` is a robust Rust library that provides efficient internationalization and localization capabilities. Designed for applications requiring multi-language support, it offers fast language detection, fluid translations, and intuitive multilingual content management.

## Features

- **Instant Language Detection:** Quickly identify the language of input text.
- **Efficient Translation:** Translate text between multiple languages.
- **Flexible Content Management:** Easily manage and retrieve localized content.
- **Performance Optimized:** Utilizes efficient algorithms for fast processing.
- **Comprehensive Language Support:** Handles a wide range of languages, including non-Latin scripts.
- **Error Handling:** Robust error management for reliable operation.

## Installation

Add `langweave` to your `Cargo.toml`:

```toml
[dependencies]
langweave = "0.0.1"
```

## Usage

Here's a basic example of how to use `langweave`:

```rust
use langweave::language_detector::LanguageDetector;
use langweave::error::I18nError;

fn main() -> Result<(), I18nError> {
    // Create a new language detector
    let detector = LanguageDetector::new();

    // Detect language
    let lang = detector.detect("Hello, world!")?;
    println!("Detected language: {}", lang);

    // Use the detected language for further processing
    // (e.g., translation, localization)

    Ok(())
}
```

This example demonstrates how to use LangWeave to detect the language of a given text.

## Documentation

For full API documentation, please visit [docs.rs/langweave][04].

## Examples

To explore more examples, clone the repository and run the following command:

```shell
cargo run --example example_name
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- [Apache License, Version 2.0][10]
- [MIT license][11]

at your option.

## Acknowledgements

Special thanks to all contributors who have helped build the `langweave` library.


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
