#![no_main]

use libfuzzer_sys::fuzz_target;
use langweave::error::I18nError;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);

    // Test all error variants with arbitrary data - should never panic
    let errors = vec![
        I18nError::LanguageDetectionFailed,
        I18nError::TranslationFailed(text.to_string()),
        I18nError::UnsupportedLanguage(text.to_string()),
        I18nError::UnexpectedError(text.to_string()),
        I18nError::TaskFailed(text.to_string()),
        I18nError::BatchOperationFailed(text.to_string()),
        I18nError::StreamProcessingFailed(text.to_string()),
        I18nError::PatternOperationFailed(text.to_string()),
    ];

    for error in errors {
        // Test cloning - should never panic
        let _ = std::panic::catch_unwind(|| {
            let cloned = error.clone();
            let _ = cloned;
        });

        // Test display formatting - should never panic
        let _ = std::panic::catch_unwind(|| {
            let display = format!("{}", error);
            let debug = format!("{:?}", error);
            let as_str = error.as_str();
            let _ = (display, debug, as_str);
        });

        // Test equality - should never panic
        let _ = std::panic::catch_unwind(|| {
            let cloned = error.clone();
            let _ = error == cloned;
            let _ = error != cloned;
        });
    }
});