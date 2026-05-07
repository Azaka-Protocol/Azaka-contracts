# Azaka TypeScript SDK

TypeScript SDK for interacting with the Azaka decentralized trade finance protocol on Stellar.

## Installation

```bash
npm install @azaka/sdk @stellar/stellar-sdk
```

## Quick Start

```typescript
import { AzakaClient, DocumentType } from '@azaka/sdk';
import { Keypair } from '@stellar/stellar-sdk';

// Initialize client
const client = new AzakaClient({
  network: 'testnet',
  contractIds: {
    trade: 'C...',
    escrow: 'C...',
    document: 'C...',
    registry: 'C...',
  },
});

// Create a trade
const keypair = Keypair.fromSecret('S...');
const tradeId = await client.createTrade(
  {
    exporter: 'GEXPORTER...',
    importer: 'GIMPORTER...',
    issuingBank: 'GBANK...',
    stablecoinAsset: 'USDC:G...',
    amount: 50000n * 10000000n,
    requiredDocs: [DocumentType.BillOfLading],
    expiryLedger: 2000000,
  },
  keypair
);

console.log(`Trade created: ${tradeId}`);
```

## Bank Integration

```typescript
import { BankClient } from '@azaka/sdk';

const bankClient = new BankClient(config, bankKeypair);

// Issue LC
const tradeId = await bankClient.issueLc({
  exporter: 'G...',
  importer: 'G...',
  issuingBank: bankKeypair.publicKey(),
  stablecoinAsset: 'USDC:G...',
  amount: 50000n * 10000000n,
  requiredDocs: [DocumentType.BillOfLading],
  expiryLedger: 2000000,
});

// Monitor trade
const trade = await bankClient.getTradeStatus(tradeId);
console.log(trade.status);
```

## API Reference

See [full documentation](https://docs.azaka.finance/sdk) for complete API reference.

## License

MIT
