# Bank Integration Guide

## Overview

Traditional banks can integrate with Azaka to offer blockchain-based Letters of Credit to their customers without needing deep blockchain expertise. This guide covers integration patterns, API usage, regulatory considerations, and example code.

## Why Banks Should Integrate

### Benefits for Banks

1. **New revenue stream**: Offer trade finance to previously underserved SME exporters
2. **Cost reduction**: Eliminate correspondent banking fees (8–15% savings)
3. **Faster settlement**: 5-second finality vs. 3–5 days
4. **Reduced fraud**: Cryptographic document verification
5. **Competitive advantage**: First-mover advantage in blockchain trade finance

### Benefits for Bank Customers

1. **Access to trade finance**: SMEs often excluded from traditional LC services
2. **Lower fees**: ~$0.00001 per transaction vs. $500–1000
3. **Faster payment**: Hours instead of 60–90 days
4. **Transparency**: Real-time visibility into LC status
5. **Global reach**: Trade with any country without correspondent banks

## Integration Patterns

### Pattern 1: Custodial Wallet (Recommended)

**Best for**: Banks new to blockchain, regulatory-cautious institutions

**Architecture**:
```
Bank's Core Banking System
         ↓
    REST API
         ↓
Custodian (Fireblocks, Anchorage, etc.)
         ↓
   Stellar Network
         ↓
  Azaka Contracts
```

**Pros**:
- No private key management
- Regulatory compliance built-in
- Insurance coverage
- Fast integration (weeks, not months)

**Cons**:
- Custodian fees (~0.1% per transaction)
- Dependency on third party
- Less control

**Recommended custodians**:
- **Fireblocks**: Enterprise-grade, supports Stellar
- **Anchorage Digital**: US-regulated, bank-friendly
- **Copper**: European focus, strong compliance

### Pattern 2: Direct Integration

**Best for**: Tech-savvy banks, crypto-native institutions

**Architecture**:
```
Bank's Core Banking System
         ↓
  Azaka TypeScript SDK
         ↓
HSM (Hardware Security Module)
         ↓
   Stellar Network
         ↓
  Azaka Contracts
```

**Pros**:
- Full control
- No custodian fees
- Direct access to Stellar

**Cons**:
- Complex key management
- Higher security burden
- Longer integration time (months)

**Requirements**:
- HSM for key storage (Thales, Utimaco, etc.)
- Stellar SDK integration
- DevOps expertise

### Pattern 3: Hybrid (API + Custodian)

**Best for**: Banks wanting fast integration with future flexibility

**Architecture**:
```
Bank's Core Banking System
         ↓
  Azaka REST API Service
         ↓
      Custodian
         ↓
   Stellar Network
         ↓
  Azaka Contracts
```

**Pros**:
- Fastest integration (days)
- Azaka handles Stellar complexity
- Can migrate to direct integration later

**Cons**:
- Dependency on Azaka API service
- Less customization

**Azaka API** (coming soon): Hosted service for banks to interact with Azaka without running Stellar nodes.

## TypeScript SDK Integration

### Installation

```bash
npm install @azaka/sdk @stellar/stellar-sdk
```

### Basic Setup

```typescript
import { BankClient } from '@azaka/sdk';
import { Keypair } from '@stellar/stellar-sdk';

// Load bank's keypair (from HSM or custodian)
const bankKeypair = Keypair.fromSecret('S...');

// Initialize client
const client = new BankClient(
  {
    network: 'testnet', // or 'mainnet'
    contractIds: {
      trade: 'C...',
      escrow: 'C...',
      document: 'C...',
      registry: 'C...',
    },
  },
  bankKeypair
);
```

### Issue a Letter of Credit

```typescript
import { DocumentType } from '@azaka/sdk';

async function issueLc() {
  const tradeId = await client.issueLc({
    exporter: 'GEXPORTER...', // Exporter's Stellar address
    importer: 'GIMPORTER...', // Importer's Stellar address
    issuingBank: bankKeypair.publicKey(),
    stablecoinAsset: 'USDC:GUSDC...', // USDC contract address
    amount: 50000n * 10000000n, // 50,000 USDC (7 decimals)
    requiredDocs: [
      DocumentType.BillOfLading,
      DocumentType.CertificateOfOrigin,
      DocumentType.InspectionCertificate,
    ],
    expiryLedger: 2000000, // ~30 days from now
  });

  console.log(`LC issued with trade ID: ${tradeId}`);
  return tradeId;
}
```

### Confirm a Letter of Credit

```typescript
async function confirmLc(tradeId: bigint) {
  await client.confirmLc(tradeId);
  console.log(`LC ${tradeId} confirmed`);
}
```

### Monitor Trade Status

```typescript
async function monitorTrade(tradeId: bigint) {
  const trade = await client.getTradeStatus(tradeId);
  
  console.log(`Trade ${tradeId} status: ${trade.status}`);
  console.log(`Exporter: ${trade.exporter}`);
  console.log(`Importer: ${trade.importer}`);
  console.log(`Amount: ${trade.amount / 10000000n} USDC`);
  
  const docProgress = await client.getDocumentProgress(tradeId);
  console.log(`Documents: ${docProgress.verified}/${docProgress.required} verified`);
}
```

### Query Trade Book

```typescript
async function getTradeBook() {
  const trades = await client.queryTradeBook();
  
  console.log(`Total trades: ${trades.length}`);
  
  for (const trade of trades) {
    console.log(`Trade ${trade.tradeId}: ${trade.status}`);
  }
}
```

## Core Banking System Integration

### Integration Points

Banks need to integrate Azaka at four points in their core banking system:

1. **LC issuance**: When customer requests LC
2. **LC confirmation**: When acting as confirming bank
3. **Status monitoring**: Real-time LC status updates
4. **Settlement notification**: When payment is released

### Example: LC Issuance Flow

```
Customer → Bank Portal: Request LC
         ↓
Bank Portal → Core Banking: Create LC record
         ↓
Core Banking → Azaka SDK: Issue LC on-chain
         ↓
Azaka SDK → Stellar: Submit transaction
         ↓
Stellar → Azaka Contracts: Execute create_trade()
         ↓
Azaka Contracts → Event: TradeCreated
         ↓
Bank Indexer → Core Banking: Update LC status
         ↓
Core Banking → Customer: LC issued notification
```

### Webhook Integration

Azaka can send webhooks for trade events:

```typescript
// Bank's webhook endpoint
app.post('/azaka/webhook', async (req, res) => {
  const { event, tradeId, data } = req.body;
  
  switch (event) {
    case 'TradeCreated':
      await updateLcStatus(tradeId, 'PENDING_ESCROW');
      break;
    case 'EscrowDeposited':
      await updateLcStatus(tradeId, 'ACTIVE');
      break;
    case 'DocumentSubmitted':
      await notifyCustomer(tradeId, `Document ${data.docType} submitted`);
      break;
    case 'TradeSettled':
      await updateLcStatus(tradeId, 'SETTLED');
      await notifyCustomer(tradeId, 'Payment released to exporter');
      break;
  }
  
  res.status(200).send('OK');
});
```

## Stablecoin Integration

### Supported Stablecoins

Azaka supports any Stellar asset, but recommends:

1. **USDC** (Circle): Most liquid, global acceptance
2. **EURC** (Circle): For European trades
3. **NGNC** (Naira stablecoin): For Nigerian trades
4. **KSHS** (Kenyan Shilling): For Kenyan trades (future)

### Stablecoin On/Off-Ramps

Banks need to provide fiat ↔ stablecoin conversion:

#### Option 1: Stellar Anchor
```
Customer's Bank Account
         ↓
  Stellar Anchor (e.g., Flutterwave)
         ↓
    USDC on Stellar
         ↓
   Azaka Contracts
```

**Recommended anchors**:
- **Flutterwave**: Nigeria, Kenya, Ghana
- **Chipper Cash**: Pan-African
- **Moneygram**: Global

#### Option 2: Direct Integration
```
Customer's Bank Account
         ↓
   Bank's Treasury
         ↓
  Circle API (mint USDC)
         ↓
    USDC on Stellar
         ↓
   Azaka Contracts
```

**Requirements**:
- Circle Business Account
- Treasury management system
- Liquidity reserves

### Example: Deposit Escrow with Stablecoin

```typescript
import { token } from '@stellar/stellar-sdk';

async function depositEscrow(tradeId: bigint, amount: bigint) {
  // 1. Convert customer's fiat to USDC (via anchor or Circle)
  const usdcAmount = await convertToUsdc(amount);
  
  // 2. Deposit USDC into Azaka escrow
  await client.depositEscrow(tradeId, usdcAmount);
  
  console.log(`Deposited ${usdcAmount / 10000000n} USDC for trade ${tradeId}`);
}
```

## Regulatory Considerations

### Know Your Customer (KYC)

Banks must KYC all participants:
- Exporters
- Importers
- Freight forwarders
- Inspectors

**Azaka's role**: Azaka does NOT perform KYC. Banks are responsible for KYC compliance.

**Recommendation**: Use existing KYC processes, store KYC data off-chain.

### Anti-Money Laundering (AML)

Banks must monitor for suspicious activity:
- Unusual trade patterns
- High-risk jurisdictions
- Sanctioned entities

**Azaka's role**: Azaka provides transaction transparency, but banks must run AML checks.

**Recommendation**: Integrate with AML screening tools (Chainalysis, Elliptic, etc.).

### Sanctions Screening

Banks must screen all participants against sanctions lists:
- OFAC (US)
- EU sanctions
- UN sanctions

**Azaka's role**: Azaka does NOT screen for sanctions. Banks are responsible.

**Recommendation**: Screen Stellar addresses before issuing LCs.

### Capital Requirements

Banks must hold capital against trade finance exposure:
- **Basel III**: 20–50% risk weight for trade finance
- **Local regulations**: Vary by jurisdiction

**Azaka's role**: Azaka does NOT affect capital requirements. Banks must comply with local rules.

### Cross-Border Regulations

Trade finance is subject to:
- Export controls
- Import restrictions
- Currency controls

**Azaka's role**: Azaka is a neutral protocol. Banks must ensure compliance with local laws.

## Security Best Practices

### Key Management

**For custodial integration**:
- Use reputable custodian (Fireblocks, Anchorage)
- Enable multi-sig for high-value transactions
- Regular security audits

**For direct integration**:
- Store keys in HSM (Hardware Security Module)
- Never store keys in software
- Implement key rotation policy
- Use multi-sig for admin operations

### Transaction Signing

**Best practices**:
- Review transaction details before signing
- Implement approval workflows for large amounts
- Log all transactions for audit
- Monitor for unusual activity

### API Security

**Best practices**:
- Use HTTPS for all API calls
- Implement rate limiting
- Validate all inputs
- Use API keys with limited scope

## Testing

### Testnet Setup

```bash
# 1. Install Stellar CLI
cargo install --locked stellar-cli

# 2. Generate testnet identity
stellar keys generate bank-test --network testnet

# 3. Fund testnet account
stellar account fund bank-test --network testnet

# 4. Get testnet USDC
# Visit https://stellar.org/laboratory to create trustline and receive USDC
```

### Test Scenarios

1. **Happy path**: Create trade → deposit escrow → submit docs → settle
2. **Cancellation**: Create trade → cancel before escrow
3. **Expiry**: Create trade → wait for expiry → refund
4. **Partial docs**: Submit some docs → verify settlement blocked
5. **Unauthorized submission**: Try to submit doc without authorization

### Example Test

```typescript
import { describe, it, expect } from 'vitest';

describe('Bank Integration', () => {
  it('should issue and settle LC', async () => {
    // Issue LC
    const tradeId = await client.issueLc({
      exporter: exporterAddress,
      importer: importerAddress,
      issuingBank: bankKeypair.publicKey(),
      stablecoinAsset: usdcAddress,
      amount: 10000n * 10000000n,
      requiredDocs: [DocumentType.BillOfLading],
      expiryLedger: 2000000,
    });
    
    expect(tradeId).toBeGreaterThan(0n);
    
    // Confirm LC
    await client.confirmLc(tradeId);
    
    // Check status
    const trade = await client.getTradeStatus(tradeId);
    expect(trade.status).toBe('Active');
  });
});
```

## Production Deployment

### Pre-Launch Checklist

- [ ] Security audit completed
- [ ] Custodian integration tested
- [ ] KYC/AML processes integrated
- [ ] Regulatory approval obtained
- [ ] Staff training completed
- [ ] Customer documentation prepared
- [ ] Incident response plan ready
- [ ] Monitoring and alerting configured

### Gradual Rollout

**Phase 1: Pilot (Month 1-3)**
- 5–10 trusted customers
- Small trade values (<$10,000)
- Manual review of all trades
- Daily monitoring

**Phase 2: Limited Release (Month 4-6)**
- 50–100 customers
- Medium trade values (<$100,000)
- Automated monitoring
- Weekly reviews

**Phase 3: General Availability (Month 7+)**
- All customers
- Full trade values
- Fully automated
- Monthly audits

### Monitoring

**Key metrics**:
- Trade volume (daily, weekly, monthly)
- Settlement time (average, median, p95)
- Document verification time
- Failed transactions
- Customer satisfaction

**Alerting**:
- Failed transaction rate >1%
- Settlement time >24 hours
- Unusual trade patterns
- Security incidents

## Support

### Azaka Support Channels

- **Documentation**: https://docs.azaka.finance
- **Discord**: https://discord.gg/azaka
- **Email**: support@azaka.finance
- **GitHub**: https://github.com/azaka/azaka

### Bank-Specific Support

For banks integrating Azaka, we offer:
- Dedicated integration engineer
- Priority support (24/7)
- Custom SLAs
- Regulatory guidance

Contact: banks@azaka.finance

## Example: Full Integration

```typescript
import { BankClient, DocumentType } from '@azaka/sdk';
import { Keypair } from '@stellar/stellar-sdk';

class AzakaBankingService {
  private client: BankClient;
  
  constructor(bankKeypair: Keypair) {
    this.client = new BankClient(
      {
        network: 'mainnet',
        contractIds: {
          trade: process.env.TRADE_CONTRACT_ID!,
          escrow: process.env.ESCROW_CONTRACT_ID!,
          document: process.env.DOCUMENT_CONTRACT_ID!,
          registry: process.env.REGISTRY_CONTRACT_ID!,
        },
      },
      bankKeypair
    );
  }
  
  async issueLcForCustomer(
    customerId: string,
    exporterAddress: string,
    importerAddress: string,
    amount: bigint,
    requiredDocs: DocumentType[]
  ) {
    // 1. Verify customer KYC
    await this.verifyKyc(customerId);
    
    // 2. Check credit limit
    await this.checkCreditLimit(customerId, amount);
    
    // 3. Issue LC on-chain
    const tradeId = await this.client.issueLc({
      exporter: exporterAddress,
      importer: importerAddress,
      issuingBank: this.client.bankKeypair.publicKey(),
      stablecoinAsset: process.env.USDC_ADDRESS!,
      amount,
      requiredDocs,
      expiryLedger: this.calculateExpiryLedger(30), // 30 days
    });
    
    // 4. Record in core banking system
    await this.recordLcInCoreSystem(customerId, tradeId, amount);
    
    // 5. Notify customer
    await this.notifyCustomer(customerId, `LC issued: ${tradeId}`);
    
    return tradeId;
  }
  
  async monitorTrades() {
    const trades = await this.client.queryTradeBook();
    
    for (const trade of trades) {
      // Update core banking system
      await this.updateLcStatus(trade.tradeId, trade.status);
      
      // Check for settlement
      if (trade.status === 'Settled') {
        await this.processSettlement(trade);
      }
    }
  }
  
  private async verifyKyc(customerId: string) {
    // Implement KYC check
  }
  
  private async checkCreditLimit(customerId: string, amount: bigint) {
    // Implement credit limit check
  }
  
  private async recordLcInCoreSystem(customerId: string, tradeId: bigint, amount: bigint) {
    // Implement core banking integration
  }
  
  private async notifyCustomer(customerId: string, message: string) {
    // Implement customer notification
  }
  
  private async updateLcStatus(tradeId: bigint, status: string) {
    // Implement status update
  }
  
  private async processSettlement(trade: any) {
    // Implement settlement processing
  }
  
  private calculateExpiryLedger(days: number): number {
    // Stellar: ~5 second blocks, ~17,280 blocks per day
    return Date.now() + (days * 17280);
  }
}

// Usage
const bankKeypair = Keypair.fromSecret(process.env.BANK_SECRET_KEY!);
const service = new AzakaBankingService(bankKeypair);

// Issue LC
const tradeId = await service.issueLcForCustomer(
  'CUST123',
  'GEXPORTER...',
  'GIMPORTER...',
  50000n * 10000000n,
  [DocumentType.BillOfLading, DocumentType.InspectionCertificate]
);

// Monitor trades (run periodically)
setInterval(() => service.monitorTrades(), 60000); // Every minute
```

## Conclusion

Integrating Azaka enables banks to:
- Serve previously underserved SME exporters
- Reduce costs by 90%+
- Settle trades in hours instead of months
- Compete in the blockchain trade finance market

The TypeScript SDK and custodial integration pattern make it possible to integrate in weeks, not months, with minimal blockchain expertise required.

For integration support, contact: banks@azaka.finance
