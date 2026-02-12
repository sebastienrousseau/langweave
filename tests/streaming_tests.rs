// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for the streaming module.

#![cfg(feature = "stream")]

use langweave::streaming::{
    chunk_text, detect_language_stream, translate_stream, StreamConfig,
};
use tokio_stream::StreamExt;

#[test]
fn test_chunk_text_empty() {
    let chunks = chunk_text("", 100);
    assert!(chunks.is_empty());
}

#[test]
fn test_chunk_text_single_chunk() {
    let chunks = chunk_text("Hello world", 100);
    assert_eq!(chunks, vec!["Hello world"]);
}

#[test]
fn test_chunk_text_word_boundary() {
    let chunks = chunk_text("Hello world foo bar baz", 12);
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], "Hello world");
    assert_eq!(chunks[1], "foo bar baz");
}

#[test]
fn test_chunk_text_exact_fit() {
    let chunks = chunk_text("Hello", 5);
    assert_eq!(chunks, vec!["Hello"]);
}

#[test]
fn test_chunk_text_long_word() {
    // A word longer than chunk_size is split at chunk_size boundary
    let chunks = chunk_text("superlongword", 5);
    assert_eq!(chunks[0], "super");
    assert!(chunks.len() > 1);
}

#[test]
fn test_chunk_text_multiple_chunks() {
    let text = "The quick brown fox jumps over the lazy dog";
    let chunks = chunk_text(text, 20);
    assert!(chunks.len() >= 2);
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
async fn test_detect_language_stream_multiple_chunks() {
    // Use a small chunk size to force multiple chunks
    let config = StreamConfig {
        chunk_size: 15,
        buffer_size: 8,
    };
    let text = "Hello world this is a test with multiple words";
    let mut stream = detect_language_stream(text, &config);
    let mut results = Vec::new();
    while let Some(chunk) = stream.next().await {
        results.push(chunk);
    }
    assert!(results.len() > 1);
    // Verify chunk indices are sequential
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.chunk_index, i);
    }
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

#[tokio::test]
async fn test_translate_stream_single() {
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
async fn test_translate_stream_empty() {
    let config = StreamConfig {
        chunk_size: 100,
        buffer_size: 4,
    };
    let mut stream = translate_stream("fr", "", &config);
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

#[test]
fn test_stream_config_copy() {
    let config = StreamConfig {
        chunk_size: 500,
        buffer_size: 8,
    };
    let copied = config;
    assert_eq!(copied.chunk_size, 500);
    assert_eq!(copied.buffer_size, 8);
    // Original still accessible since StreamConfig implements Copy
    assert_eq!(config.chunk_size, 500);
}
