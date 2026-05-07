# Azaka Quickstart Guide

Get up and running with Azaka in 10 minutes.

## Prerequisites

- Rust 1.81.0+ (`rustup install 1.81.0`)
- Stellar CLI (`cargo install --locked stellar-cli`)
- Node.js 18+ (for SDK)

## 1. Clone and Build

```bash
# Clone repository
git clone https://github.com/yourusername/azaka.git
cd azaka

# Build contracts
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test
```

## 2. Deploy to Testnet

```bash
# Generate deployer identity
stellar keys generate deployer --network testnet

# Deploy contracts
./scripts/deploy.sh
```

This will output contract IDs. Save them to `.env`:

```bash
cp .env.example .env
# Edit .env with your contract IDs
```

## 3. Seed Test Data

```bash
cd sdk/typescript
npm install
npm run seed
```

This creates:
- 2 sample trades
- 3 registered participants
- 5 document submissions

## 4. Interact with Contracts

### Create a Trade

```bash
stellar contract invoke \
  --id <TRADE_CONTRACT_ID> \
  --source issuing_bank \
  --network testnet \
  -- create_trade \
  --exporter GEXPORTER... \
  --importer GIMPORTER... \
  --issuing_bank GBANK... \
  --stablecoin_asset USDC:GUSDC... \
  --amount 50000000000 \
  --required_docs '["BillOfLading","InspectionCertificate"]' \
  --expiry_ledger 2000000
```

### Submit a Document

```bash
stellar contract invoke \
  --id <DOCUMENT_CONTRACT_ID> \
  --source freight_forwarder \
  --network testnet \
  -- submit_document \
  --trade_id 1 \
  --doc_type BillOfLading \
  --doc_hash 5a2e8f9c3b1d4a6e7f8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0 \
  --metadata_uri ipfs://QmTest123
```

### Check Trade Status

```bash
stellar contract invoke \
  --id <TRADE_CONTRACT_ID> \
  --network testnet \
  -- get_trade \
  --trade_id 1
```

## 5. Use TypeScript SDK

```typescript
import { AzakaClient, DocumentType } from '@azaka/sdk';
import { Keypair } from '@stellar/stellar-sdk';

const client = new AzakaClient({
  network: 'testnet',
  contractIds: {
    trade: process.env.TRADE_CONTRACT_ID!,
    escrow: process.env.ESCROW_CONTRACT_ID!,
    document: process.env.DOCUMENT_CONTRACT_ID!,
    registry: process.env.REGISTRY_CONTRACT_ID!,
  },
});

const keypair = Keypair.fromSecret('S...');

// Create trade
const tradeId = await client.createTrade(
  {
    exporter: 'G...',
    importer: 'G...',
    issuingBank: keypair.publicKey(),
    stablecoinAsset: 'USDC:G...',
    amount: 50000n * 10000000n,
    requiredDocs: [DocumentType.BillOfLading],
    expiryLedger: 2000000,
  },
  keypair
);

console.log(`Trade created: ${tradeId}`);
```

## Next Steps

- Read [Architecture](docs/architecture.md) to understand the system
- Read [Bank Integration Guide](docs/bank-integration.md) for bank integration
- Read [Exporter Guide](docs/exporter-guide.md) for end-user documentation
- Join [Discord](https://discord.gg/azaka) for support

## Troubleshooting

### Build fails with "wasm32-unknown-unknown not found"

```bash
rustup target add wasm32-unknown-unknown
```

### Stellar CLI not found

```bash
cargo install --locked stellar-cli
```

### Tests fail with "contract not found"

Make sure you've built the contracts first:

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Deployment fails with "insufficient balance"

Fund your testnet account:

```bash
stellar account fund deployer --network testnet
```

## Support

- **Discord**: https://discord.gg/azaka
- **GitHub Issues**: https://github.com/azaka/azaka/issues
- **Email**: support@azaka.finance
