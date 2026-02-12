# Architectural Layer Guidelines

## Overview

This project enforces strict architectural boundaries to maintain clean code separation, testability, and maintainability. The architecture follows a layered approach where **Core** modules must remain independent of **UI**, **Network**, and **Filesystem** layers.

## Layer Definitions

### Core Layer
**Location**: `src/core/`, `src/lib.rs`, `src/error.rs`, `src/language_detector*.rs`, `src/translator*.rs`, `src/translation*.rs`

**Purpose**: Contains pure business logic and domain models

**Allowed Dependencies**:
- ✅ Standard library (except `std::net`, direct `std::fs`)
- ✅ Data processing crates (`serde`, `thiserror`)
- ✅ Utility crates (`once_cell`, `lazy_static`)
- ✅ Core computation crates (`regex`, `smallvec`)

**Forbidden Dependencies**:
- ❌ Network crates (`tokio::net`, `reqwest`, `hyper`, `actix-web`)
- ❌ UI/GUI crates (`gtk`, `egui`, `tauri`, `druid`)
- ❌ Direct filesystem access (`notify`, `walkdir`, `glob`)
- ❌ System integration crates

### UI Layer
**Location**: `src/ui/`, `src/gui/`, `src/terminal/`

**Purpose**: Presentation logic and user interaction

**Dependencies**: Can depend on Core layer through well-defined interfaces

### Network Layer
**Location**: `src/network/`, `src/http/`, `src/api/`

**Purpose**: External communication and data fetching

**Dependencies**: Can depend on Core layer for business logic

### Filesystem Layer
**Location**: `src/fs/`, `src/storage/`, `src/persistence/`

**Purpose**: Data persistence and file operations

**Dependencies**: Can depend on Core layer for data models

## Dependency Rules

### 1. Dependency Direction
```
UI Layer     ──┐
Network Layer ─┼─→ Core Layer
Filesystem    ──┘
```

**Core never imports upward layers**

### 2. Interface-Based Integration

Instead of direct imports, use dependency injection:

```rust
// ❌ BAD: Core importing network directly
use reqwest::Client;

pub fn translate_remote(text: &str) -> Result<String, Error> {
    let client = Client::new();
    // Direct HTTP dependency in core
}

// ✅ GOOD: Core defining interface, network implementing it
#[async_trait]
pub trait TranslationProvider {
    async fn translate(&self, text: &str, lang: &str) -> Result<String, Error>;
}

// Core defines the interface
pub struct TranslationService<T: TranslationProvider> {
    provider: T,
}

// Network layer implements the interface
pub struct HttpTranslationProvider {
    client: reqwest::Client,
}

#[async_trait]
impl TranslationProvider for HttpTranslationProvider {
    async fn translate(&self, text: &str, lang: &str) -> Result<String, Error> {
        // HTTP implementation here
    }
}
```

### 3. Allowed Tokio Usage

Tokio is allowed for async coordination but **not** for network operations:

```rust
// ✅ ALLOWED: Async coordination
use tokio::task;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

// ❌ FORBIDDEN: Network operations
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::fs;  // Direct filesystem access
```

## Automated Enforcement

### CI/CD Guardrails

The architecture is enforced automatically in CI through:

1. **Source Code Analysis** (`scripts/check_architecture.py`)
   - Scans all Rust files for forbidden imports
   - Checks `use` statements against layer boundaries
   - Validates standard library usage

2. **Dependency Analysis** (GitHub Actions)
   - Examines `Cargo.toml` for forbidden dependencies
   - Validates feature flags don't enable forbidden functionality
   - Checks for circular dependencies

3. **Build-time Validation**
   - Ensures code compiles with minimal features
   - Validates all feature combinations work
   - Tests without any features enabled

### Running Locally

```bash
# Check architectural boundaries
python3 scripts/check_architecture.py

# Analyze dependency graph
cargo depgraph --all-features --workspace-only

# Test feature combinations
cargo check --no-default-features
cargo check --all-features
cargo check --features async
```

## Violation Examples

### Import Violations

```rust
// ❌ Core importing UI layer
use crate::ui::display_progress;

// ❌ Core importing network layer
use crate::network::http_client;

// ❌ Direct standard library networking
use std::net::TcpStream;

// ❌ Direct filesystem access
use std::fs::File;
use walkdir::WalkDir;
```

### Dependency Violations

```toml
# ❌ Forbidden dependencies in Cargo.toml
[dependencies]
reqwest = "0.11"      # Network layer dependency
tauri = "1.0"         # UI layer dependency
notify = "5.0"        # Filesystem layer dependency
```

## Migration Strategy

If you encounter violations, follow this migration path:

### Step 1: Extract Interface
Define a trait in the core layer:

```rust
pub trait FileReader {
    fn read_content(&self, path: &str) -> Result<String, Error>;
}
```

### Step 2: Move Implementation
Move the concrete implementation to the appropriate layer:

```rust
// In src/filesystem/reader.rs
impl FileReader for LocalFileReader {
    fn read_content(&self, path: &str) -> Result<String, Error> {
        std::fs::read_to_string(path).map_err(Into::into)
    }
}
```

### Step 3: Inject Dependency
Use dependency injection in your application:

```rust
// In main.rs or application setup
let file_reader = LocalFileReader::new();
let service = TranslationService::new(file_reader);
```

## Benefits

This architectural approach provides:

- **Testability**: Core logic can be unit tested without external dependencies
- **Flexibility**: Different implementations can be swapped easily
- **Maintainability**: Clear boundaries reduce coupling and complexity
- **Performance**: Core operations don't carry unnecessary overhead
- **Security**: Reduced attack surface in business logic

## Enforcement Status

- ✅ **Active**: All violations fail the build
- ✅ **Comprehensive**: Covers imports, dependencies, and features
- ✅ **Automated**: Runs on every PR and push
- ✅ **Documented**: Clear violation reporting with remediation steps

For questions about specific cases or exemption requests, please file an issue with the "architecture" label.