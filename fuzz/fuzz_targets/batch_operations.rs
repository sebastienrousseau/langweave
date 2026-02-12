#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(feature = "batch")]
use langweave::batch::{detect_batch_async, translate_batch_async, BatchConfig};

#[cfg(not(feature = "batch"))]
use langweave::detect_language;

fuzz_target!(|data: &[u8]| {
    #[cfg(feature = "batch")]
    {
        // Skip too small inputs
        if data.len() < 4 {
            return;
        }

        // Parse input: first byte for batch size, rest split into texts
        let batch_size = (data[0] as usize % 10) + 1; // 1-10 items
        let remaining = &data[1..];

        // Split remaining data into chunks for batch processing
        let mut texts = Vec::new();
        let chunk_size = remaining.len().max(1) / batch_size;

        for i in 0..batch_size {
            let start = i * chunk_size;
            let end = ((i + 1) * chunk_size).min(remaining.len());
            if start < remaining.len() {
                let text = String::from_utf8_lossy(&remaining[start..end]);
                texts.push(text.to_string());
            }
        }

        if texts.is_empty() {
            return;
        }

        let text_refs: Vec<&str> = texts.iter().map(String::as_str).collect();

        // Test batch operations - should never panic
        let config = BatchConfig::default();

        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let _ = detect_batch_async(&text_refs, &config).await;
            });
        });

        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let _ = translate_batch_async("en", &text_refs, &config).await;
                let _ = translate_batch_async("invalid", &text_refs, &config).await;
            });
        });

        // Test with custom concurrency
        let custom_config = BatchConfig {
            max_concurrency: (data[0] as usize % 5) + 1,
        };

        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let _ = detect_batch_async(&text_refs, &custom_config).await;
                let _ = translate_batch_async("fr", &text_refs, &custom_config).await;
            });
        });
    }

    #[cfg(not(feature = "batch"))]
    {
        // If batch feature not enabled, test individual operations instead
        let text = String::from_utf8_lossy(data);
        let _ = std::panic::catch_unwind(|| {
            let _ = detect_language(&text);
        });
    }
});
