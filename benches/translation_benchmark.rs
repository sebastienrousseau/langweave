// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#![allow(missing_docs)]

//! # Translation Benchmark for SSG I18n
//!
//! This benchmark measures the performance of the translation functionality in the `langweave` library using the `criterion` crate.
//!

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion,
};
use std::hint::black_box;
use langweave::error::I18nError;
use langweave::translator::Translator;

/// Benchmark the translation of various strings using the `langweave` library.
fn benchmark_translation(c: &mut Criterion) {
    let languages = ["fr", "de", "en"]; // Removed "es" as it's not supported
    let texts = ["Hello", "Goodbye", "Thank you"];

    let mut group = c.benchmark_group("translations");
    for lang in languages.iter() {
        match Translator::new(lang) {
            Ok(translator) => {
                for text in texts.iter() {
                    let _ = group.bench_with_input(
                        BenchmarkId::new(lang.to_string(), text),
                        text,
                        |b, text| {
                            b.iter(|| {
                                match translator
                                    .translate(black_box(text))
                                {
                                    Ok(translated) => translated,
                                    Err(_) => String::from(*text),
                                }
                            })
                        },
                    );
                }
            }
            Err(I18nError::UnsupportedLanguage(_)) => {
                println!("Skipping unsupported language: {}", lang);
            }
            Err(e) => {
                panic!(
                    "Unexpected error creating translator for {}: {:?}",
                    lang, e
                );
            }
        }
    }
    group.finish();
}

criterion_group!(benches, benchmark_translation);
criterion_main!(benches);
