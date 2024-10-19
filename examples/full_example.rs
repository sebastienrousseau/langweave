// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # LangWeave Examples
//!
//! This module serves as an entry point for running all the LangWeave examples,
//! demonstrating various aspects of the library including language detection,
//! translation, error handling, and usage of different components.

mod error_example;
mod language_detector_example;
mod language_detector_trait_example;
mod lib_example;
mod translations_example;
mod translator_example;

use std::error::Error;

/// Runs all LangWeave examples.
///
/// This function sequentially executes all individual examples, demonstrating
/// various features and capabilities of the LangWeave library.
fn main() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Running LangWeave Examples 🦀");

    // Run synchronous examples first
    translations_example::main()?;
    translator_example::main()?;

    // Now, create a runtime for async examples
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        error_example::main()?;
        language_detector_trait_example::main()?;
        language_detector_example::main()?;
        lib_example::main()?;
        Ok::<_, Box<dyn Error>>(())
    })?;

    println!("\n🎉 All LangWeave examples completed successfully!\n");

    Ok(())
}
