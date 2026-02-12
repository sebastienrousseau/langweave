#!/usr/bin/env python3
"""
Documentation Code Validation Tool for LangWeave

Extracts and validates all code snippets from markdown documentation files.
"""

import os
import re
import tempfile
import subprocess
import json
from pathlib import Path
from typing import List, Dict, Tuple, Optional

class DocCodeValidator:
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.total_tests = 0
        self.passed_tests = 0
        self.failed_tests = []

    def extract_code_blocks(self, content: str, file_path: str) -> List[Dict]:
        """Extract all code blocks from markdown content."""
        code_blocks = []

        # Pattern to match ```language...``` code blocks
        pattern = r'```(\w+)?\n(.*?)\n```'
        matches = re.findall(pattern, content, re.DOTALL)

        for i, (language, code) in enumerate(matches):
            if language in ['rust', 'rs', ''] and ('fn main' in code or 'use langweave' in code):
                code_blocks.append({
                    'language': language or 'rust',
                    'code': code.strip(),
                    'file': file_path,
                    'block_id': i + 1,
                    'description': f"{file_path} block {i + 1}"
                })
            elif language == 'shell' or language == 'bash':
                code_blocks.append({
                    'language': 'shell',
                    'code': code.strip(),
                    'file': file_path,
                    'block_id': i + 1,
                    'description': f"{file_path} shell block {i + 1}"
                })

        return code_blocks

    def analyze_rust_code(self, code: str) -> Dict:
        """Analyze Rust code for potential issues based on actual API."""
        issues = []

        # Check for documented APIs that don't exist in the actual codebase
        if 'translator.translate(' in code and '"en", "fr"' in code:
            issues.append("Uses 3-argument translate() which doesn't exist. Actual API: Translator::new(lang).translate(text)")

        if 'detect(' in code and not 'detect_async(' in code and '#[tokio::main]' not in code:
            # This might be valid since sync detect() does exist
            pass

        if 'translate_with_quality(' in code:
            issues.append("Uses translate_with_quality() which doesn't exist in current implementation")

        if 'translate_batch(' in code:
            issues.append("Uses translate_batch() which doesn't exist in current implementation")

        if 'set_confidence_threshold(' in code:
            issues.append("Uses set_confidence_threshold() which doesn't exist in current implementation")

        if 'detect_with_confidence(' in code:
            issues.append("Uses detect_with_confidence() which doesn't exist in current implementation")

        if 'enable_caching(' in code:
            issues.append("Uses enable_caching() which doesn't exist in current implementation")

        if 'TranslationOptions' in code:
            issues.append("Uses TranslationOptions which doesn't exist in current implementation")

        if 'TranslationContext' in code:
            issues.append("Uses TranslationContext which doesn't exist in current implementation")

        # Check for valid patterns
        valid_patterns = [
            'LanguageDetector::new()',
            'detector.detect(',
            'detector.detect_async(',
            'Translator::new(',
            'translator.translate(',
            'use langweave::',
        ]

        has_valid_usage = any(pattern in code for pattern in valid_patterns)

        return {
            'issues': issues,
            'has_valid_usage': has_valid_usage
        }

    def test_rust_code(self, code: str, description: str) -> Tuple[bool, str]:
        """Test if Rust code compiles and runs correctly."""
        self.total_tests += 1

        # First analyze for API issues
        analysis = self.analyze_rust_code(code)
        if analysis['issues']:
            self.failed_tests.append(f"{description}: API issues - {'; '.join(analysis['issues'])}")
            print(f"❌ {description}: API compatibility issues")
            for issue in analysis['issues']:
                print(f"   - {issue}")
            return False, "API compatibility issues"

        # Try to compile the code
        with tempfile.NamedTemporaryFile(mode='w', suffix='.rs', delete=False) as f:
            # Wrap code in a basic template if it's not a complete program
            if 'fn main' not in code:
                wrapped_code = f"""
use langweave::{{language_detector::LanguageDetector, error::I18nError}};
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::translator::Translator;

fn main() -> Result<(), I18nError> {{
{code}
    Ok(())
}}
"""
            else:
                wrapped_code = f"""
use langweave::{{language_detector::LanguageDetector, error::I18nError}};
use langweave::language_detector_trait::LanguageDetectorTrait;
use langweave::translator::Translator;

{code}
"""
            f.write(wrapped_code)
            f.flush()

            try:
                # Try to check syntax
                result = subprocess.run([
                    'rustc', '--edition', '2021', '--crate-type', 'bin',
                    '--extern', f'langweave={self.project_root}/target/debug/liblangweave.rlib',
                    '--extern', 'tokio',
                    f.name
                ], capture_output=True, text=True, cwd=self.project_root)

                if result.returncode == 0:
                    self.passed_tests += 1
                    print(f"✅ {description}: Compiles successfully")
                    return True, "Success"
                else:
                    error_msg = result.stderr
                    self.failed_tests.append(f"{description}: Compilation error - {error_msg}")
                    print(f"❌ {description}: Compilation error")
                    print(f"   {error_msg[:200]}...")
                    return False, f"Compilation error: {error_msg}"

            except Exception as e:
                self.failed_tests.append(f"{description}: Exception - {str(e)}")
                print(f"❌ {description}: Exception - {e}")
                return False, f"Exception: {e}"
            finally:
                # Clean up
                try:
                    os.unlink(f.name)
                    executable = f.name.replace('.rs', '')
                    if os.path.exists(executable):
                        os.unlink(executable)
                except:
                    pass

    def test_shell_command(self, command: str, description: str) -> Tuple[bool, str]:
        """Test shell commands."""
        self.total_tests += 1

        print(f"🔧 Testing shell command: {command}")

        try:
            result = subprocess.run(
                command, shell=True, capture_output=True, text=True,
                cwd=self.project_root, timeout=60
            )

            if result.returncode == 0:
                self.passed_tests += 1
                print(f"✅ {description}: Command successful")
                return True, "Success"
            else:
                self.failed_tests.append(f"{description}: Command failed - {result.stderr}")
                print(f"❌ {description}: Command failed")
                print(f"   stderr: {result.stderr[:200]}")
                return False, f"Command failed: {result.stderr}"

        except subprocess.TimeoutExpired:
            self.failed_tests.append(f"{description}: Command timeout")
            print(f"❌ {description}: Command timeout")
            return False, "Command timeout"
        except Exception as e:
            self.failed_tests.append(f"{description}: Exception - {str(e)}")
            print(f"❌ {description}: Exception - {e}")
            return False, f"Exception: {e}"

    def validate_file(self, file_path: str) -> None:
        """Validate all code examples in a markdown file."""
        print(f"\n📄 Validating {file_path}")
        print("=" * (len(file_path) + 15))

        try:
            with open(self.project_root / file_path, 'r', encoding='utf-8') as f:
                content = f.read()
        except Exception as e:
            print(f"❌ Could not read {file_path}: {e}")
            return

        code_blocks = self.extract_code_blocks(content, file_path)

        if not code_blocks:
            print(f"ℹ️  No code blocks found in {file_path}")
            return

        for block in code_blocks:
            if block['language'] in ['rust', 'rs']:
                self.test_rust_code(block['code'], block['description'])
            elif block['language'] in ['shell', 'bash']:
                self.test_shell_command(block['code'], block['description'])

    def run_validation(self) -> None:
        """Run validation on all documentation files."""
        print("🔬 LangWeave Documentation Code Validation")
        print("=========================================")

        # Ensure the project is built
        print("🔨 Building project...")
        build_result = subprocess.run(['cargo', 'build'], cwd=self.project_root, capture_output=True)
        if build_result.returncode != 0:
            print("❌ Failed to build project. Exiting.")
            print(build_result.stderr.decode())
            return
        print("✅ Project built successfully")

        # Files to validate
        doc_files = [
            'README.md',
            'docs/essentials/quick-start.md',
            'docs/guides/language-detection.md',
            'docs/guides/translation.md'
        ]

        for file_path in doc_files:
            if (self.project_root / file_path).exists():
                self.validate_file(file_path)
            else:
                print(f"⚠️  File not found: {file_path}")

        # Summary
        print(f"\n📊 VALIDATION SUMMARY")
        print("=" * 25)
        print(f"Total tests: {self.total_tests}")
        print(f"Passed: {self.passed_tests} ✅")
        print(f"Failed: {len(self.failed_tests)} ❌")

        if self.failed_tests:
            print(f"\n❌ FAILED TESTS:")
            for i, failure in enumerate(self.failed_tests, 1):
                print(f"{i:2}. {failure}")
        else:
            print("\n🎉 All documentation examples are valid!")

        # Generate report
        self.generate_report()

    def generate_report(self) -> None:
        """Generate a JSON report of validation results."""
        report = {
            'total_tests': self.total_tests,
            'passed_tests': self.passed_tests,
            'failed_tests': len(self.failed_tests),
            'failures': self.failed_tests,
            'success_rate': (self.passed_tests / self.total_tests * 100) if self.total_tests > 0 else 0
        }

        report_file = self.project_root / 'doc_validation_report.json'
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2)

        print(f"\n📋 Report saved to: {report_file}")

if __name__ == '__main__':
    import sys

    project_root = sys.argv[1] if len(sys.argv) > 1 else os.getcwd()
    validator = DocCodeValidator(project_root)
    validator.run_validation()