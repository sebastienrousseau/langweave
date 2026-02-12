use langweave::language_detector::LanguageDetector;
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::error::I18nError;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    let detector = LanguageDetector::new();
    let lang = detector.detect_async("Hello, world!").await?;
    println!("Detected: {}", lang);
    Ok(())
}