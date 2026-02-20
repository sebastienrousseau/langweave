# (c) 2026 Euxis Fleet. All rights reserved.

# Contributing to LangWeave

Welcome! We're thrilled that you're interested in contributing to LangWeave. Whether you're fixing bugs, adding features, improving documentation, or spreading the word, your contributions make LangWeave better for everyone.

## Quick Start for Contributors

1. **Fork the repository** on GitHub
2. **Clone your fork** locally: `git clone https://github.com/your-username/langweave.git`
3. **Create a feature branch**: `git checkout -b feature/your-feature-name`
4. **Make your changes** and test them
5. **Commit your changes**: `git commit -m "Add your feature"`
6. **Push to your fork**: `git push origin feature/your-feature-name`
7. **Create a Pull Request** on GitHub

## Ways to Contribute

### 🐛 Bug Reports

Found a bug? Help us fix it by providing detailed information:

- **Use the bug report template** when creating an issue
- **Include steps to reproduce** the problem
- **Provide system information** (OS, Rust version, LangWeave version)
- **Include error messages** and stack traces when available
- **Add test cases** that demonstrate the bug

**Example Bug Report:**
```
**Bug Description**: Language detection fails for short German texts

**Steps to Reproduce**:
1. Create LanguageDetector with default settings
2. Call detect("Hallo")
3. Expected: "de", Actual: "en"

**Environment**:
- OS: macOS 14.0
- Rust: 1.85.0
- LangWeave: 0.0.2

**Additional Context**:
Works correctly for longer German texts (20+ characters)
```

### ✨ Feature Requests

Have an idea for a new feature? We'd love to hear it:

- **Describe the problem** your feature would solve
- **Explain the proposed solution** in detail
- **Consider alternatives** and explain why your approach is best
- **Provide use cases** where this feature would be helpful
- **Think about backward compatibility**

### 💻 Code Contributions

#### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/sebastienrousseau/langweave.git
cd langweave

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_usage_example
```

#### Code Style and Standards

We follow Rust best practices and maintain high code quality:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Check documentation
cargo doc --no-deps

# Run all quality checks
make test    # Runs tests, linting, and formatting checks
```

**Code Style Guidelines:**
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Write comprehensive documentation comments (`///`)
- Include examples in documentation when helpful
- Keep functions focused and single-purpose
- Use meaningful variable and function names
- Handle errors explicitly, avoid `.unwrap()` in library code

#### Writing Tests

All new code should include comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use langweave::error::I18nError;

    #[tokio::test]
    async fn test_language_detection_english() {
        let detector = LanguageDetector::new();
        let result = detector.detect_async("Hello, world!").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "en");
    }

    #[test]
    fn test_error_handling_empty_input() {
        let detector = LanguageDetector::new();
        let result = detector.detect("");

        assert!(matches!(result, Err(I18nError::EmptyInput)));
    }
}
```

**Testing Guidelines:**
- Write unit tests for all public functions
- Include integration tests for complex workflows
- Test both success and error cases
- Use descriptive test names that explain what's being tested
- Keep tests focused and independent
- Add benchmark tests for performance-critical code

#### Adding Examples

Help others learn by adding examples:

```rust
//! Basic usage example for LangWeave
//!
//! This example demonstrates:
//! - Language detection
//! - Error handling
//! - Basic translation

use langweave::{language_detector::LanguageDetector, error::I18nError};
use langweave::language_detector_trait::LanguageDetectorTrait;

#[tokio::main]
async fn main() -> Result<(), I18nError> {
    // Create detector
    let detector = LanguageDetector::new();

    // Detect language
    let text = "Bonjour le monde!";
    match detector.detect_async(text).await {
        Ok(language) => {
            println!("Input: {}", text);
            println!("Detected language: {}", language);
        }
        Err(e) => {
            eprintln!("Detection failed: {}", e);
        }
    }

    Ok(())
}
```

### 📚 Documentation Contributions

Good documentation is crucial for any library:

#### Improving API Documentation

- Add missing documentation comments
- Include usage examples in doc comments
- Explain complex concepts clearly
- Link to related functions and types

#### Writing Guides and Tutorials

- Create step-by-step tutorials for common use cases
- Write integration guides for popular frameworks
- Document best practices and common patterns
- Add troubleshooting guides

#### Updating Examples

- Ensure all examples work with current API
- Add comments explaining what each part does
- Create examples for new features
- Update existing examples to show best practices

### 🌐 Localization

Help make LangWeave more accessible:

- Translate documentation to other languages
- Add support for new language pairs in translation
- Improve language detection accuracy for specific languages
- Add locale-specific formatting and conventions

## Development Workflow

### Branch Naming

Use descriptive branch names that indicate the type of change:

- `feature/add-batch-translation` - New features
- `fix/detection-accuracy-issue` - Bug fixes
- `docs/improve-error-handling-guide` - Documentation
- `refactor/simplify-api` - Code refactoring
- `test/add-integration-tests` - Test improvements

### Commit Messages

Write clear, descriptive commit messages:

```
Add support for confidence thresholds in language detection

- Add confidence_threshold field to LanguageDetector
- Improve detection accuracy algorithms
- Update detect methods to respect threshold
- Add tests for confidence-based detection
- Update documentation and examples

Fixes #123
```

**Commit Message Guidelines:**
- Use imperative mood ("Add feature" not "Added feature")
- Keep first line under 50 characters
- Include detailed description for complex changes
- Reference issue numbers when applicable
- Separate logical changes into separate commits

### Pull Request Guidelines

When you're ready to submit your changes:

1. **Update your branch** with the latest main: `git rebase main`
2. **Run all tests** and ensure they pass: `cargo test`
3. **Update documentation** if needed
4. **Add changelog entry** if it's a notable change
5. **Create detailed PR description**

**PR Description Template:**
```markdown
## Summary
Brief description of what this PR does.

## Changes Made
- List of specific changes
- Include any breaking changes
- Note new dependencies

## Testing
- How you tested the changes
- Which test cases were added
- Performance impact (if any)

## Documentation
- Documentation updates made
- Examples updated/added

## Related Issues
Closes #123
References #456
```

### Review Process

All contributions go through code review:

1. **Automated checks** run first (tests, linting, formatting)
2. **Manual review** by maintainers focuses on:
   - Code correctness and style
   - Test coverage and quality
   - Documentation completeness
   - Performance implications
   - API design and usability

3. **Feedback incorporation** - address review comments
4. **Final approval** and merge

## Specific Contribution Areas

### Language Detection Improvements

Help improve detection accuracy:

- Add training data for new languages
- Implement better confidence scoring algorithms
- Optimize detection speed for large texts
- Add support for mixed-language texts

### Translation Engine Enhancements

Contribute to translation capabilities:

- Add new language pair support
- Implement context-aware translation
- Add quality assessment features
- Optimize translation caching

### Performance Optimizations

Help make LangWeave faster:

- Profile and optimize hot code paths
- Implement better caching strategies
- Reduce memory allocations
- Add benchmarking tests

### Error Handling Improvements

Enhance error handling and debugging:

- Add more specific error types
- Improve error messages with actionable advice
- Add error recovery mechanisms
- Create debugging utilities

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please:

- Be respectful and considerate in all interactions
- Welcome newcomers and help them get started
- Provide constructive feedback
- Focus on what's best for the community

### Communication

- **GitHub Issues** - Bug reports, feature requests, questions
- **GitHub Discussions** - General discussion, ideas, help
- **Pull Request Comments** - Code-specific feedback

### Getting Help

Need help with your contribution?

- **Check existing issues** and discussions first
- **Ask questions** in GitHub Discussions
- **Join our community** chat (if available)
- **Read the documentation** and examples

## Recognition

We value all contributions! Contributors are recognized:

- In the project's AUTHORS.md file
- In release notes for significant contributions
- Through GitHub's contributor recognition features
- In project documentation and examples

## Legal

By contributing to LangWeave, you agree that:

- Your contributions will be licensed under the same license as the project (Apache 2.0 / MIT)
- You have the right to submit the contribution
- You understand that your contribution may be redistributed

## Questions?

Don't hesitate to ask if you have questions:

- **General questions**: Open a GitHub Discussion
- **Bug reports**: Create an Issue with the bug template
- **Feature ideas**: Create an Issue with the feature template
- **Contribution help**: Comment on existing issues or discussions

Thank you for helping make LangWeave better! 🚀
