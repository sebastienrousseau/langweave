//! # Memory Stress Test for LangWeave
//!
//! Tests memory usage patterns, allocation patterns, and leaks under sustained load.

#![allow(unused_results)]
#![allow(missing_docs)]

use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use langweave::{
    language_detector::LanguageDetector,
    language_detector_trait::LanguageDetectorTrait,
    supported_languages, translate,
};
use std::hint::black_box;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;

/// Test for memory leaks during sustained operations
fn bench_memory_leak_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_leak_detection");

    // Test: Repeated detector creation (should not accumulate)
    group.bench_function("repeated_detector_creation", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let detector = LanguageDetector::new();
                let _ = black_box(detector);
            }
        });
    });

    // Test: Repeated language support checks (check for Vec allocation leaks)
    group.bench_function("repeated_language_checks", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = black_box(supported_languages());
            }
        });
    });

    group.finish();
}

/// Test memory usage under concurrent access
fn bench_concurrent_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_memory");

    let rt = Runtime::new().unwrap();
    let detector = Arc::new(LanguageDetector::new());

    // Test concurrent access patterns that might cause memory fragmentation
    for &thread_count in &[4, 8, 16, 32] {
        group.bench_with_input(
            BenchmarkId::new("concurrent_detection", thread_count),
            &thread_count,
            |b, &thread_count| {
                b.iter(|| {
                    rt.block_on(async {
                        let detector = detector.clone();
                        let mut handles =
                            Vec::with_capacity(thread_count);

                        for i in 0..thread_count {
                            let detector = detector.clone();
                            handles.push(tokio::spawn(async move {
                                // Different texts to avoid caching effects
                                let texts = [
                                    "Hello world",
                                    "Bonjour monde",
                                    "Hallo Welt",
                                    "Hola mundo",
                                ];
                                let text = texts[i % texts.len()];
                                detector.detect_async(text).await
                            }));
                        }

                        for handle in handles {
                            let _ = handle.await;
                        }
                    })
                })
            },
        );
    }

    group.finish();
}

/// Test large input handling for memory efficiency
fn bench_large_input_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_input_memory");

    let detector = LanguageDetector::new();
    let base_text = "The quick brown fox jumps over the lazy dog. ";

    // Test with increasingly large inputs to check for linear memory growth
    for &multiplier in &[100, 1000, 10000] {
        let large_text = base_text.repeat(multiplier);
        let text_size = large_text.len();

        group.throughput(Throughput::Bytes(text_size as u64));
        group.bench_with_input(
            BenchmarkId::new(
                "large_text",
                format!("{}KB", text_size / 1024),
            ),
            &large_text,
            |b, text| {
                b.iter(|| {
                    let _ = detector.detect(black_box(text));
                });
            },
        );
    }

    group.finish();
}

/// Test for allocation hotspots and potential optimizations
fn bench_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_patterns");

    // Baseline: Direct string operations without library overhead
    group.bench_function("baseline_string_operations", |b| {
        b.iter(|| {
            let text = black_box("Hello world");
            let _trimmed = text.trim();
            let _lowercase = text.to_lowercase();
            let _words: Vec<&str> = text.split_whitespace().collect();
        });
    });

    // Library: Language detection path (shows allocation overhead)
    group.bench_function("library_detection_allocations", |b| {
        let detector = LanguageDetector::new();
        b.iter(|| {
            let text = black_box("Hello world");
            let _ = detector.detect(text);
        });
    });

    // Compare translation allocation patterns
    group.bench_function("translation_allocations", |b| {
        b.iter(|| {
            for key in ["Hello", "Goodbye", "Thank you"] {
                let _ = translate(black_box("fr"), black_box(key));
            }
        });
    });

    group.finish();
}

/// Test memory usage during regex matching (potential hotspot)
fn bench_regex_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("regex_memory");

    let detector = LanguageDetector::new();

    // Test texts that will hit different regex patterns
    let test_cases = [
        ("english_patterns", "Hello world thank you please"),
        ("french_patterns", "Bonjour monde merci s'il vous plaît"),
        ("german_patterns", "Hallo Welt danke bitte"),
        ("mixed_patterns", "Hello bonjour hallo"),
        ("no_patterns", "xyz 123 !@# nonsense text"),
    ];

    for (name, text) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                // Run detection multiple times to stress regex engine
                for _ in 0..100 {
                    let _ = detector.detect(black_box(text));
                }
            });
        });
    }

    group.finish();
}

/// Test memory behavior during thread contention
fn bench_thread_memory_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_memory_contention");

    // Test shared Arc usage under thread contention
    group.bench_function("shared_detector_contention", |b| {
        let detector = Arc::new(LanguageDetector::new());

        b.iter(|| {
            let handles: Vec<_> = (0..8)
                .map(|i| {
                    let detector = detector.clone();
                    thread::spawn(move || {
                        let texts =
                            ["Hello", "Bonjour", "Hallo", "Hola"];
                        let text = texts[i % texts.len()];
                        for _ in 0..100 {
                            let _ = detector.detect(text);
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    memory_benches,
    bench_memory_leak_detection,
    bench_concurrent_memory_usage,
    bench_large_input_memory,
    bench_allocation_patterns,
    bench_regex_memory_usage,
    bench_thread_memory_contention
);

criterion_main!(memory_benches);
