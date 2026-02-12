#![no_main]

use libfuzzer_sys::fuzz_target;
use langweave::{detect_language, detect_language_async};

fuzz_target!(|data: &[u8]| {
    // Convert arbitrary bytes to string, allowing invalid UTF-8
    let text = String::from_utf8_lossy(data);

    // Test synchronous language detection - should never panic
    let _ = std::panic::catch_unwind(|| {
        let _ = detect_language(&text);
    });

    // Test async language detection in blocking context
    let _ = std::panic::catch_unwind(|| {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(async {
            let _ = detect_language_async(&text).await;
        });
    });
});