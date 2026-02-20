#!/usr/bin/env python3
"""
Simplified Architectural Guardrail Script (No External Dependencies)
===================================================================
Basic version that only requires Python standard library.
"""

import os
import sys
import re
from pathlib import Path
from typing import List
import json

class ArchitectureViolation:
    def __init__(self, file_path: str, line: int, violation_type: str, detail: str):
        self.file_path = file_path
        self.line = line
        self.violation_type = violation_type
        self.detail = detail

    def __str__(self):
        return f"{self.file_path}:{self.line} [{self.violation_type}] {self.detail}"

def find_rust_files() -> List[Path]:
    """Find all Rust source files in the project."""
    files = []
    src_path = Path("src")
    if src_path.exists():
        for rust_file in src_path.rglob("*.rs"):
            files.append(rust_file)
    return files

def check_forbidden_imports(file_path: Path) -> List[ArchitectureViolation]:
    """Check a Rust file for forbidden imports."""
    violations = []

    # Define what we consider core files and forbidden patterns
    forbidden_patterns = [
        r'tokio::net::',
        r'tokio::fs::',
        r'std::net::',
        r'std::fs::',
        r'use\s+reqwest',
        r'use\s+hyper',
        r'use\s+actix_web',
        r'use\s+warp',
        r'use\s+gtk',
        r'use\s+egui',
        r'use\s+tauri',
        r'use\s+walkdir',
        r'use\s+notify',
    ]

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except (UnicodeDecodeError, IOError):
        return violations

    for line_num, line in enumerate(lines, 1):
        line_stripped = line.strip()

        for pattern in forbidden_patterns:
            if re.search(pattern, line_stripped):
                violations.append(ArchitectureViolation(
                    str(file_path), line_num, "FORBIDDEN_IMPORT",
                    f"Core module imports forbidden layer: {line_stripped}"
                ))

    return violations

def check_cargo_toml() -> List[ArchitectureViolation]:
    """Basic check of Cargo.toml for forbidden dependencies."""
    violations = []
    cargo_path = Path("Cargo.toml")

    if not cargo_path.exists():
        return violations

    forbidden_deps = [
        'reqwest', 'hyper', 'actix-web', 'warp', 'axum', 'surf', 'ureq',
        'gtk', 'egui', 'tauri', 'druid', 'iced', 'conrod', 'cursive', 'tui',
        'notify', 'walkdir', 'glob', 'directories'
    ]

    try:
        with open(cargo_path, 'r') as f:
            content = f.read()

        for dep in forbidden_deps:
            # Simple pattern matching for dependencies
            if re.search(f'^{dep}\s*=', content, re.MULTILINE):
                violations.append(ArchitectureViolation(
                    str(cargo_path), 0, "FORBIDDEN_DEPENDENCY",
                    f"Core modules cannot use forbidden dependency '{dep}'"
                ))

    except IOError:
        pass

    return violations

def main():
    """Main entry point."""
    print("🔍 Checking architectural boundaries (simplified)...")

    all_violations = []

    # Check source files
    print("📁 Analyzing source files...")
    rust_files = find_rust_files()

    for file_path in rust_files:
        violations = check_forbidden_imports(file_path)
        all_violations.extend(violations)

    # Check Cargo.toml
    print("📦 Analyzing Cargo.toml...")
    cargo_violations = check_cargo_toml()
    all_violations.extend(cargo_violations)

    # Report results
    if all_violations:
        print(f"\n❌ Found {len(all_violations)} architectural violation(s):")
        for violation in all_violations:
            print(f"  - {violation}")

        # Save simple report
        with open("architecture_report.json", "w") as f:
            json.dump([
                {
                    "file": v.file_path,
                    "line": v.line,
                    "type": v.violation_type,
                    "detail": v.detail
                }
                for v in all_violations
            ], f, indent=2)

        sys.exit(1)
    else:
        print("\n✅ No architectural violations found!")
        sys.exit(0)

if __name__ == "__main__":
    main()