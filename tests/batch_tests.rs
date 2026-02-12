// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for the batch processing module.

#![cfg(feature = "batch")]

use langweave::batch::{
    detect_batch_async, translate_batch_async, BatchConfig,
};

#[tokio::test]
async fn test_detect_batch_empty() {
    let config = BatchConfig::default();
    let results = detect_batch_async(&[], &config).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn test_detect_batch_single() {
    let config = BatchConfig::default();
    let results = detect_batch_async(&["Hello world"], &config).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].index, 0);
    assert_eq!(
        results[0].result.as_ref().ok(),
        Some(&"en".to_string())
    );
}

#[tokio::test]
async fn test_detect_batch_multiple() {
    let texts = vec![
        "Hello world",
        "Bonjour le monde",
        "Hola mundo",
        "こんにちは",
    ];
    let config = BatchConfig::default();
    let results = detect_batch_async(&texts, &config).await;
    assert_eq!(results.len(), 4);

    // Verify ordering
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.index, i);
    }

    assert_eq!(
        results[0].result.as_ref().ok(),
        Some(&"en".to_string())
    );
    assert_eq!(
        results[1].result.as_ref().ok(),
        Some(&"fr".to_string())
    );
    assert_eq!(
        results[2].result.as_ref().ok(),
        Some(&"es".to_string())
    );
    assert_eq!(
        results[3].result.as_ref().ok(),
        Some(&"ja".to_string())
    );
}

#[tokio::test]
async fn test_detect_batch_mixed_valid_invalid() {
    let texts = vec!["Hello world", "", "12345"];
    let config = BatchConfig::default();
    let results = detect_batch_async(&texts, &config).await;
    assert_eq!(results.len(), 3);
    assert!(results[0].result.is_ok());
    assert!(results[1].result.is_err());
    assert!(results[2].result.is_err());
}

#[tokio::test]
async fn test_detect_batch_low_concurrency() {
    let texts = vec!["Hello", "Bonjour", "Hallo"];
    let config = BatchConfig { max_concurrency: 1 };
    let results = detect_batch_async(&texts, &config).await;
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].index, 0);
    assert_eq!(results[1].index, 1);
    assert_eq!(results[2].index, 2);
}

#[tokio::test]
async fn test_detect_batch_large() {
    let texts: Vec<&str> = (0..100).map(|_| "Hello world").collect();
    let config = BatchConfig::default();
    let results = detect_batch_async(&texts, &config).await;
    assert_eq!(results.len(), 100);
    for result in &results {
        assert_eq!(
            result.result.as_ref().ok(),
            Some(&"en".to_string())
        );
    }
}

#[tokio::test]
async fn test_translate_batch_empty() {
    let config = BatchConfig::default();
    let results = translate_batch_async("fr", &[], &config).await;
    assert!(results.is_empty());
}

#[tokio::test]
async fn test_translate_batch_single() {
    let config = BatchConfig::default();
    let results =
        translate_batch_async("fr", &["Hello"], &config).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].index, 0);
    assert_eq!(
        results[0].result.as_ref().ok(),
        Some(&"Bonjour".to_string())
    );
}

#[tokio::test]
async fn test_translate_batch_multiple() {
    let texts = vec!["Hello", "Goodbye"];
    let config = BatchConfig::default();
    let results = translate_batch_async("fr", &texts, &config).await;
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].index, 0);
    assert_eq!(results[1].index, 1);
    assert!(results[0].result.is_ok());
}

#[tokio::test]
async fn test_batch_config_default() {
    let config = BatchConfig::default();
    assert_eq!(config.max_concurrency, 10);
}

#[tokio::test]
async fn test_batch_config_copy() {
    let config = BatchConfig { max_concurrency: 5 };
    let copied = config;
    assert_eq!(copied.max_concurrency, 5);
    // Original is still accessible since BatchConfig implements Copy
    assert_eq!(config.max_concurrency, 5);
}
