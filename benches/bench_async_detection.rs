// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Decoupled async-focused benchmarks.
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod stress_shared;

criterion_group!(benches, stress_shared::benchmark_async_detection);
criterion_main!(benches);
