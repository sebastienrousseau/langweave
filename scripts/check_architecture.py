#!/usr/bin/env python3
"""
Architectural Guardrail Script
==============================
Prevents Core layer modules from importing UI, Network, or Filesystem layers.

This script enforces architectural boundaries by:
1. Analyzing Rust source files for forbidden imports
2. Checking Cargo.toml dependencies for layer violations
3. Failing the build if Core modules violate architectural constraints

Usage: python3 scripts/check_architecture.py
Exit code: 0 (clean) | 1 (violations found)
"""

import os
import sys
import re
import toml
from pathlib import Path
from typing import List, Tuple, Set
import json

# Architectural layer definitions
LAYER_DEFINITIONS = {
    "core": {
        "patterns": ["src/core/**/*.rs", "src/lib.rs", "src/error.rs", "src/language_detector*.rs", "src/translator*.rs", "src/translation*.rs"],
        "forbidden_imports": ["ui", "network", "filesystem", "web", "http", "tcp", "gui"],
        "forbidden_deps": {
            # Network/HTTP dependencies
            "tokio": ["net", "tcp", "udp"],  # Allow other tokio features, forbid networking
            "reqwest": "*",
            "hyper": "*",
            "actix-web": "*",
            "warp": "*",
            "axum": "*",
            "surf": "*",
            "ureq": "*",

            # UI/GUI dependencies
            "gtk": "*",
            "egui": "*",
            "tauri": "*",
            "druid": "*",
            "iced": "*",
            "conrod": "*",
            "cursive": "*",
            "tui": "*",
            "crossterm": "*",

            # Filesystem dependencies (core should use abstracted I/O)
            "notify": "*",  # File watching
            "walkdir": "*",  # Directory traversal
            "glob": "*",  # File globbing
            "directories": "*",  # System directories
        }
    }
}

# Network-related standard library imports
FORBIDDEN_STD_IMPORTS = [
    "std::net",
    "std::fs",  # Direct filesystem access
    "std::path::Path",  # Allow PathBuf for abstractions, forbid direct Path usage
]

class ArchitectureViolation:
    def __init__(self, file_path: str, line: int, violation_type: str, detail: str):
        self.file_path = file_path
        self.line = line
        self.violation_type = violation_type
        self.detail = detail

    def __str__(self):
        return f"{self.file_path}:{self.line} [{self.violation_type}] {self.detail}"

def find_rust_files(patterns: List[str]) -> List[Path]:
    """Find Rust files matching the given patterns."""
    files = []
    for pattern in patterns:
        if "**" in pattern:
            # Handle glob patterns
            base_path = pattern.split("**")[0]
            if os.path.exists(base_path):
                for rust_file in Path(base_path).rglob("*.rs"):
                    files.append(rust_file)
        else:
            # Handle explicit files
            if os.path.exists(pattern):
                files.append(Path(pattern))
    return files

def check_forbidden_imports(file_path: Path, forbidden_modules: List[str]) -> List[ArchitectureViolation]:
    """Check a Rust file for forbidden module imports."""
    violations = []

    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except (UnicodeDecodeError, IOError):
        return violations

    for line_num, line in enumerate(lines, 1):
        line = line.strip()

        # Check use statements
        if line.startswith('use '):
            for forbidden in forbidden_modules:
                if f"::{forbidden}::" in line or line.endswith(f"::{forbidden}"):
                    violations.append(ArchitectureViolation(
                        str(file_path), line_num, "FORBIDDEN_IMPORT",
                        f"Core module imports forbidden layer '{forbidden}': {line}"
                    ))

        # Check for forbidden standard library imports
        for forbidden_std in FORBIDDEN_STD_IMPORTS:
            if forbidden_std in line and ("use " in line or "extern " in line):
                violations.append(ArchitectureViolation(
                    str(file_path), line_num, "FORBIDDEN_STD_IMPORT",
                    f"Core module uses forbidden std import '{forbidden_std}': {line}"
                ))

    return violations

def check_cargo_dependencies() -> List[ArchitectureViolation]:
    """Check Cargo.toml for forbidden dependencies in core modules."""
    violations = []
    cargo_path = Path("Cargo.toml")

    if not cargo_path.exists():
        return violations

    try:
        cargo_data = toml.load(cargo_path)
    except Exception as e:
        violations.append(ArchitectureViolation(
            str(cargo_path), 0, "CARGO_PARSE_ERROR",
            f"Failed to parse Cargo.toml: {e}"
        ))
        return violations

    dependencies = cargo_data.get("dependencies", {})
    forbidden_deps = LAYER_DEFINITIONS["core"]["forbidden_deps"]

    for dep_name, dep_config in dependencies.items():
        if dep_name in forbidden_deps:
            forbidden_features = forbidden_deps[dep_name]

            if forbidden_features == "*":
                # Completely forbidden dependency
                violations.append(ArchitectureViolation(
                    str(cargo_path), 0, "FORBIDDEN_DEPENDENCY",
                    f"Core modules cannot use forbidden dependency '{dep_name}'"
                ))
            elif isinstance(forbidden_features, list):
                # Check if forbidden features are enabled
                if isinstance(dep_config, dict) and "features" in dep_config:
                    enabled_features = dep_config["features"]
                    for forbidden_feature in forbidden_features:
                        if forbidden_feature in enabled_features:
                            violations.append(ArchitectureViolation(
                                str(cargo_path), 0, "FORBIDDEN_FEATURE",
                                f"Core modules cannot use forbidden feature '{forbidden_feature}' of '{dep_name}'"
                            ))
                elif isinstance(dep_config, dict) and dep_config.get("default-features", True):
                    # Check if default features include forbidden ones
                    # This would require external crate metadata, so we warn
                    violations.append(ArchitectureViolation(
                        str(cargo_path), 0, "POTENTIAL_VIOLATION",
                        f"Core modules use '{dep_name}' with default features - verify no forbidden features: {forbidden_features}"
                    ))

    return violations

def check_tokio_usage() -> List[ArchitectureViolation]:
    """Special check for tokio usage patterns."""
    violations = []
    core_files = find_rust_files(LAYER_DEFINITIONS["core"]["patterns"])

    tokio_network_patterns = [
        r'tokio::net::',
        r'tokio::fs::',
        r'TcpListener',
        r'TcpStream',
        r'UdpSocket',
    ]

    for file_path in core_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')
        except (UnicodeDecodeError, IOError):
            continue

        for line_num, line in enumerate(lines, 1):
            for pattern in tokio_network_patterns:
                if re.search(pattern, line):
                    violations.append(ArchitectureViolation(
                        str(file_path), line_num, "TOKIO_NETWORK_USAGE",
                        f"Core module uses tokio networking: {line.strip()}"
                    ))

    return violations

def generate_report(violations: List[ArchitectureViolation]) -> str:
    """Generate a formatted violation report."""
    if not violations:
        return "✅ No architectural violations found. All layer boundaries are respected."

    report = f"❌ Found {len(violations)} architectural violation(s):\n\n"

    # Group by violation type
    by_type = {}
    for v in violations:
        if v.violation_type not in by_type:
            by_type[v.violation_type] = []
        by_type[v.violation_type].append(v)

    for vtype, vlist in by_type.items():
        report += f"## {vtype} ({len(vlist)} violations)\n"
        for violation in vlist:
            report += f"  - {violation}\n"
        report += "\n"

    report += "\n## Remediation\n"
    report += "Core modules must not directly import UI, Network, or Filesystem layers.\n"
    report += "Instead, use dependency injection or abstract interfaces.\n"
    report += "For more info, see: docs/architecture/layer-guidelines.md\n"

    return report

def main():
    """Main entry point for architectural guardrail checking."""
    print("🔍 Checking architectural boundaries...")

    all_violations = []

    # Check source code imports
    print("📁 Analyzing source files...")
    core_files = find_rust_files(LAYER_DEFINITIONS["core"]["patterns"])
    forbidden_imports = LAYER_DEFINITIONS["core"]["forbidden_imports"]

    for file_path in core_files:
        violations = check_forbidden_imports(file_path, forbidden_imports)
        all_violations.extend(violations)

    # Check Cargo.toml dependencies
    print("📦 Analyzing dependencies...")
    dep_violations = check_cargo_dependencies()
    all_violations.extend(dep_violations)

    # Special tokio check
    print("🔄 Checking tokio usage patterns...")
    tokio_violations = check_tokio_usage()
    all_violations.extend(tokio_violations)

    # Generate and print report
    report = generate_report(all_violations)
    print("\n" + "="*60)
    print("ARCHITECTURAL GUARDRAIL REPORT")
    print("="*60)
    print(report)

    # Save report for CI artifacts
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

    # Exit with error if violations found
    if all_violations:
        print(f"\n❌ Build FAILED: {len(all_violations)} architectural violations found")
        sys.exit(1)
    else:
        print("\n✅ Build PASSED: Architecture is clean")
        sys.exit(0)

if __name__ == "__main__":
    main()