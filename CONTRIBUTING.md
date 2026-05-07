# Contributing to Azaka

Thank you for your interest in contributing to Azaka! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Code Style](#code-style)
- [Issue Labels](#issue-labels)
- [Bounty Program](#bounty-program)
- [Community](#community)

## Getting Started

### Prerequisites

- Rust 1.81.0 or later
- Node.js 18 or later
- Stellar CLI (`cargo install --locked stellar-cli`)
- Git

### First-Time Contributors

Look for issues labeled `good first issue` — these are specifically chosen to be approachable for newcomers.

## Development Setup

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/azaka.git
cd azaka
```

### 2. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Node dependencies for SDK
cd sdk/typescript
npm install
cd ../..
```

### 3. Build Contracts

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 4. Run Tests

```bash
# Rust tests
cargo test

# SDK tests
cd sdk/typescript
npm test
```

### 5. Set Up Pre-Commit Hooks (Optional)

```bash
# Install pre-commit hooks
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
EOF

chmod +x .git/hooks/pre-commit
```

## Project Structure

```
azaka/
├── contracts/          # Soroban smart contracts
│   ├── trade/         # Core trade contract
│   ├── escrow/        # Escrow contract
│   ├── document/      # Document registry
│   └── registry/      # Participant registry
├── sdk/               # Client SDKs
│   └── typescript/    # TypeScript SDK
├── tests/             # Integration tests
├── scripts/           # Deployment and utility scripts
├── docs/              # Documentation
└── .github/           # CI/CD workflows
```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-partial-payments` - New features
- `fix/escrow-double-release` - Bug fixes
- `docs/update-bank-guide` - Documentation
- `refactor/optimize-storage` - Code refactoring

### Commit Messages

Follow conventional commits:

```
feat: add partial payment support to escrow contract
fix: prevent double-release in escrow contract
docs: update bank integration guide with custodian info
test: add integration test for trade expiry
refactor: optimize document storage keys
```

### Code Changes

1. **Contracts**: All contract changes must include:
   - Rust doc comments (`///`)
   - Unit tests
   - Integration tests (if applicable)
   - Updated documentation

2. **SDK**: All SDK changes must include:
   - TypeScript types
   - JSDoc comments
   - Unit tests
   - Updated README

3. **Documentation**: All doc changes must:
   - Be clear and concise
   - Include examples where applicable
   - Be tested (code examples must work)

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific contract tests
cargo test -p azaka-trade

# Run with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Build contracts first
cargo build --target wasm32-unknown-unknown --release

# Run integration tests
cargo test --test integration_test
```

### SDK Tests

```bash
cd sdk/typescript
npm test
```

### Manual Testing on Testnet

```bash
# Deploy to testnet
./scripts/deploy.sh

# Run seed script
cd sdk/typescript
npm run seed
```

## Pull Request Process

### Before Submitting

- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages follow conventions

### PR Checklist

When opening a PR, include:

1. **Description**: What does this PR do?
2. **Motivation**: Why is this change needed?
3. **Testing**: How was this tested?
4. **Breaking Changes**: Does this break existing functionality?
5. **Screenshots**: If UI changes, include screenshots

### PR Template

```markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- Change 1
- Change 2

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manually tested on testnet

## Breaking Changes
None / List breaking changes

## Checklist
- [ ] Code compiles
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Clippy clean
```

### Review Process

1. Maintainer reviews PR
2. Feedback provided (if needed)
3. Contributor addresses feedback
4. Maintainer approves and merges

**Timeline**: Most PRs reviewed within 48 hours.

## Code Style

### Rust

Follow standard Rust conventions:

```rust
// Good
pub fn create_trade(
    env: Env,
    exporter: Address,
    importer: Address,
) -> u64 {
    // Implementation
}

// Bad
pub fn create_trade(env:Env,exporter:Address,importer:Address)->u64{
    // Implementation
}
```

**Key points**:
- Use `cargo fmt` for formatting
- No `unwrap()` in production code (use `?` or `unwrap_or`)
- All public functions have doc comments
- Use typed errors (no raw `u32`)
- Use `Symbol` for storage keys (no raw strings)

### TypeScript

Follow standard TypeScript conventions:

```typescript
// Good
export async function createTrade(
  params: CreateTradeParams
): Promise<bigint> {
  // Implementation
}

// Bad
export async function createTrade(params:any):Promise<any>{
  // Implementation
}
```

**Key points**:
- Use TypeScript strict mode
- All functions have JSDoc comments
- Use proper types (no `any`)
- Use async/await (not callbacks)
- Use ESLint for linting

### Documentation

Follow these conventions:

```markdown
# Title (H1)

Brief introduction.

## Section (H2)

Content.

### Subsection (H3)

More content.

**Bold** for emphasis.
*Italic* for terms.
`code` for inline code.

\`\`\`language
code block
\`\`\`
```

## Issue Labels

### Type Labels
- `bug` - Something isn't working
- `feature` - New feature request
- `docs` - Documentation improvements
- `refactor` - Code refactoring
- `test` - Test improvements

### Priority Labels
- `critical` - Critical bug or security issue
- `high` - High priority
- `medium` - Medium priority
- `low` - Low priority

### Status Labels
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed
- `in progress` - Someone is working on this
- `blocked` - Blocked by another issue

### Component Labels
- `contracts` - Smart contract changes
- `sdk` - SDK changes
- `bank-integration` - Bank integration
- `docs` - Documentation

## Bounty Program

Azaka offers bounties for significant contributions:

### Bounty Tiers

**Tier 1: $100–500**
- Critical bug fixes
- Security improvements
- Major feature implementations

**Tier 2: $50–100**
- Important bug fixes
- Medium features
- Significant documentation

**Tier 3: $10–50**
- Minor bug fixes
- Small features
- Documentation improvements

### How to Claim Bounty

1. Look for issues labeled `bounty`
2. Comment on issue to claim it
3. Submit PR with fix/feature
4. PR is reviewed and merged
5. Bounty paid via USDC on Stellar

### Bounty Rules

- One person per bounty issue
- Must be claimed before starting work
- PR must be merged to receive bounty
- Bounty amount determined by maintainers
- Payment within 7 days of merge

## Community

### Communication Channels

- **Discord**: https://discord.gg/azaka (most active)
- **GitHub Discussions**: For long-form discussions
- **GitHub Issues**: For bugs and features
- **Twitter**: @azaka_finance (announcements)

### Code of Conduct

We follow the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

**In short**:
- Be respectful and inclusive
- Welcome newcomers
- Accept constructive criticism
- Focus on what's best for the community

### Getting Help

Stuck? Ask for help:
- Discord #dev-help channel
- GitHub Discussions
- Tag maintainers in PR comments

## Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Azaka website (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Azaka! Together, we're building the future of trade finance for African SME exporters. 🚀
