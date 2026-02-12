# (c) 2026 Euxis Fleet. All rights reserved.

# Error Handling Guide

This guide covers LangWeave's comprehensive error handling system and how to handle errors gracefully in your applications.

## Overview

LangWeave uses the `I18nError` enum to represent all possible error conditions. This provides clear error categorization and enables robust error handling strategies.

## Error Types

### Core Error Categories

```rust
use langweave::error::I18nError;

match some_operation() {
    Ok(result) => println!("Success: {}", result),
    Err(I18nError::EmptyInput) => {
        // Handle empty or null input
    }
    Err(I18nError::UnsupportedLanguage(lang)) => {
        // Handle unsupported language codes
        println!("Language not supported: {}", lang);
    }
    Err(I18nError::ConfidenceTooLow) => {
        // Handle low-confidence detections
    }
    Err(I18nError::TranslationFailed(reason)) => {
        // Handle translation failures
        println!("Translation failed: {}", reason);
    }
    Err(I18nError::NetworkError) => {
        // Handle network connectivity issues
    }
    Err(I18nError::ConfigurationError(msg)) => {
        // Handle configuration problems
        println!("Configuration error: {}", msg);
    }
    Err(I18nError::ParseError(msg)) => {
        // Handle parsing errors
        println!("Parse error: {}", msg);
    }
    Err(I18nError::IoError(err)) => {
        // Handle I/O errors
        println!("I/O error: {}", err);
    }
}
```

## Language Detection Errors

### Handling Detection Failures

```rust
use langweave::detect_language;
use langweave::error::I18nError;

async fn safe_language_detection(text: &str) -> Result<String, String> {
    match detect_language(text).await {
        Ok(language) => Ok(language),
        Err(I18nError::LanguageDetectionFailed) => {
            Err("Cannot detect language for this text".to_string())
        }
        Err(e) => Err(format!("Detection failed: {}", e)),
    }
}
```

### Fallback Detection Strategies

```rust
use langweave::detect_language;

async fn detect_with_fallbacks(text: &str) -> String {
    // Try primary detection
    if let Ok(lang) = detect_language(text).await {
        return lang;
    }

    // Fallback 1: Analyze text characteristics for basic patterns
    if text.to_lowercase().contains("bonjour") || text.to_lowercase().contains("merci") {
        return "fr".to_string(); // Likely French
    }

    if text.to_lowercase().contains("hallo") || text.to_lowercase().contains("auf") {
        return "de".to_string(); // Likely German
    }

    // Final fallback: Default to English
    println!("Warning: Using default language (English)");
    "en".to_string()
}
```

## Translation Errors

### Robust Translation Handling

```rust
use langweave::translate;
use langweave::error::I18nError;

fn safe_translate(text: &str, to_lang: &str) -> Result<String, String> {
    match translate(to_lang, text) {
        Ok(translation) => Ok(translation),
        Err(I18nError::UnsupportedLanguage(lang)) => {
            Err(format!("Language '{}' is not supported", lang))
        }
        Err(I18nError::TranslationFailed(reason)) => {
            Err(format!("Translation failed: {}", reason))
        }
        Err(e) => Err(format!("Unexpected error: {}", e)),
    }
}
```

### Translation with Retry Logic

```rust
use langweave::translate;
use langweave::error::I18nError;
use std::thread::sleep;
use std::time::Duration;

fn translate_with_retry(text: &str, to_lang: &str, max_retries: u32) -> Result<String, I18nError> {
    let mut retries = 0;

    loop {
        match translate(to_lang, text) {
            Ok(translation) => return Ok(translation),
            Err(I18nError::TranslationFailed(reason)) if retries < max_retries => {
                retries += 1;
                let delay = Duration::from_millis(1000 * retries as u64);
                println!("Translation failed, retrying in {:?}...", delay);
                sleep(delay);
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Configuration Errors

### Handling Setup Issues

```rust
use langweave::language_detector::LanguageDetector;

fn setup_detector_safely() -> Result<LanguageDetector, String> {
    // LanguageDetector::new() doesn't return a Result in the current implementation
    let detector = LanguageDetector::new();
    Ok(detector)
}

// For translator setup with error handling
use langweave::translator::Translator;
use langweave::error::I18nError;

fn setup_translator_safely(lang: &str) -> Result<Translator, String> {
    match Translator::new(lang) {
        Ok(translator) => Ok(translator),
        Err(I18nError::UnsupportedLanguage(lang)) => {
            Err(format!("Language '{}' is not supported. Use 'en', 'fr', or 'de'.", lang))
        }
        Err(e) => Err(format!("Unexpected setup error: {}", e)),
    }
}
```

## Custom Error Types

### Wrapping LangWeave Errors

```rust
use langweave::error::I18nError;
use std::fmt;

#[derive(Debug)]
enum AppError {
    LangWeave(I18nError),
    UserInput(String),
    BusinessLogic(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::LangWeave(err) => write!(f, "Language processing error: {}", err),
            AppError::UserInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::BusinessLogic(msg) => write!(f, "Business logic error: {}", msg),
        }
    }
}

impl From<I18nError> for AppError {
    fn from(err: I18nError) -> AppError {
        AppError::LangWeave(err)
    }
}

impl std::error::Error for AppError {}

// Usage example
use langweave::detect_language;

async fn process_text(text: &str) -> Result<String, AppError> {
    if text.trim().is_empty() {
        return Err(AppError::UserInput("Text cannot be empty".to_string()));
    }

    let language = detect_language(text).await?; // Automatically converts I18nError

    Ok(language)
}
```

## Error Recovery Strategies

### Graceful Degradation

```rust
use langweave::{language_detector::LanguageDetector, translate, detect_language};
use langweave::language_detector_trait::LanguageDetectorTrait;

pub struct RobustLanguageProcessor {
    detector: LanguageDetector,
    fallback_language: String,
}

impl RobustLanguageProcessor {
    pub fn new() -> Self {
        Self {
            detector: LanguageDetector::new(),
            fallback_language: "en".to_string(),
        }
    }

    pub async fn process_with_fallback(
        &self,
        text: &str,
        target_lang: &str
    ) -> String {
        // Try full processing
        if let Ok(result) = self.full_process(text, target_lang).await {
            return result;
        }

        // Fallback 1: Assume source language is fallback language
        if let Ok(translation) = translate(target_lang, text) {
            return translation;
        }

        // Fallback 2: Return original text with warning
        format!("(Translation unavailable) {}", text)
    }

    async fn full_process(&self, text: &str, target_lang: &str) -> Result<String, Box<dyn std::error::Error>> {
        let _detected_lang = self.detector.detect_async(text).await?;
        let translation = translate(target_lang, text)?;
        Ok(translation)
    }
}
```

### Circuit Breaker Pattern

```rust
use langweave::translator::Translator;
use langweave::error::I18nError;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct TranslationCircuitBreaker {
    translator: Translator,
    failure_count: Arc<Mutex<u32>>,
    last_failure: Arc<Mutex<Option<Instant>>>,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl TranslationCircuitBreaker {
    pub fn new() -> Self {
        Self {
            translator: Translator::new("fr")?,
            failure_count: Arc::new(Mutex::new(0)),
            last_failure: Arc::new(Mutex::new(None)),
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
        }
    }

    pub async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, String> {
        // Check if circuit is open
        if self.is_circuit_open() {
            return Err("Translation service temporarily unavailable".to_string());
        }

        match self.translator.translate(text) {
            Ok(result) => {
                // Reset failure count on success
                *self.failure_count.lock().unwrap() = 0;
                Ok(result)
            }
            Err(e) => {
                self.record_failure();
                Err(format!("Translation failed: {}", e))
            }
        }
    }

    fn is_circuit_open(&self) -> bool {
        let failure_count = *self.failure_count.lock().unwrap();

        if failure_count >= self.failure_threshold {
            if let Some(last_failure) = *self.last_failure.lock().unwrap() {
                return last_failure.elapsed() < self.recovery_timeout;
            }
        }

        false
    }

    fn record_failure(&self) {
        *self.failure_count.lock().unwrap() += 1;
        *self.last_failure.lock().unwrap() = Some(Instant::now());
    }
}
```

## Error Logging and Monitoring

### Structured Error Logging

```rust
use langweave::error::I18nError;
use serde_json::json;

fn log_error(operation: &str, error: &I18nError, context: Option<&str>) {
    let log_entry = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "level": "ERROR",
        "operation": operation,
        "error_type": error.error_type(),
        "error_message": error.to_string(),
        "context": context.unwrap_or(""),
        "recoverable": error.is_recoverable(),
    });

    eprintln!("{}", log_entry);
}

// Error classification extension
impl I18nError {
    fn error_type(&self) -> &str {
        match self {
            I18nError::EmptyInput => "input_validation",
            I18nError::UnsupportedLanguage(_) => "configuration",
            I18nError::ConfidenceTooLow => "quality",
            I18nError::TranslationFailed(_) => "service",
            I18nError::NetworkError => "network",
            I18nError::ConfigurationError(_) => "configuration",
            I18nError::ParseError(_) => "parsing",
            I18nError::IoError(_) => "io",
        }
    }

    fn is_recoverable(&self) -> bool {
        matches!(
            self,
            I18nError::NetworkError |
            I18nError::TranslationFailed(_) |
            I18nError::ConfidenceTooLow
        )
    }
}
```

### Error Metrics Collection

```rust
use langweave::error::I18nError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ErrorMetrics {
    counts: Arc<Mutex<HashMap<String, u64>>>,
}

impl ErrorMetrics {
    pub fn new() -> Self {
        Self {
            counts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_error(&self, error: &I18nError) {
        let error_type = error.error_type().to_string();
        let mut counts = self.counts.lock().unwrap();
        *counts.entry(error_type).or_insert(0) += 1;
    }

    pub fn get_error_counts(&self) -> HashMap<String, u64> {
        self.counts.lock().unwrap().clone()
    }

    pub fn reset(&self) {
        self.counts.lock().unwrap().clear();
    }
}
```

## Testing Error Conditions

### Unit Tests for Error Handling

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use langweave::{language_detector::LanguageDetector, error::I18nError};

    #[tokio::test]
    async fn test_empty_input_handling() {
        let detector = LanguageDetector::new();
        let result = detector.detect_async("").await;

        assert!(matches!(result, Err(I18nError::EmptyInput)));
    }

    #[tokio::test]
    async fn test_unsupported_language_handling() {
        let translator = Translator::new("en")?;
        let result = translator.translate("Hello", "en", "xyz").await;

        assert!(matches!(result, Err(I18nError::UnsupportedLanguage(_))));
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let result = safe_language_detection("").await;
        assert!(result.is_err());

        let result = safe_language_detection("Hello world").await;
        assert!(result.is_ok());
    }
}
```

## Best Practices

### Error Handling Guidelines

1. **Always Handle Errors**: Never use `.unwrap()` in production code
2. **Provide Context**: Include relevant context in error messages
3. **Log Strategically**: Log errors with structured data for monitoring
4. **Implement Fallbacks**: Provide graceful degradation when possible
5. **Test Error Paths**: Write tests for error conditions
6. **Monitor Errors**: Track error rates and patterns in production

### User Experience

1. **User-Friendly Messages**: Convert technical errors to user-friendly messages
2. **Progressive Degradation**: Offer reduced functionality rather than complete failure
3. **Clear Actions**: Tell users what they can do when errors occur
4. **Retry Mechanisms**: Implement automatic retries for transient errors

### Performance Considerations

1. **Avoid Error Overhead**: Don't use errors for control flow
2. **Cache Error States**: Remember persistent error conditions
3. **Circuit Breakers**: Prevent cascading failures in distributed systems
4. **Timeout Handling**: Set reasonable timeouts for all operations

By following these error handling patterns, you can build robust applications that gracefully handle various failure modes while providing a good user experience.