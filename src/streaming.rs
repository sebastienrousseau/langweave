// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Streaming Module
//!
//! Provides streaming operations for language detection and translation of large texts.
//! Text is split into chunks and processed as an async stream.
//! Requires the `stream` feature flag.
//!
//! ## Examples
//!
//! ```
//! use langweave::streaming::{StreamConfig, detect_language_stream};
//! use tokio_stream::StreamExt;
//!
//! # async fn example() {
//! let config = StreamConfig::default();
//! let mut stream = detect_language_stream("Hello world, this is a test.", &config);
//! while let Some(chunk_result) = stream.next().await {
//!     println!("Chunk {}: {:?}", chunk_result.chunk_index, chunk_result.result);
//! }
//! # }
//! ```

use crate::error::I18nError;
use crate::{detect_language, translate};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;

/// Configuration for streaming operations.
#[derive(Debug, Clone, Copy)]
pub struct StreamConfig {
    /// Maximum number of characters per chunk. Defaults to 1000.
    pub chunk_size: usize,
    /// Size of the internal channel buffer. Defaults to 16.
    pub buffer_size: usize,
}

impl Default for StreamConfig {
    fn default() -> Self {
        StreamConfig {
            chunk_size: 1000,
            buffer_size: 16,
        }
    }
}

/// The result of processing a single chunk.
#[derive(Debug)]
pub struct ChunkResult {
    /// The index of this chunk in the sequence.
    pub chunk_index: usize,
    /// The text of this chunk.
    pub chunk_text: String,
    /// The result of the operation on this chunk.
    pub result: Result<String, I18nError>,
}

/// Splits text into chunks at word boundaries.
///
/// Each chunk will be at most `chunk_size` characters, splitting at the last
/// whitespace boundary before the limit. If a single word exceeds `chunk_size`,
/// it will be placed in its own chunk.
///
/// # Arguments
///
/// * `text` - The text to split.
/// * `chunk_size` - The maximum number of characters per chunk.
///
/// # Returns
///
/// A vector of chunk strings.
///
/// # Examples
///
/// ```
/// use langweave::streaming::chunk_text;
///
/// let chunks = chunk_text("Hello world foo bar", 12);
/// assert_eq!(chunks, vec!["Hello world", "foo bar"]);
/// ```
pub fn chunk_text(text: &str, chunk_size: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![];
    }

    let mut chunks = Vec::new();
    let mut remaining = text.trim_start();

    while !remaining.is_empty() {
        if remaining.len() <= chunk_size {
            chunks.push(remaining.to_string());
            break;
        }

        // Find the nearest char boundary at or before chunk_size
        let mut byte_limit = chunk_size.min(remaining.len());
        while byte_limit > 0 && !remaining.is_char_boundary(byte_limit) {
            byte_limit -= 1;
        }
        if byte_limit == 0 {
            // Fallback: take the first char
            byte_limit = remaining
                .chars()
                .next()
                .map_or(remaining.len(), char::len_utf8);
        }

        // Find the last whitespace boundary within the safe range
        let boundary = remaining[..byte_limit]
            .rfind(char::is_whitespace)
            .filter(|&b| b > 0)
            .unwrap_or(byte_limit);

        let (chunk, rest) = remaining.split_at(boundary);
        chunks.push(chunk.trim().to_string());
        remaining = rest.trim_start();
    }

    chunks
}

/// Creates a stream that detects the language of each chunk of the input text.
///
/// # Arguments
///
/// * `text` - The text to process in chunks.
/// * `config` - Streaming configuration controlling chunk size and buffer.
///
/// # Returns
///
/// An async `Stream` of `ChunkResult` items.
///
/// # Examples
///
/// ```
/// use langweave::streaming::{StreamConfig, detect_language_stream};
/// use tokio_stream::StreamExt;
///
/// # async fn example() {
/// let config = StreamConfig { chunk_size: 20, buffer_size: 4 };
/// let mut stream = detect_language_stream("Hello world", &config);
/// while let Some(chunk) = stream.next().await {
///     println!("{:?}", chunk.result);
/// }
/// # }
/// ```
pub fn detect_language_stream(
    text: &str,
    config: &StreamConfig,
) -> impl Stream<Item = ChunkResult> {
    let chunks = chunk_text(text, config.chunk_size);
    let (tx, rx) = tokio::sync::mpsc::channel(config.buffer_size);

    drop(tokio::spawn(async move {
        for (chunk_index, chunk_text) in chunks.into_iter().enumerate()
        {
            let text_for_detect = chunk_text.clone();
            let result =
                crate::run_blocking(move || {
                    detect_language(&text_for_detect)
                })
                .await;

            let chunk_result = ChunkResult {
                chunk_index,
                chunk_text,
                result,
            };
            if tx.send(chunk_result).await.is_err() {
                break;
            }
        }
    }));

    ReceiverStream::new(rx)
}

/// Creates a stream that translates each chunk of the input text.
///
/// # Arguments
///
/// * `lang` - The target language code.
/// * `text` - The text to process in chunks.
/// * `config` - Streaming configuration controlling chunk size and buffer.
///
/// # Returns
///
/// An async `Stream` of `ChunkResult` items.
///
/// # Examples
///
/// ```
/// use langweave::streaming::{StreamConfig, translate_stream};
/// use tokio_stream::StreamExt;
///
/// # async fn example() {
/// let config = StreamConfig { chunk_size: 20, buffer_size: 4 };
/// let mut stream = translate_stream("fr", "Hello", &config);
/// while let Some(chunk) = stream.next().await {
///     println!("{:?}", chunk.result);
/// }
/// # }
/// ```
pub fn translate_stream(
    lang: &str,
    text: &str,
    config: &StreamConfig,
) -> impl Stream<Item = ChunkResult> {
    let chunks = chunk_text(text, config.chunk_size);
    let lang_owned = lang.to_string();
    let (tx, rx) = tokio::sync::mpsc::channel(config.buffer_size);

    drop(tokio::spawn(async move {
        for (chunk_index, chunk_text) in chunks.into_iter().enumerate()
        {
            let lang_clone = lang_owned.clone();
            let text_for_translate = chunk_text.clone();
            let result = crate::run_blocking(move || {
                translate(&lang_clone, &text_for_translate)
            })
            .await;

            let chunk_result = ChunkResult {
                chunk_index,
                chunk_text,
                result,
            };
            if tx.send(chunk_result).await.is_err() {
                break;
            }
        }
    }));

    ReceiverStream::new(rx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_stream::StreamExt;

    #[test]
    fn test_chunk_text_empty() {
        let chunks = chunk_text("", 100);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_chunk_text_single() {
        let chunks = chunk_text("Hello", 100);
        assert_eq!(chunks, vec!["Hello"]);
    }

    #[test]
    fn test_chunk_text_splits_at_word_boundary() {
        let chunks = chunk_text("Hello world foo bar", 12);
        assert_eq!(chunks, vec!["Hello world", "foo bar"]);
    }

    #[test]
    fn test_chunk_text_long_word() {
        let chunks = chunk_text("superlongword next", 5);
        // When no word boundary exists within chunk_size, splits at chunk_size
        assert_eq!(chunks[0], "super");
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_chunk_text_multibyte_small_chunk() {
        // chunk_size=1 on a 4-byte emoji forces byte_limit to 0,
        // triggering the fallback to take the first char
        let chunks = chunk_text("\u{1F980}abc", 1);
        assert_eq!(chunks[0], "\u{1F980}");
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_chunk_text_leading_whitespace() {
        let chunks = chunk_text("  Hello world", 10);
        assert!(!chunks.is_empty());
        for chunk in &chunks {
            assert!(!chunk.is_empty());
        }
    }

    #[tokio::test]
    async fn test_detect_language_stream_single_chunk() {
        let config = StreamConfig {
            chunk_size: 1000,
            buffer_size: 4,
        };
        let mut stream = detect_language_stream("Hello world", &config);
        let mut results = Vec::new();
        while let Some(chunk) = stream.next().await {
            results.push(chunk);
        }
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk_index, 0);
        assert!(results[0].result.is_ok());
    }

    #[tokio::test]
    async fn test_translate_stream_single_chunk() {
        let config = StreamConfig {
            chunk_size: 1000,
            buffer_size: 4,
        };
        let mut stream = translate_stream("fr", "Hello", &config);
        let mut results = Vec::new();
        while let Some(chunk) = stream.next().await {
            results.push(chunk);
        }
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].chunk_index, 0);
    }

    #[tokio::test]
    async fn test_detect_language_stream_empty() {
        let config = StreamConfig {
            chunk_size: 100,
            buffer_size: 4,
        };
        let mut stream = detect_language_stream("", &config);
        let mut count = 0;
        while let Some(_chunk) = stream.next().await {
            count += 1;
        }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_stream_config_default() {
        let config = StreamConfig::default();
        assert_eq!(config.chunk_size, 1000);
        assert_eq!(config.buffer_size, 16);
    }
}
