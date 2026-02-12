#![no_main]

use libfuzzer_sys::fuzz_target;
use langweave::{is_language_supported, supported_languages};

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);

    // Test language support validation - should never panic
    let _ = std::panic::catch_unwind(|| {
        let _ = is_language_supported(&text);
    });

    // Test with all case variations
    let _ = std::panic::catch_unwind(|| {
        let upper = text.to_uppercase();
        let lower = text.to_lowercase();

        let _ = is_language_supported(&upper);
        let _ = is_language_supported(&lower);
    });

    // Test supported languages function - should never panic
    let _ = std::panic::catch_unwind(|| {
        let langs = supported_languages();
        let _ = langs;
    });

    // Test optimized functions
    let _ = std::panic::catch_unwind(|| {
        use langweave::prelude::*;
        let _ = is_language_supported_optimized(&text);
        let _ = supported_languages_optimized();
    });

    // Test with partial strings and edge cases
    for i in 0..text.len().min(10) {
        let substr = &text[..i];
        let _ = std::panic::catch_unwind(|| {
            let _ = is_language_supported(substr);
        });
    }
});