// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Comprehensive stress test and performance baseline establishment for LangWeave.
//!
//! This benchmark keeps the original full-fidelity suite in one entrypoint.
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod stress_shared;

criterion_group!(
    stress_benches,
    stress_shared::benchmark_language_detection_baselines,
    stress_shared::benchmark_async_detection,
    stress_shared::benchmark_translation_performance,
    stress_shared::benchmark_language_support_optimizations,
    stress_shared::benchmark_memory_hotspots,
    stress_shared::benchmark_extreme_stress_test,
    stress_shared::benchmark_concurrent_performance,
    stress_shared::benchmark_regex_performance
);

criterion_main!(stress_benches);
