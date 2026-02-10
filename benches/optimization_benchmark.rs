//! Optimization benchmarks demonstrating zero-cost abstraction violations and fixes
//! Targets: eliminate unnecessary clones, Box<dyn> -> generics, reduce allocations

#![allow(missing_docs)]
#![allow(unused_results)]
#![allow(clippy::empty_line_after_doc_comments)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion,
};
use std::collections::HashMap;
use std::hint::black_box;
use std::sync::Arc;

/// Current implementation problems and optimized solutions

// Problem 1: supported_languages() allocates Vec<String> with clones every call
fn current_supported_languages() -> Vec<String> {
    vec!["en".to_string(), "fr".to_string(), "de".to_string()]
}

// Optimization 1: Use static slice, avoid allocations
static SUPPORTED_LANGS: &[&str] = &["en", "fr", "de"];

fn optimized_supported_languages() -> &'static [&'static str] {
    SUPPORTED_LANGS
}

// Problem 2: is_language_supported does linear search through allocated Vec
fn current_is_supported(lang: &str) -> bool {
    current_supported_languages().contains(&lang.to_lowercase())
}

// Optimization 2: Direct slice check, avoid allocation
fn optimized_is_supported(lang: &str) -> bool {
    SUPPORTED_LANGS
        .iter()
        .any(|&l| l.eq_ignore_ascii_case(lang))
}

// Problem 3: Language detector patterns cloned on every new()
#[derive(Clone)]
struct CurrentDetector {
    #[allow(dead_code)]
    patterns: Arc<Vec<(&'static str, &'static str)>>,
}

impl CurrentDetector {
    fn new() -> Self {
        static PATTERNS: &[(&str, &str)] =
            &[("hello", "en"), ("bonjour", "fr"), ("hallo", "de")];
        Self {
            patterns: Arc::new(PATTERNS.to_vec()), // Clone on every new()
        }
    }
}

// Optimization 3: Use static reference, zero allocations
struct OptimizedDetector {
    #[allow(dead_code)]
    patterns: &'static [(&'static str, &'static str)],
}

impl OptimizedDetector {
    fn new() -> Self {
        static PATTERNS: &[(&str, &str)] =
            &[("hello", "en"), ("bonjour", "fr"), ("hallo", "de")];
        Self { patterns: PATTERNS }
    }
}

// Problem 4: convert_lang_code always allocates String
fn current_convert_lang_code(code: u32) -> String {
    match code {
        1 => "en",
        2 => "fr",
        3 => "de",
        _ => "unknown",
    }
    .to_string()
}

// Optimization 4: Return static str, use Cow for flexibility
use std::borrow::Cow;

fn optimized_convert_lang_code(code: u32) -> &'static str {
    match code {
        1 => "en",
        2 => "fr",
        3 => "de",
        _ => "unknown",
    }
}

fn cow_convert_lang_code(code: u32) -> Cow<'static, str> {
    match code {
        1 => Cow::Borrowed("en"),
        2 => Cow::Borrowed("fr"),
        3 => Cow::Borrowed("de"),
        _ => Cow::Borrowed("unknown"),
    }
}

// Problem 5: Translation lookup clones strings unnecessarily
fn current_translate_lookup(
    translations: &HashMap<String, String>,
    key: &str,
) -> Option<String> {
    translations.get(key).cloned()
}

// Optimization 5: Return reference, avoid clone
fn optimized_translate_lookup<'a>(
    translations: &'a HashMap<String, String>,
    key: &str,
) -> Option<&'a str> {
    translations.get(key).map(|v| v.as_str())
}

// Problem 6: Case-insensitive search does full linear scan
fn current_case_insensitive_lookup(
    translations: &HashMap<String, String>,
    key: &str,
) -> Option<String> {
    for (k, v) in translations {
        if k.to_lowercase() == key.to_lowercase() {
            return Some(v.clone());
        }
    }
    None
}

// Optimization 6: Use HashMap with pre-lowercased keys for O(1) lookup
struct OptimizedTranslations {
    exact: HashMap<String, String>,
    lowercased: HashMap<String, String>,
}

impl OptimizedTranslations {
    fn new() -> Self {
        let mut exact = HashMap::new();
        let mut lowercased = HashMap::new();

        let data = [("Hello", "Bonjour"), ("Goodbye", "Au revoir")];
        for (k, v) in data {
            exact.insert(k.to_string(), v.to_string());
            lowercased.insert(k.to_lowercase(), v.to_string());
        }

        Self { exact, lowercased }
    }

    fn get(&self, key: &str) -> Option<&str> {
        // Try exact match first (O(1))
        if let Some(v) = self.exact.get(key) {
            return Some(v);
        }
        // Fall back to case-insensitive (O(1))
        self.lowercased.get(&key.to_lowercase()).map(|s| s.as_str())
    }
}

/// Benchmarks comparing current vs optimized implementations

fn bench_language_support_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_support_allocation");

    // Current: allocates Vec<String> with clones
    group.bench_function("current_supported_languages", |b| {
        b.iter(|| black_box(current_supported_languages()));
    });

    // Optimized: returns static slice
    group.bench_function("optimized_supported_languages", |b| {
        b.iter(|| black_box(optimized_supported_languages()));
    });

    // Test language checking
    let test_langs = ["en", "fr", "de", "es", "invalid"];
    for lang in &test_langs {
        group.bench_with_input(
            BenchmarkId::new("current_is_supported", lang),
            lang,
            |b, lang| b.iter(|| current_is_supported(black_box(lang))),
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_is_supported", lang),
            lang,
            |b, lang| {
                b.iter(|| optimized_is_supported(black_box(lang)))
            },
        );
    }

    group.finish();
}

fn bench_detector_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("detector_allocation");

    // Current: clones patterns Vec on every new()
    group.bench_function("current_detector_new", |b| {
        b.iter(|| black_box(CurrentDetector::new()));
    });

    // Optimized: zero allocation
    group.bench_function("optimized_detector_new", |b| {
        b.iter(|| black_box(OptimizedDetector::new()));
    });

    group.finish();
}

fn bench_string_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_allocation");

    let test_codes = [1, 2, 3, 999];
    for code in &test_codes {
        group.bench_with_input(
            BenchmarkId::new("current_convert", code),
            code,
            |b, code| {
                b.iter(|| current_convert_lang_code(black_box(*code)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_convert", code),
            code,
            |b, code| {
                b.iter(|| optimized_convert_lang_code(black_box(*code)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("cow_convert", code),
            code,
            |b, code| {
                b.iter(|| cow_convert_lang_code(black_box(*code)))
            },
        );
    }

    group.finish();
}

fn bench_translation_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_lookup");

    // Setup test data
    let mut translations = HashMap::new();
    translations.insert("Hello".to_string(), "Bonjour".to_string());
    translations.insert("Goodbye".to_string(), "Au revoir".to_string());

    let optimized = OptimizedTranslations::new();

    let test_keys = ["Hello", "hello", "HELLO", "Goodbye", "NotFound"];

    for key in &test_keys {
        group.bench_with_input(
            BenchmarkId::new("current_exact_lookup", key),
            key,
            |b, key| {
                b.iter(|| {
                    current_translate_lookup(
                        black_box(&translations),
                        black_box(key),
                    )
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_exact_lookup", key),
            key,
            |b, key| {
                b.iter(|| {
                    optimized_translate_lookup(
                        black_box(&translations),
                        black_box(key),
                    )
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("current_case_insensitive", key),
            key,
            |b, key| {
                b.iter(|| {
                    current_case_insensitive_lookup(
                        black_box(&translations),
                        black_box(key),
                    )
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_case_insensitive", key),
            key,
            |b, key| b.iter(|| optimized.get(black_box(key))),
        );
    }

    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // Test memory allocation patterns
    group.bench_function("vec_vs_slice_iteration", |b| {
        let vec_data =
            ["en".to_string(), "fr".to_string(), "de".to_string()];
        let slice_data = ["en", "fr", "de"];

        b.iter(|| {
            // Current: iterate over Vec<String>
            let vec_count = vec_data.len();
            // Optimized: iterate over &[&str]
            let slice_count = slice_data.iter().count();
            black_box((vec_count, slice_count))
        });
    });

    // ArrayVec vs Vec for small collections
    group.bench_function("arrayvec_vs_vec", |b| {
        b.iter(|| {
            // Current: heap allocation
            let vec_langs: Vec<&str> = vec!["en", "fr", "de"];

            // Optimized: stack allocation (simulated with array)
            let array_langs: [&str; 3] = ["en", "fr", "de"];

            black_box((vec_langs.len(), array_langs.len()))
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_language_support_allocation,
    bench_detector_allocation,
    bench_string_allocation,
    bench_translation_lookup,
    bench_memory_usage
);
criterion_main!(benches);
