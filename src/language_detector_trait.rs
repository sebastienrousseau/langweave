// Copyright © 2024 LangWeave. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Language Detector Trait
//!
//! This module defines the `LanguageDetectorTrait` which allows for custom
//! implementations of language detection methods. It also provides a
//! `CompositeLanguageDetector` that can combine multiple detectors for
//! improved accuracy through ensemble detection.
//!
//! ## Examples
//!
//! ### Creating a Custom Detector
//!
//! ```
//! use langweave::language_detector_trait::LanguageDetectorTrait;
//! use langweave::error::I18nError;
//! use async_trait::async_trait;
//!
//! struct CustomDetector;
//!
//! #[async_trait]
//! impl LanguageDetectorTrait for CustomDetector {
//!     fn detect(&self, text: &str) -> Result<String, I18nError> {
//!         if text.contains("hello") {
//!             Ok("en".to_string())
//!         } else {
//!             Err(I18nError::LanguageDetectionFailed)
//!         }
//!     }
//!
//!     async fn detect_async(&self, text: &str) -> Result<String, I18nError> {
//!         self.detect(text)
//!     }
//! }
//! ```
//!
//! ### Using CompositeLanguageDetector
//!
//! ```
//! use langweave::language_detector_trait::CompositeLanguageDetector;
//! use langweave::language_detector::LanguageDetector;
//!
//! let mut composite = CompositeLanguageDetector::new();
//! composite.add_detector(Box::new(LanguageDetector::new()));
//!
//! // Now use composite.detect() or composite.detect_async()
//! ```

use crate::error::I18nError;
use async_trait::async_trait;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A trait for implementing custom language detection methods.
///
/// This trait allows for extensible language detection by providing both synchronous
/// and asynchronous detection methods. Implementers can use any detection algorithm,
/// from simple keyword matching to sophisticated machine learning models.
///
/// # Examples
///
/// ```
/// use langweave::language_detector_trait::LanguageDetectorTrait;
/// use langweave::error::I18nError;
/// use async_trait::async_trait;
///
/// struct SimpleDetector;
///
/// #[async_trait]
/// impl LanguageDetectorTrait for SimpleDetector {
///     fn detect(&self, text: &str) -> Result<String, I18nError> {
///         if text.to_lowercase().contains("hello") {
///             Ok("en".to_string())
///         } else if text.to_lowercase().contains("bonjour") {
///             Ok("fr".to_string())
///         } else {
///             Err(I18nError::LanguageDetectionFailed)
///         }
///     }
///
///     async fn detect_async(&self, text: &str) -> Result<String, I18nError> {
///         self.detect(text)
///     }
/// }
///
/// let detector = SimpleDetector;
/// assert_eq!(detector.detect("Hello world").unwrap(), "en");
/// assert_eq!(detector.detect("Bonjour monde").unwrap(), "fr");
/// ```
#[async_trait]
pub trait LanguageDetectorTrait: Send + Sync {
    /// Detects the language of the given text synchronously.
    ///
    /// This method should analyze the input text and return a language code
    /// (typically ISO 639-1 format like "en", "fr", "de") if detection is successful.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The detected language code if successful.
    /// * `Err(I18nError)` - An error if detection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// let detector = LanguageDetector::new();
    /// let result = detector.detect("Hello, world!");
    /// assert_eq!(result.unwrap(), "en");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The input text is empty or contains only non-alphabetic characters.
    /// * The detection algorithm cannot identify a language with sufficient confidence.
    fn detect(&self, text: &str) -> Result<String, I18nError>;

    /// Detects the language of the given text asynchronously.
    ///
    /// This method provides the same functionality as `detect`, but operates asynchronously,
    /// allowing for non-blocking language detection in concurrent contexts.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The detected language code if successful.
    /// * `Err(I18nError)` - An error if detection fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector::LanguageDetector;
    /// use langweave::language_detector_trait::LanguageDetectorTrait;
    ///
    /// # async fn example() {
    /// let detector = LanguageDetector::new();
    /// let result = detector.detect_async("Bonjour le monde!").await;
    /// assert_eq!(result.unwrap(), "fr");
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The input text is empty or contains only non-alphabetic characters.
    /// * The detection algorithm cannot identify a language with sufficient confidence.
    async fn detect_async(
        &self,
        text: &str,
    ) -> Result<String, I18nError>;
}

/// A struct to hold multiple language detectors for ensemble detection.
///
/// The `CompositeLanguageDetector` allows combining multiple detection strategies
/// to improve overall accuracy. It tries each detector in order until one succeeds,
/// implementing a simple fallback mechanism.
///
/// # Examples
///
/// ```
/// use langweave::language_detector_trait::CompositeLanguageDetector;
/// use langweave::language_detector::LanguageDetector;
///
/// let mut composite = CompositeLanguageDetector::new();
/// composite.add_detector(Box::new(LanguageDetector::new()));
///
/// // Now the composite can be used for detection
/// let result = composite.detect("Hello, world!");
/// assert_eq!(result.unwrap(), "en");
/// ```
#[derive(Default)]
pub struct CompositeLanguageDetector {
    detectors: Vec<Box<dyn LanguageDetectorTrait>>,
}

impl Debug for CompositeLanguageDetector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CompositeLanguageDetector with {} detectors",
            self.detectors.len()
        )
    }
}

impl CompositeLanguageDetector {
    /// Creates a new empty `CompositeLanguageDetector`.
    ///
    /// # Returns
    ///
    /// A new instance with no detectors. You must add detectors using `add_detector`
    /// before performing detection.
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector_trait::CompositeLanguageDetector;
    ///
    /// let composite = CompositeLanguageDetector::new();
    /// assert_eq!(format!("{:?}", composite), "CompositeLanguageDetector with 0 detectors");
    /// ```
    pub fn new() -> Self {
        CompositeLanguageDetector {
            detectors: Vec::new(),
        }
    }

    /// Adds a new detector to the composite.
    ///
    /// Detectors are tried in the order they are added. The first detector that
    /// successfully identifies a language will be used, and subsequent detectors
    /// will not be consulted for that detection request.
    ///
    /// # Arguments
    ///
    /// * `detector` - A boxed language detector implementing `LanguageDetectorTrait`
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector_trait::CompositeLanguageDetector;
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// let mut composite = CompositeLanguageDetector::new();
    /// composite.add_detector(Box::new(LanguageDetector::new()));
    ///
    /// // Add more detectors as needed
    /// // composite.add_detector(Box::new(AnotherDetector::new()));
    /// ```
    pub fn add_detector(
        &mut self,
        detector: Box<dyn LanguageDetectorTrait>,
    ) {
        self.detectors.push(detector);
    }

    /// Detects the language using all added detectors in sequence.
    ///
    /// This method tries each detector in the order they were added until one
    /// successfully identifies a language. If all detectors fail, returns an error.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The detected language code from the first successful detector
    /// * `Err(I18nError)` - An error if all detectors fail
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector_trait::CompositeLanguageDetector;
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// let mut composite = CompositeLanguageDetector::new();
    /// composite.add_detector(Box::new(LanguageDetector::new()));
    ///
    /// let result = composite.detect("Hello, world!");
    /// assert_eq!(result.unwrap(), "en");
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * No detectors have been added to the composite.
    /// * All added detectors fail to identify the language.
    pub fn detect(&self, text: &str) -> Result<String, I18nError> {
        for detector in &self.detectors {
            if let Ok(lang) = detector.detect(text) {
                return Ok(lang);
            }
        }
        Err(I18nError::LanguageDetectionFailed)
    }

    /// Detects the language asynchronously using all added detectors in sequence.
    ///
    /// This method provides the same functionality as `detect`, but operates asynchronously.
    /// It tries each detector in order until one succeeds.
    ///
    /// # Arguments
    ///
    /// * `text` - A string slice that holds the text to analyze
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The detected language code from the first successful detector
    /// * `Err(I18nError)` - An error if all detectors fail
    ///
    /// # Examples
    ///
    /// ```
    /// use langweave::language_detector_trait::CompositeLanguageDetector;
    /// use langweave::language_detector::LanguageDetector;
    ///
    /// # async fn example() {
    /// let mut composite = CompositeLanguageDetector::new();
    /// composite.add_detector(Box::new(LanguageDetector::new()));
    ///
    /// let result = composite.detect_async("Bonjour le monde!").await;
    /// assert_eq!(result.unwrap(), "fr");
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * No detectors have been added to the composite.
    /// * All added detectors fail to identify the language.
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
