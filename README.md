# Azaka — Decentralised Trade Finance for African SME Exporters


[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue.svg)](https://stellar.org/soroban)
[![Rust](https://img.shields.io/badge/rust-1.95.0-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/status-alpha-yellow.svg)](https://github.com/yourusername/azaka)

Azaka digitises the Letter of Credit process for small exporters. An importer locks payment into a smart contract escrow at deal creation. When authorised third parties (freight forwarders, inspection companies, port authorities) submit and sign the required shipping documents on-chain, the escrow releases payment automatically to the exporter in stablecoins — cutting settlement from 90 days to hours and eliminating correspondent bank fees.

## The Problem

African SME exporters face systemic barriers in international trade:

- **90-day payment cycles**: Traditional Letters of Credit take 60–90 days to settle
- **8–15% transaction fees**: Correspondent banking fees eat into already thin margins
- **Exclusion from banking**: Many African SMEs cannot access trade finance from international banks
- **Document fraud risk**: Paper-based document verification is slow and vulnerable to tampering

Azaka solves this by moving the entire LC lifecycle on-chain, with cryptographic document verification and instant stablecoin settlement.

## Architecture

```
azaka/
├── contracts/                  # Soroban smart contracts
│   ├── trade/                 # Trade contract (LC lifecycle)
│   │   └── src/lib.rs
│   ├── escrow/                # Escrow contract (stablecoin custody)
│   │   └── src/lib.rs
│   ├── document/              # Document contract (hash verification)
│   │   └── src/lib.rs
│   └── registry/              # Registry contract (participant management)
│       └── src/lib.rs
│
├── sdk/                       # Client SDKs
│   └── typescript/            # TypeScript SDK
│       ├── src/
│       │   ├── client.ts      # Main Azaka client
│       │   ├── bank-client.ts # Bank-specific client
│       │   ├── types.ts       # Type definitions
│       │   └── index.ts
│       └── package.json
│
├── docs/                      # Documentation
│   ├── architecture.md        # System architecture
│   ├── bank-integration.md    # Bank integration guide
│   ├── document-verification.md
│   └── exporter-guide.md
│
├── scripts/                   # Deployment & utility scripts
│   ├── deploy.sh             # Contract deployment (partial)
│   └── seed.ts               # Testnet seed data (partial)
│
├── tests/                     # Integration tests
│   └── integration_test.rs
│
├── Cargo.toml                # Rust workspace config
├── README.md
├── ROADMAP.md                # Development roadmap
└── TODO.md                   # Task list
```

## Quickstart

### Prerequisites

- Rust 1.95.0+ with `wasm32-unknown-unknown` target
- Stellar CLI (`cargo install --locked stellar-cli`)
- Node.js 18+ (for SDK and seed script)

### Build

```bash
git clone https://github.com/yourusername/Azaka-contracts.git
cd azaka
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests

```bash
cargo test
```

**Note**: Some integration tests may fail due to incomplete cross-contract call implementations. This is expected in the current alpha version.

### Deploy to Stellar Testnet

```bash
# Set up Testnet identity
stellar keys generate deployer --network testnet

# Deploy contracts (partial implementation)
./scripts/deploy.sh

# Seed test data (partial implementation)
cd sdk/typescript
npm install
npm run seed
```

## Trade Lifecycle

### 1. Create Trade

```bash
stellar contract invoke \
  --id <TRADE_CONTRACT_ID> \
  --source exporter \
  --network testnet \
  -- create_trade \
  --exporter GEXPORTER... \
  --importer GIMPORTER... \
  --issuing_bank GBANK... \
  --stablecoin_asset USDC:GUSDC... \
  --amount 50000000000 \
  --required_docs '["BillOfLading","CertificateOfOrigin","InspectionCertificate"]' \
  --expiry_ledger 1234567
```

### 2. Confirm Trade (Confirming Bank)

```bash
stellar contract invoke \
  --id <TRADE_CONTRACT_ID> \
  --source confirming_bank \
  --network testnet \
  -- confirm_trade \
  --trade_id 1
```

### 3. Deposit Escrow (Importer)

```bash
stellar contract invoke \
  --id <ESCROW_CONTRACT_ID> \
  --source importer \
  --network testnet \
  -- deposit \
  --trade_id 1 \
  --amount 50000000000
```

### 4. Submit Documents (Freight Forwarder)

```bash
stellar contract invoke \
  --id <DOCUMENT_CONTRACT_ID> \
  --source freight_forwarder \
  --network testnet \
  -- submit_document \
  --trade_id 1 \
  --doc_type BillOfLading \
  --doc_hash 5a2e8f9c3b1d4a6e7f8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0 \
  --metadata_uri ipfs://QmXyz123...
```

### 5. Sign Documents (Port Authority)

```bash
stellar contract invoke \
  --id <DOCUMENT_CONTRACT_ID> \
  --source port_authority \
  --network testnet \
  -- sign_document \
  --trade_id 1 \
  --doc_type BillOfLading \
  --signer GPORT...
```

### 6. Release Payment (Anyone)

```bash
stellar contract invoke \
  --id <ESCROW_CONTRACT_ID> \
  --source anyone \
  --network testnet \
  -- release \
  --trade_id 1
```

## Document Types

| Document Type | Description | Typical Submitter | Required Signer |
|--------------|-------------|-------------------|-----------------|
| **BillOfLading** | Proof of shipment and title to goods | Freight Forwarder | Port Authority |
| **CertificateOfOrigin** | Certifies country of manufacture | Chamber of Commerce | Customs Authority |
| **InspectionCertificate** | Quality/quantity verification | Independent Inspector | Exporter |
| **PhytosanitaryCertificate** | Agricultural product health cert | Phytosanitary Authority | Ministry of Agriculture |
| **CustomsDeclaration** | Export customs clearance | Customs Broker | Customs Authority |

## Participant Types

| Participant Type | Role | Permissions |
|-----------------|------|-------------|
| **IssuingBank** | Importer's bank that issues the LC | Create trades, co-sign terms |
| **ConfirmingBank** | Exporter's local bank that confirms LC | Confirm trades, guarantee payment |
| **FreightForwarder** | Logistics provider | Submit Bill of Lading |
| **Inspector** | Independent quality inspector | Submit Inspection Certificate |
| **PortAuthority** | Port/customs official | Sign and verify documents |

## Contract Addresses

### Testnet

```
Trade Contract:     <DEPLOY_AND_UPDATE>
Escrow Contract:    <DEPLOY_AND_UPDATE>
Document Contract:  <DEPLOY_AND_UPDATE>
Registry Contract:  <DEPLOY_AND_UPDATE>
```

### Mainnet

```
Coming soon
```

## Bank Integration

Traditional banks can integrate with Azaka without holding cryptocurrency. See [docs/bank-integration.md](docs/bank-integration.md) for:

- API-based integration via the TypeScript SDK
- Custodial wallet solutions for banks
- Regulatory compliance considerations
- Example integration code

## TypeScript SDK

```typescript
import { AzakaClient } from '@azaka/sdk';

const client = new AzakaClient({
  network: 'testnet',
  contractIds: {
    trade: 'C...',
    escrow: 'C...',
    document: 'C...',
    registry: 'C...'
  }
});

// Create a trade
const tradeId = await client.createTrade({
  exporter: 'GEXPORTER...',
  importer: 'GIMPORTER...',
  issuingBank: 'GBANK...',
  stablecoinAsset: 'USDC:GUSDC...',
  amount: 50000n * 10000000n, // 50,000 USDC
  requiredDocs: ['BillOfLading', 'CertificateOfOrigin'],
  expiryLedger: 1234567
});

// Submit a document
await client.submitDocument({
  tradeId,
  docType: 'BillOfLading',
  docHash: '5a2e8f9c...',
  metadataUri: 'ipfs://QmXyz...'
});

// Check trade status
const trade = await client.getTrade(tradeId);
console.log(trade.status); // "DocumentsPending"
```

## Stellar/Soroban Compatibility

- **Soroban SDK**: 21.x
- **Rust Toolchain**: 1.81.0
- **Stellar CLI**: 21.x
- **Network**: Testnet (Futurenet for early testing)

## Why Stellar?

1. **Low fees**: ~$0.00001 per transaction vs. $50–500 for traditional LC amendments
2. **Stablecoin-native**: USDC, EURC, and African stablecoins (NGNC) are first-class assets
3. **African fintech ecosystem**: Stellar anchors provide local currency on/off-ramps in Nigeria, Kenya, South Africa
4. **Fast finality**: 5-second settlement vs. 3–5 days for correspondent banking
5. **Regulatory clarity**: Stellar's compliance framework aligns with African central bank requirements

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for local setup, PR guidelines, and bounty information.


**Priority Contributions**:
- Complete SDK implementation (transaction parsing, proper contract invocation)
- Implement cross-contract calls in smart contracts
- Add deployment automation
- Improve test coverage
- Add event indexer

## Project Status

**Version**: v0.1.0 (Alpha)  
**Status**: Active Development


## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

Built with support from the Stellar Development Foundation and inspired by the needs of African SME exporters.

---

**Disclaimer**: Azaka is experimental software. Conduct thorough audits before using in production. Not financial advice.
