// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Decoupled regex-focused benchmarks.
#![allow(missing_docs)]

use criterion::{criterion_group, criterion_main};

mod stress_shared;

criterion_group!(benches, stress_shared::benchmark_regex_performance);
criterion_main!(benches);
