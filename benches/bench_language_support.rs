// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Decoupled language-support and allocation benchmarks.
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod stress_shared;

criterion_group!(
    benches,
    stress_shared::benchmark_language_support_optimizations,
    stress_shared::benchmark_memory_hotspots
);
criterion_main!(benches);
