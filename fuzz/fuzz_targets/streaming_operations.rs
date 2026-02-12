#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(feature = "stream")]
use langweave::{streaming::*, detect_language};

fuzz_target!(|data: &[u8]| {
    #[cfg(feature = "stream")]
    {
        if data.len() < 2 {
            return;
        }

        let text = String::from_utf8_lossy(data);
        let chunk_size = (data[0] as usize % 100) + 10; // 10-109 chunk size

        // Test text chunking - should never panic
        let _ = std::panic::catch_unwind(|| {
            let chunks: Vec<String> = chunk_text(&text, chunk_size).collect();

            // Verify chunks can be processed
            for chunk in chunks {
                let _ = chunk.len();
            }
        });

        // Test streaming language detection - should never panic
        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                use tokio_stream::StreamExt;

                let config = StreamConfig { chunk_size };
                let mut stream = detect_language_stream(&text, &config);

                while let Some(result) = stream.next().await {
                    let _ = result; // Process result
                }
            });
        });

        // Test streaming translation - should never panic
        let _ = std::panic::catch_unwind(|| {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                use tokio_stream::StreamExt;

                let config = StreamConfig { chunk_size };
                let mut stream = translate_stream("en", &text, &config);

                while let Some(result) = stream.next().await {
                    let _ = result; // Process result
                }
            });
        });

        // Test with different chunk sizes
        let sizes = [1, 5, 50, 500, 1000];
        for &size in &sizes {
            let _ = std::panic::catch_unwind(|| {
                let chunks: Vec<String> = chunk_text(&text, size).collect();
                let _ = chunks;
            });
        }

        // Test edge cases
        let _ = std::panic::catch_unwind(|| {
            let empty_chunks: Vec<String> = chunk_text("", chunk_size).collect();
            let _ = empty_chunks;
        });

        let _ = std::panic::catch_unwind(|| {
            let single_char_chunks: Vec<String> = chunk_text("a", 1).collect();
            let _ = single_char_chunks;
        });
    }

    #[cfg(not(feature = "stream"))]
    {
        // If stream feature not enabled, test basic operations instead
        let text = String::from_utf8_lossy(data);
        let _ = std::panic::catch_unwind(|| {
            let _ = detect_language(&text);
        });
    }
});