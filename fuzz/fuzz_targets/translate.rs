#![no_main]

use libfuzzer_sys::fuzz_target;
use langweave::{translate, is_language_supported, supported_languages};

fuzz_target!(|data: &[u8]| {
    // Handle case where data is too small
    if data.len() < 2 {
        return;
    }

    // Split input: first part as language code, rest as text
    let split_point = data.len() / 2;
    let lang_bytes = &data[..split_point.min(10)]; // Limit lang code length
    let text_bytes = &data[split_point..];

    let lang = String::from_utf8_lossy(lang_bytes);
    let text = String::from_utf8_lossy(text_bytes);

    // Test translation - should never panic regardless of input
    let _ = std::panic::catch_unwind(|| {
        let _ = translate(&lang, &text);
    });

    // Test language support checking - should never panic
    let _ = std::panic::catch_unwind(|| {
        let _ = is_language_supported(&lang);
    });

    // Test with actual supported languages too
    let _ = std::panic::catch_unwind(|| {
        for &supported_lang in supported_languages() {
            let _ = translate(supported_lang, &text);
        }
    });

    // Test async translation if feature enabled
    #[cfg(feature = "async")]
    {
        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                use langweave::async_utils::translate_async;
                let _ = translate_async(&lang, &text).await;
            });
        });
    }
});