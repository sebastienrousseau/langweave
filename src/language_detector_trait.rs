// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Language Detector Trait
//!
//! This module defines the `LanguageDetectorTrait` which allows for custom
//! implementations of language detection methods.

use crate::error::I18nError;
use async_trait::async_trait;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A trait for implementing custom language detection methods.
#[async_trait]
pub trait LanguageDetectorTrait: Send + Sync {
    /// Detects the language of the given text synchronously.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Result<String, I18nError>` - The detected language code if successful, or an error if detection fails.
    fn detect(&self, text: &str) -> Result<String, I18nError>;

    /// Detects the language of the given text asynchronously.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Result<String, I18nError>` - The detected language code if successful, or an error if detection fails.
    async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError>;
}

/// A struct to hold multiple language detectors.
#[derive(Default)]
pub struct CompositeLanguageDetector {
    detectors: Vec<Box<dyn LanguageDetectorTrait>>,
}

impl Debug for CompositeLanguageDetector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "CompositeLanguageDetector")
    }
}

impl CompositeLanguageDetector {
    /// Creates a new `CompositeLanguageDetector`.
    pub fn new() -> Self {
        CompositeLanguageDetector {
            detectors: Vec::new(),
        }
    }

    /// Adds a new detector to the composite.
    pub fn add_detector(
        &mut self,
        detector: Box<dyn LanguageDetectorTrait>,
    ) {
        self.detectors.push(detector);
    }

    /// Detects the language using all added detectors.
    pub fn detect(&self, text: &str) -> Result<String, I18nError> {
        for detector in &self.detectors {
            if let Ok(lang) = detector.detect(text) {
                return Ok(lang);
            }
        }
        Err(I18nError::LanguageDetectionFailed)
    }

    /// Detects the language asynchronously using all added detectors.
    pub async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError> {
        for detector in &self.detectors {
            if let Ok(lang) = detector.detect_async(text).await {
                return Ok(lang);
            }
        }
        Err(I18nError::LanguageDetectionFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockDetector;

    #[async_trait]
    impl LanguageDetectorTrait for MockDetector {
        fn detect(&self, text: &str) -> Result<String, I18nError> {
            if text.contains("English") {
                Ok("en".to_string())
            } else {
                Err(I18nError::LanguageDetectionFailed)
            }
        }

        async fn detect_async(
            &self,
            text: &str,
        ) -> Result<String, I18nError> {
            self.detect(text)
        }
    }

    #[test]
    fn test_composite_detector() {
        let mut composite = CompositeLanguageDetector::new();
        composite.add_detector(Box::new(MockDetector));

        assert_eq!(composite.detect("This is English").unwrap(), "en");
        assert!(composite.detect("Это русский").is_err());
    }

    #[tokio::test]
    async fn test_composite_detector_async() {
        let mut composite = CompositeLanguageDetector::new();
        composite.add_detector(Box::new(MockDetector));

        assert_eq!(
            composite.detect_async("This is English").await.unwrap(),
            "en"
        );
        assert!(composite.detect_async("Это русский").await.is_err());
    }
}
