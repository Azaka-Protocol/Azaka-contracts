# Azaka Architecture

## Overview

Azaka is a decentralized trade finance protocol built on Stellar's Soroban smart contract platform. It digitizes the Letter of Credit (LC) process for African SME exporters, reducing settlement times from 90 days to hours and eliminating costly correspondent banking fees.

## Why Stellar/Soroban?

### 1. Low Transaction Fees
Traditional LC amendments cost $50–500 per change. Stellar transactions cost ~$0.00001, making micro-adjustments economically viable.

### 2. Stablecoin-Native Platform
Stellar has native support for stablecoins like USDC, EURC, and African stablecoins (NGNC). This eliminates the need for custom token implementations and provides instant liquidity.

### 3. African Fintech Ecosystem
Stellar has deep penetration in African markets through:
- **Stellar anchors**: Provide local currency on/off-ramps in Nigeria, Kenya, South Africa, Ghana
- **Mobile money integration**: Direct integration with M-Pesa, MTN Mobile Money
- **Regulatory alignment**: Stellar's compliance framework aligns with African central bank requirements

### 4. Fast Finality
5-second block times enable near-instant settlement compared to 3–5 days for correspondent banking.

### 5. Built-in Compliance
Stellar's regulated assets and KYC/AML hooks make it easier to comply with trade finance regulations.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Azaka Protocol                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │   Registry   │  │   Document   │  │    Trade     │         │
│  │   Contract   │  │   Contract   │  │   Contract   │         │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘         │
│         │                  │                  │                 │
│         │ Authorize        │ Verify Docs      │ Trigger         │
│         │                  │                  │                 │
│         └──────────────────┴──────────────────┘                 │
│                            │                                    │
│                    ┌───────▼────────┐                          │
│                    │     Escrow     │                          │
│                    │    Contract    │                          │
│                    └────────────────┘                          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                             │
                             │ Stellar Network
                             │
        ┌────────────────────┼────────────────────┐
        │                    │                    │
   ┌────▼─────┐        ┌────▼─────┐        ┌────▼─────┐
   │ Exporter │        │ Importer │        │  Banks   │
   └──────────┘        └──────────┘        └──────────┘
        │                    │                    │
        │                    │                    │
   ┌────▼─────┐        ┌────▼─────┐        ┌────▼─────┐
   │ Freight  │        │Inspector │        │   Port   │
   │Forwarder │        │          │        │Authority │
   └──────────┘        └──────────┘        └──────────┘
```

## Contract Interaction Flow

### 1. Trade Creation
```
Issuing Bank → Trade Contract: create_trade()
  ↓
Trade Contract: Stores trade terms, assigns ID
  ↓
Event: TradeCreated(trade_id, exporter, importer, amount)
```

### 2. Trade Confirmation
```
Confirming Bank → Trade Contract: confirm_trade(trade_id)
  ↓
Trade Contract: Updates status to Active
  ↓
Event: TradeConfirmed(trade_id, confirming_bank)
```

### 3. Escrow Deposit
```
Importer → Escrow Contract: deposit(trade_id, amount)
  ↓
Escrow Contract: Locks stablecoin
  ↓
Escrow Contract → Trade Contract: mark_escrow_deposited(trade_id)
  ↓
Trade Contract: Updates status to DocumentsPending
  ↓
Event: EscrowDeposited(trade_id, amount)
```

### 4. Document Submission & Verification
```
Freight Forwarder → Document Contract: submit_document(trade_id, doc_type, hash, uri)
  ↓
Document Contract ← Registry Contract: is_authorised(forwarder, FreightForwarder)
  ↓
Document Contract: Stores document hash and metadata
  ↓
Event: DocumentSubmitted(trade_id, doc_type, hash)

Port Authority → Document Contract: sign_document(trade_id, doc_type, signer)
  ↓
Document Contract: Adds signature, checks if verified
  ↓
Event: DocumentSigned(trade_id, doc_type, signer)
  ↓
If all signatures collected:
  Event: DocumentVerified(trade_id, doc_type)
```

### 5. Settlement
```
Anyone → Trade Contract: settle_trade(trade_id)
  ↓
Trade Contract → Document Contract: all_docs_verified(trade_id, required_docs)
  ↓
Document Contract: Returns true if all docs verified
  ↓
Trade Contract: Updates status to Settled
  ↓
Trade Contract → Escrow Contract: release(trade_id, exporter)
  ↓
Escrow Contract: Transfers stablecoin to exporter
  ↓
Event: TradeSettled(trade_id, exporter)
```

## Document Verification Flow

The document verification system is the most novel component of Azaka. It implements a multi-signature pattern scoped to specific document types per trade.

### Document Lifecycle

1. **Submission**: Authorized participant submits SHA-256 hash of document + IPFS URI
2. **Counter-signing**: Second authorized party verifies and signs the document hash
3. **Verification**: Once required signatures collected, document marked as verified
4. **Settlement trigger**: When all required documents verified, escrow releases

### Why Hash-Based Verification?

- **Privacy**: Actual documents stored off-chain (IPFS), only hashes on-chain
- **Immutability**: Hash anchoring prevents document tampering
- **Efficiency**: Small on-chain footprint (32 bytes per document)
- **Compliance**: Original documents available for audit via IPFS

### Multi-Signature Pattern

Each document type requires multiple signatures from different participant types:

| Document Type | Primary Submitter | Required Counter-Signer |
|--------------|-------------------|------------------------|
| Bill of Lading | Freight Forwarder | Port Authority |
| Certificate of Origin | Chamber of Commerce | Customs Authority |
| Inspection Certificate | Independent Inspector | Exporter |
| Phytosanitary Certificate | Phyto Authority | Ministry of Agriculture |
| Customs Declaration | Customs Broker | Customs Authority |

This creates a web of trust where no single party can unilaterally verify documents.

## Escrow Security Model

### Access Control

The escrow contract enforces strict access control:

- **deposit()**: Callable by anyone (typically importer)
- **release()**: Only callable by trade contract after document verification
- **refund()**: Only callable by trade contract on cancellation/expiry

### Why Only Trade Contract Can Release?

This prevents:
- Exporter from releasing funds prematurely
- Importer from withdrawing funds after deposit
- Third parties from draining escrow

The trade contract acts as the **sole arbiter** of when funds should move, based on:
- Document verification status
- Trade expiry
- Cancellation conditions

### Double-Release Prevention

Once funds are released, the balance entry is deleted. Subsequent release attempts fail with `NoBalance` error.

### Refund Conditions

Funds are refunded to importer if:
1. Trade is cancelled before settlement
2. Trade expires without all documents verified
3. Both parties agree to cancel (future feature)

## Bank Integration Model

Traditional banks can participate in Azaka without holding cryptocurrency directly.

### Integration Patterns

#### 1. Custodial Wallet (Recommended for Banks)
```
Bank → Custodian (e.g., Fireblocks) → Stellar Network
```
- Bank never holds private keys
- Custodian handles signing and transaction submission
- Bank integrates via REST API

#### 2. Direct Integration
```
Bank → HSM (Hardware Security Module) → Stellar Network
```
- Bank manages keys in secure hardware
- Direct Stellar SDK integration
- Full control, higher complexity

#### 3. Hybrid (API + Custodian)
```
Bank → Azaka API Service → Custodian → Stellar Network
```
- Bank calls Azaka's REST API
- API service handles Stellar complexity
- Lowest integration effort

### Bank Operations

Banks perform three main operations:

1. **Issue LC**: Create trade on behalf of importer
2. **Confirm LC**: Co-sign trade terms as confirming bank
3. **Monitor**: Query trade status and document progress

All operations available via TypeScript SDK's `BankClient` class.

## Participant Registry

The registry contract maintains a whitelist of authorized participants.

### Registration Process

1. Admin (Azaka governance) registers participant
2. Participant type assigned (Bank, Forwarder, Inspector, etc.)
3. Participant can now submit/sign documents

### Revocation

Admin can revoke participants for:
- Fraudulent document submission
- Repeated verification failures
- Regulatory violations

Revoked participants cannot submit new documents, but existing trades are unaffected.

## Event System

All contracts emit events for off-chain indexing and monitoring:

### Trade Events
- `TradeCreated(trade_id, exporter, importer, amount)`
- `TradeConfirmed(trade_id, confirming_bank)`
- `EscrowDeposited(trade_id, amount)`
- `TradeSettled(trade_id, exporter)`
- `TradeCancelled(trade_id, caller)`
- `TradeExpired(trade_id)`

### Document Events
- `DocumentSubmitted(trade_id, doc_type, hash)`
- `DocumentSigned(trade_id, doc_type, signer)`
- `DocumentVerified(trade_id, doc_type)`

### Escrow Events
- `Deposit(trade_id, from, amount)`
- `Release(trade_id, to, amount)`
- `Refund(trade_id, to, amount)`

### Registry Events
- `ParticipantRegistered(address, type, name)`
- `ParticipantRevoked(address)`

## Scalability Considerations

### Current Throughput
- Stellar: ~1000 TPS
- Soroban: ~100 contract invocations/second

### Bottlenecks
- Document verification requires multiple transactions
- Each signature is a separate transaction

### Future Optimizations
1. **Batch document submission**: Submit multiple docs in one transaction
2. **Aggregated signatures**: Collect signatures off-chain, submit proof on-chain
3. **Layer 2 for high-frequency operations**: Move document submission to L2, settle on L1

## Security Considerations

### Smart Contract Risks
- **Reentrancy**: Not applicable (Soroban doesn't support reentrancy)
- **Integer overflow**: Rust's checked arithmetic prevents this
- **Access control**: All privileged functions require authentication

### Operational Risks
- **Key management**: Banks must secure private keys
- **Oracle risk**: Document verification relies on trusted participants
- **Regulatory risk**: Cross-border trade finance is heavily regulated

### Mitigation Strategies
1. **Formal verification**: Audit contracts with formal methods
2. **Bug bounty**: Incentivize security researchers
3. **Gradual rollout**: Start with small trade values
4. **Insurance**: Explore trade finance insurance products

## Future Enhancements

### Phase 2: Advanced Features
- **Partial payments**: Release funds in tranches as documents arrive
- **Dispute resolution**: On-chain arbitration for contested trades
- **Dynamic pricing**: Adjust fees based on trade risk
- **LC amendments**: TODO - Implement amendment functionality
- **Multi-currency support**: TODO - Support multiple stablecoins per trade

### Phase 3: Ecosystem Expansion
- **Invoice financing**: Use verified LCs as collateral for loans
- **Trade insurance**: Integrate with parametric insurance protocols
- **Cross-chain bridges**: Support trades settled in other stablecoins
- **Supply chain tracking**: TODO - Integrate with IoT devices for real-time cargo tracking
- **Carbon credits**: TODO - Track and trade carbon offsets for shipments

### Phase 4: Governance
- **DAO formation**: Decentralize protocol governance
- **Participant voting**: Let ecosystem vote on registry additions
- **Fee distribution**: Share protocol revenue with stakeholders
- **Protocol upgrades**: TODO - Implement upgrade mechanism for contracts
- **Emergency pause**: TODO - Add circuit breaker for security incidents

## Known Limitations

### Current Version (v0.1.0)

1. **No cross-contract calls**: Trade contract doesn't automatically verify documents
   - **Workaround**: Manual verification required before settlement
   - **Fix planned**: Phase 2

2. **No LC amendments**: Once created, LC terms cannot be changed
   - **Workaround**: Cancel and recreate trade
   - **Fix planned**: Phase 2

3. **Single stablecoin per trade**: Cannot mix USDC and EURC
   - **Workaround**: Use single currency
   - **Fix planned**: Phase 2

4. **No partial releases**: All-or-nothing payment
   - **Workaround**: Create multiple smaller trades
   - **Fix planned**: Phase 2

5. **Limited document types**: Only 5 document types supported
   - **Workaround**: Use CustomsDeclaration for other docs
   - **Fix planned**: Phase 3

## Conclusion

Azaka's architecture prioritizes:
1. **Security**: Multi-signature verification, strict access control
2. **Simplicity**: Four focused contracts, clear separation of concerns
3. **Interoperability**: Standard Stellar assets, open APIs
4. **African context**: Low fees, mobile money integration, local anchors

By building on Stellar/Soroban, Azaka leverages a mature, compliant, and Africa-friendly blockchain platform to bring trade finance into the 21st century.
