# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: security@azaka.finance

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

## Bug Bounty Program

Azaka offers bounties for security vulnerabilities:

### Severity Levels

**Critical** ($5,000 - $10,000)
- Loss of funds from escrow
- Unauthorized document verification
- Contract takeover

**High** ($1,000 - $5,000)
- Denial of service
- Unauthorized state changes
- Access control bypass

**Medium** ($500 - $1,000)
- Information disclosure
- Logic errors without fund loss

**Low** ($100 - $500)
- Best practice violations
- Minor issues

### Scope

**In scope:**
- Smart contracts in `contracts/`
- TypeScript SDK in `sdk/typescript/`
- Deployment scripts

**Out of scope:**
- Third-party dependencies
- Social engineering
- Physical attacks
- Denial of service attacks

### Rules

- You must be the first reporter of the vulnerability
- The vulnerability must be a valid security issue
- You must not exploit the vulnerability beyond proof-of-concept
- You must not disclose the vulnerability publicly before it is fixed
- You must not access or modify user data

## Security Best Practices

### For Users

- Never share your secret key
- Use hardware wallets for large amounts
- Verify contract addresses before interacting
- Start with small test transactions

### For Developers

- All contracts must be audited before mainnet deployment
- Use `cargo clippy` and `cargo audit` regularly
- Follow Rust security best practices
- No `unwrap()` in production code
- All storage operations must be checked

### For Banks

- Use HSM or custodian for key management
- Implement multi-sig for high-value operations
- Monitor all transactions
- Have incident response plan ready

## Audit History

| Date | Auditor | Report |
|------|---------|--------|
| TBD  | TBD     | TBD    |

## Security Updates

Security updates will be announced via:
- GitHub Security Advisories
- Discord #security channel
- Email to registered banks
- Twitter @azaka_finance

## Contact

- Email: security@azaka.finance
- PGP Key: [Coming soon]
- Discord: https://discord.gg/azaka (DM @security-team)
