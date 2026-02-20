// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Comprehensive performance benchmarks for LangWeave.
//!
//! This benchmark suite provides detailed performance testing
//! for all major LangWeave library operations.

#![allow(unused_results)]
#![allow(missing_docs)]

use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use langweave::{
    is_language_supported, language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    supported_languages, translate, translator::Translator,
};
use std::hint::black_box;
use std::sync::Arc;

/// Benchmark language detection performance
fn bench_language_detection(c: &mut Criterion) {
    let detector = LanguageDetector::new();
    let texts = [
        "The quick brown fox jumps over the lazy dog",
        "Le chat noir saute par-dessus le chien paresseux",
        "Der schnelle braune Fuchs springt über den faulen Hund",
        "Hello world how are you today",
        "Bonjour le monde comment allez-vous",
        "Hallo Welt wie geht es dir heute",
    ];

    let mut group = c.benchmark_group("language_detection");

    // Sync detection
    for (i, text) in texts.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("sync", i),
            text,
            |b, text| {
                b.iter(|| detector.detect(black_box(text)));
            },
        );
    }

    // Async detection (simplified for criterion 0.8)
    for (i, text) in texts.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("async", i),
            text,
            |b, text| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                b.iter(|| {
                    rt.block_on(detector.detect_async(black_box(text)))
                });
            },
        );
    }

    group.finish();
}

/// Benchmark supported languages operations
fn bench_language_support(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_support");

    // Benchmark supported_languages() - this allocates Vec<String>
    group.bench_function("supported_languages_allocation", |b| {
        b.iter(|| black_box(supported_languages()));
    });

    // Benchmark is_language_supported - this calls supported_languages internally
    let languages = vec!["en", "fr", "de", "es", "invalid"];
    for lang in &languages {
        group.bench_with_input(
            BenchmarkId::new("is_supported", *lang),
            lang,
            |b, lang| {
                b.iter(|| is_language_supported(black_box(lang)));
            },
        );
    }

    group.finish();
}

/// Benchmark translation performance with different strategies
fn bench_translation_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("translation_strategies");

    // Benchmark creating new translator each time (current pattern)
    group.bench_function("translator_new_each_call", |b| {
        b.iter(|| {
            let translator = Translator::new(black_box("fr")).unwrap();
            translator.translate(black_box("Hello")).unwrap()
        });
    });

    // Benchmark reusing translator instance
    let translator = Translator::new("fr").unwrap();
    group.bench_function("translator_reuse", |b| {
        b.iter(|| translator.translate(black_box("Hello")).unwrap());
    });

    // Direct translation call
    group.bench_function("direct_translate", |b| {
        b.iter(|| {
            translate(black_box("fr"), black_box("Hello")).unwrap()
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocations");

    // String operations that could be optimized
    let detector = LanguageDetector::new();

    // Test regex pattern cloning (happens in LanguageDetector::new)
    group.bench_function("detector_new_clone_patterns", |b| {
        b.iter(LanguageDetector::new);
    });

    // Test convert_lang_code string allocation
    group.bench_function("convert_lang_code_allocation", |b| {
        b.iter(|| {
            detector.convert_lang_code(black_box(whatlang::Lang::Eng))
        });
    });

    group.finish();
}

/// Benchmark with concurrent access patterns
fn bench_concurrent_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_access");

    let detector = Arc::new(LanguageDetector::new());

    // Single-threaded baseline
    group.bench_function("single_thread", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(detector.detect_async(black_box("Hello world")))
        });
    });

    // Sync multi-clone access pattern
    group.throughput(Throughput::Elements(10));
    group.bench_function("multi_clone", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for _ in 0..10 {
                let detector = Arc::clone(&detector);
                results.push(detector.detect(black_box("Hello world")));
            }
            results
        });
    });

    group.finish();
}

/// Benchmark compile-time patterns
fn bench_compile_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("compile_patterns");

    // Benchmark regex compilation costs (simulated)
    group.bench_function("regex_heavy_pattern", |b| {
        b.iter(|| {
            regex::Regex::new(black_box(
                r"(?i)\b(hello|hi|hey|goodbye|bye|thank you|thanks|please|the|a|an|in|on|at|for|to|of)\b"
            )).unwrap()
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_language_detection,
    bench_language_support,
    bench_translation_strategies,
    bench_allocations,
    bench_concurrent_access,
    bench_compile_patterns
);
criterion_main!(benches);
