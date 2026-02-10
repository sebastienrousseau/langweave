# Branch Protection Configuration

## Repository Settings Required

To enforce CI pipeline compliance and block merges on failures, configure the following branch protection rules:

### Main Branch Protection Rules

**Branch:** `main`

#### Status Checks
- [x] Require status checks to pass before merging
- [x] Require branches to be up to date before merging

**Required Status Checks:**
- `🧹 Lint & Format Check (ubuntu-latest)`
- `🧹 Lint & Format Check (macos-latest)`
- `🔨 Build Check (ubuntu-latest)`
- `🔨 Build Check (macos-latest)`
- `🧪 Test Suite (100% Coverage) (ubuntu-latest)`
- `🧪 Test Suite (100% Coverage) (macos-latest)`
- `🔒 Security Audit`
- `🏗️ Architecture Compliance`
- `🔗 Integration Check (ubuntu-latest)`
- `🔗 Integration Check (macos-latest)`

#### Additional Protections
- [x] Restrict pushes that create files larger than 100 MB
- [x] Require pull request reviews before merging (minimum 1)
- [x] Dismiss stale pull request approvals when new commits are pushed
- [x] Require review from code owners
- [x] Restrict pushes to matching branches (admins only)
- [x] Allow force pushes (disabled)
- [x] Allow deletions (disabled)

### Feature Branch Protection Rules

**Branch Pattern:** `feat/*`

#### Status Checks
- [x] Require status checks to pass before merging to main
- [x] Require branches to be up to date before merging

**Required Status Checks:** (Same as main branch)

## CLI Configuration Commands

### Using GitHub CLI (gh)

```bash
# Enable branch protection for main
gh api repos/:owner/:repo/branches/main/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["🧹 Lint & Format Check (ubuntu-latest)","🧹 Lint & Format Check (macos-latest)","🔨 Build Check (ubuntu-latest)","🔨 Build Check (macos-latest)","🧪 Test Suite (100% Coverage) (ubuntu-latest)","🧪 Test Suite (100% Coverage) (macos-latest)","🔒 Security Audit","🏗️ Architecture Compliance","🔗 Integration Check (ubuntu-latest)","🔗 Integration Check (macos-latest)"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":1,"dismiss_stale_reviews":true,"require_code_owner_reviews":true}' \
  --field restrictions=null
```

## Verification

To verify branch protection is working:

1. Create a test branch
2. Make a change that violates one of the checks (e.g., add unused code)
3. Create a pull request
4. Confirm the PR cannot be merged until all checks pass

## Security Enforcement

### Zero-Tolerance Policies

- **NO** `|| true` on security-critical steps
- **NO** bypassing coverage requirements
- **NO** disabling linting checks
- **NO** formatting violations
- **NO** security vulnerabilities

### Emergency Override Process

In critical production emergencies only:

1. Document the emergency in an issue
2. Get approval from 2 repository admins
3. Create hotfix branch with minimal changes
4. Merge with admin override
5. Create follow-up PR to restore compliance within 24 hours

## Compliance Verification

Run this command to verify your branch meets all requirements:

```bash
# Run all checks locally before pushing
make ci-check  # If Makefile exists, or:

# Manual verification
cargo fmt --all -- --check && \
cargo clippy --all-targets --all-features --workspace -- -D warnings && \
cargo test --all-features --workspace && \
cargo audit && \
cargo llvm-cov --all-features --workspace --summary-only
```

## Contact

For questions about CI pipeline configuration or branch protection:
- Create an issue with label `ci/cd`
- Tag the `automaton` agent for infrastructure questions