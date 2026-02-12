// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Batch Processing Module
//!
//! Provides concurrent batch operations for language detection and translation.
//! Requires the `batch` feature flag.
//!
//! ## Examples
//!
//! ```
//! use langweave::batch::{BatchConfig, detect_batch_async};
//!
//! # async fn example() {
//! let texts = vec!["Hello world", "Bonjour le monde", "Hola mundo"];
//! let config = BatchConfig::default();
//! let results = detect_batch_async(&texts, &config).await;
//! assert_eq!(results.len(), 3);
//! # }
//! ```

use crate::error::I18nError;
use crate::{detect_language, translate};
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Configuration for batch operations.
#[derive(Debug, Clone, Copy)]
pub struct BatchConfig {
    /// Maximum number of concurrent tasks. Defaults to 10.
    pub max_concurrency: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        BatchConfig {
            max_concurrency: 10,
        }
    }
}

/// Result of a single item in a batch operation.
#[derive(Debug)]
pub struct BatchResult<T> {
    /// The index of this item in the original input slice.
    pub index: usize,
    /// The result of the operation for this item.
    pub result: Result<T, I18nError>,
}

/// Detects the language of multiple texts concurrently.
///
/// Results are returned sorted by the original input index.
///
/// # Arguments
///
/// * `texts` - A slice of text strings to detect languages for.
/// * `config` - Batch configuration controlling concurrency.
///
/// # Returns
///
/// A vector of `BatchResult<String>` with detection results, sorted by input index.
///
/// # Examples
///
/// ```
/// use langweave::batch::{BatchConfig, detect_batch_async};
///
/// # async fn example() {
/// let texts = vec!["Hello", "Bonjour"];
/// let config = BatchConfig::default();
/// let results = detect_batch_async(&texts, &config).await;
/// assert_eq!(results.len(), 2);
/// assert_eq!(results[0].index, 0);
/// # }
/// ```
pub async fn detect_batch_async(
    texts: &[&str],
    config: &BatchConfig,
) -> Vec<BatchResult<String>> {
    let semaphore = Arc::new(Semaphore::new(config.max_concurrency));
    let mut join_set = tokio::task::JoinSet::new();

    for (index, &text) in texts.iter().enumerate() {
        let sem = Arc::clone(&semaphore);
        let text_owned = text.to_string();
        let _ = join_set.spawn(async move {
            let _permit = sem
                .acquire()
                .await
                .map_err(|e| I18nError::TaskFailed(e.to_string()));
            let result = crate::run_blocking(move || {
                detect_language(&text_owned)
            })
            .await;
            (index, result)
        });
    }

    let mut results = collect_join_set(&mut join_set).await;

    results.sort_by_key(|r| r.index);
    results
}

async fn collect_join_set(
    join_set: &mut tokio::task::JoinSet<(
        usize,
        Result<String, I18nError>,
    )>,
) -> Vec<BatchResult<String>> {
    let mut results = Vec::new();
    while let Some(join_result) = join_set.join_next().await {
        // Inner tasks use run_blocking which converts panics to TaskFailed,
        // so the outer JoinSet task cannot panic.
        let (index, result) =
            join_result.expect("inner task should not panic");
        results.push(BatchResult { index, result });
    }
    results
}

/// Translates multiple texts to a target language concurrently.
///
/// Results are returned sorted by the original input index.
///
/// # Arguments
///
/// * `lang` - The target language code (e.g., "fr", "de").
/// * `texts` - A slice of text strings to translate.
/// * `config` - Batch configuration controlling concurrency.
///
/// # Returns
///
/// A vector of `BatchResult<String>` with translation results, sorted by input index.
///
/// # Examples
///
/// ```
/// use langweave::batch::{BatchConfig, translate_batch_async};
///
/// # async fn example() {
/// let texts = vec!["Hello", "Goodbye"];
/// let config = BatchConfig::default();
/// let results = translate_batch_async("fr", &texts, &config).await;
/// assert_eq!(results.len(), 2);
/// # }
/// ```
pub async fn translate_batch_async(
    lang: &str,
    texts: &[&str],
    config: &BatchConfig,
) -> Vec<BatchResult<String>> {
    let semaphore = Arc::new(Semaphore::new(config.max_concurrency));
    let mut join_set = tokio::task::JoinSet::new();

    for (index, &text) in texts.iter().enumerate() {
        let sem = Arc::clone(&semaphore);
        let lang_owned = lang.to_string();
        let text_owned = text.to_string();
        let _ = join_set.spawn(async move {
            let _permit = sem
                .acquire()
                .await
                .map_err(|e| I18nError::TaskFailed(e.to_string()));
            let result = crate::run_blocking(move || {
                translate(&lang_owned, &text_owned)
            })
            .await;
            (index, result)
        });
    }

    let mut results = collect_join_set(&mut join_set).await;

    results.sort_by_key(|r| r.index);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_batch_empty() {
        let config = BatchConfig::default();
        let results = detect_batch_async(&[], &config).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_detect_batch_single() {
        let config = BatchConfig::default();
        let results =
            detect_batch_async(&["Hello world"], &config).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].index, 0);
        assert_eq!(
            results[0].result.as_ref().ok(),
            Some(&"en".to_string())
        );
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
        assert!(results[0].result.is_ok());
    }

    #[tokio::test]
    async fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.max_concurrency, 10);
    }

    #[tokio::test]
    async fn test_detect_batch_ordering() {
        let texts =
            vec!["Hello world", "Bonjour le monde", "Hola mundo"];
        let config = BatchConfig { max_concurrency: 1 };
        let results = detect_batch_async(&texts, &config).await;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].index, 0);
        assert_eq!(results[1].index, 1);
        assert_eq!(results[2].index, 2);
    }
}
