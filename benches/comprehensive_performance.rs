// SPDX-License-Identifier: Apache-2.0 OR MIT
// Comprehensive performance benchmark for langweave

#![allow(unused_results)]
#![allow(missing_docs)]

use criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
};
use langweave::{
    is_language_supported, language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    supported_languages, translator::Translator,
};
use std::hint::black_box;

/// Benchmark the inefficient supported_languages() function
fn bench_supported_languages(c: &mut Criterion) {
    c.bench_function("supported_languages", |b| {
        b.iter(|| black_box(supported_languages()))
    });
}

/// Benchmark the inefficient is_language_supported() function
fn bench_is_language_supported(c: &mut Criterion) {
    let langs = ["en", "fr", "de", "es", "invalid"];

    let mut group = c.benchmark_group("is_language_supported");
    for lang in langs.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(lang),
            lang,
            |b, lang| b.iter(|| black_box(is_language_supported(lang))),
        );
    }
    group.finish();
}

/// Benchmark translator creation (which validates language support)
fn bench_translator_creation(c: &mut Criterion) {
    let langs = ["en", "fr", "de"];

    let mut group = c.benchmark_group("translator_creation");
    for lang in langs.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(lang),
            lang,
            |b, lang| b.iter(|| black_box(Translator::new(lang))),
        );
    }
    group.finish();
}

/// Benchmark language detection with various text sizes
fn bench_language_detection(c: &mut Criterion) {
    let detector = LanguageDetector::new();

    let long_text =
        "The quick brown fox jumps over the lazy dog. ".repeat(50);
    let test_cases = [
        ("short_en", "Hello world"),
        (
            "medium_en",
            "The quick brown fox jumps over the lazy dog. This is a longer sentence to test detection.",
        ),
        ("long_en", long_text.as_str()),
        ("short_fr", "Bonjour monde"),
        (
            "medium_fr",
            "Le chat noir mange une souris blanche dans le jardin.",
        ),
        ("mixed", "Hello bonjour hola guten tag"),
    ];

    let mut group = c.benchmark_group("language_detection_sync");
    for (name, text) in test_cases.iter() {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            text,
            |b, text| b.iter(|| black_box(detector.detect(text))),
        );
    }
    group.finish();
}

/// Benchmark case-insensitive translation fallback (worst case)
fn bench_translation_fallback(c: &mut Criterion) {
    let translator = Translator::new("fr").unwrap();
    let test_cases = [
        ("exact_match", "Hello"),
        ("case_mismatch", "HELLO"), // Forces case-insensitive search
        ("not_found", "NonexistentKey"),
    ];

    let mut group = c.benchmark_group("translation_fallback");
    for (name, key) in test_cases.iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            key,
            |b, key| b.iter(|| black_box(translator.translate(key))),
        );
    }
    group.finish();
}

/// Benchmark word-by-word fallback detection (worst case)
fn bench_word_by_word_detection(c: &mut Criterion) {
    let detector = LanguageDetector::new();

    // Create text that won't match patterns but will trigger word-by-word fallback
    let problematic_texts = [
        ("numbers_and_en", "123 hello 456 world"),
        ("mixed_scripts", "hello مرحبا 你好"),
        (
            "long_mixed",
            "word1 word2 word3 word4 word5 hello world test case example",
        ),
    ];

    let mut group = c.benchmark_group("word_by_word_detection");
    for (name, text) in problematic_texts.iter() {
        group.throughput(Throughput::Bytes(text.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            text,
            |b, text| b.iter(|| black_box(detector.detect(text))),
        );
    }
    group.finish();
}

/// Memory allocation benchmark
fn bench_memory_allocations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocations");

    // Benchmark string allocations in hot paths
    group.bench_function("supported_languages_vec", |b| {
        b.iter(|| {
            // This creates a new Vec<String> on every call
            black_box(supported_languages())
        })
    });

    group.bench_function("language_validation_chain", |b| {
        b.iter(|| {
            // Chain of allocations: supported_languages() + contains() + to_lowercase()
            black_box(is_language_supported("FR"))
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_supported_languages,
    bench_is_language_supported,
    bench_translator_creation,
    bench_language_detection,
    bench_translation_fallback,
    bench_word_by_word_detection,
    bench_memory_allocations
);
criterion_main!(benches);
