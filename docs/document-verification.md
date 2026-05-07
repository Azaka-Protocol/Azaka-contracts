# Document Verification in Azaka

## Overview

Document verification is the core innovation of Azaka. Traditional Letters of Credit rely on banks manually reviewing paper documents, which is slow, expensive, and fraud-prone. Azaka replaces this with cryptographic hash verification and multi-party digital signatures.

## The Problem with Traditional Document Verification

### Current Process
1. Exporter ships goods and obtains shipping documents
2. Exporter presents documents to their bank (confirming bank)
3. Confirming bank reviews documents for compliance
4. Confirming bank sends documents to issuing bank (importer's bank)
5. Issuing bank reviews documents again
6. If compliant, issuing bank releases payment
7. Issuing bank sends documents to importer

### Pain Points
- **Time**: 7–14 days for document review and courier
- **Cost**: $200–500 in document handling fees
- **Fraud**: Paper documents can be forged or altered
- **Errors**: Manual review leads to discrepancies (70% of LCs have errors)
- **Opacity**: Exporter has no visibility into review status

## Azaka's Solution: Hash-Based Verification

### Core Concept

Instead of sending physical documents, participants:
1. Upload documents to IPFS (decentralized storage)
2. Compute SHA-256 hash of the document
3. Submit hash to Azaka's document contract
4. Authorized parties counter-sign the hash
5. Once all signatures collected, document is verified

### Benefits

- **Speed**: Instant submission and verification
- **Cost**: ~$0.00001 per transaction vs. $200–500
- **Immutability**: Hash anchoring prevents tampering
- **Transparency**: All parties see verification status in real-time
- **Privacy**: Actual documents stored off-chain, only hashes on-chain

## Document Types

Azaka supports five standard trade finance documents:

### 1. Bill of Lading (B/L)
**Purpose**: Proof of shipment and title to goods

**Submitter**: Freight forwarder or shipping line

**Counter-signer**: Port authority or customs

**Why it matters**: The B/L is the most critical document in trade finance. It proves that goods were loaded onto a vessel and gives the holder the right to claim the goods at destination.

**Verification flow**:
```
Freight Forwarder → Uploads B/L to IPFS
                  → Computes hash
                  → Submits to document contract
                  
Port Authority → Reviews B/L on IPFS
               → Verifies goods match manifest
               → Signs hash in document contract
               
Document Contract → Marks B/L as verified
```

### 2. Certificate of Origin (CoO)
**Purpose**: Certifies country of manufacture

**Submitter**: Chamber of Commerce or trade association

**Counter-signer**: Customs authority

**Why it matters**: Required for tariff determination and trade agreement benefits (e.g., African Continental Free Trade Area).

### 3. Inspection Certificate
**Purpose**: Quality and quantity verification

**Submitter**: Independent inspection company (SGS, Bureau Veritas, etc.)

**Counter-signer**: Exporter or importer

**Why it matters**: Proves that goods meet the specifications in the LC.

### 4. Phytosanitary Certificate
**Purpose**: Agricultural product health certification

**Submitter**: National phytosanitary authority

**Counter-signer**: Ministry of Agriculture

**Why it matters**: Required for agricultural exports to prove products are pest-free.

### 5. Customs Declaration
**Purpose**: Export customs clearance

**Submitter**: Customs broker

**Counter-signer**: Customs authority

**Why it matters**: Proves goods legally cleared export customs.

## Multi-Signature Verification Pattern

### Why Multiple Signatures?

A single signature is vulnerable to:
- **Collusion**: Submitter could forge documents
- **Compromise**: Hacked account could submit fake documents
- **Error**: Single reviewer might miss discrepancies

Multiple signatures create a **web of trust** where:
- Different parties verify different aspects
- No single party can unilaterally approve
- Fraud requires collusion between multiple entities

### Signature Requirements

Each document type has a `required_signatures` parameter (default: 2).

```rust
pub struct Document {
    pub trade_id: u64,
    pub doc_type: DocumentType,
    pub doc_hash: BytesN<32>,
    pub metadata_uri: String,
    pub submitter: Address,
    pub signers: Vec<Address>,
    pub verified: bool,
    pub required_signatures: u32,  // Default: 2
}
```

### Signature Flow

```
Step 1: Submission
  Freight Forwarder → submit_document(trade_id, BillOfLading, hash, ipfs_uri)
  
  Document Contract:
    - Checks forwarder is authorized (via registry)
    - Creates document record
    - Adds submitter as first signer
    - Sets verified = false
    - Emits DocumentSubmitted event

Step 2: Counter-signing
  Port Authority → sign_document(trade_id, BillOfLading, port_authority_address)
  
  Document Contract:
    - Checks port authority is authorized
    - Checks not already signed by this address
    - Adds port authority to signers list
    - Checks if signers.len() >= required_signatures
    - If yes, sets verified = true
    - Emits DocumentSigned event
    - If verified, emits DocumentVerified event

Step 3: Verification Check
  Anyone → verify_document(trade_id, BillOfLading)
  
  Document Contract:
    - Returns document.verified (true/false)
```

## Hash Computation

### Client-Side Hashing

Documents are hashed client-side before submission:

```typescript
import { createHash } from 'crypto';
import { readFileSync } from 'fs';

function computeDocumentHash(filePath: string): string {
  const fileBuffer = readFileSync(filePath);
  const hash = createHash('sha256').update(fileBuffer).digest('hex');
  return hash;
}

// Example
const billOfLadingHash = computeDocumentHash('./documents/bill-of-lading.pdf');
// Output: 5a2e8f9c3b1d4a6e7f8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0
```

### Why SHA-256?

- **Collision resistance**: Computationally infeasible to find two documents with same hash
- **Deterministic**: Same document always produces same hash
- **Fast**: Can hash large documents in milliseconds
- **Standard**: Widely supported across all platforms

### Hash Storage

Hashes are stored as `BytesN<32>` in Soroban:

```rust
pub doc_hash: BytesN<32>,  // 32 bytes = 256 bits
```

This is the most efficient storage format for SHA-256 hashes.

## IPFS Integration

### Why IPFS?

- **Decentralized**: No single point of failure
- **Content-addressed**: Files identified by their hash
- **Permanent**: Files persist as long as someone pins them
- **Censorship-resistant**: No central authority can delete files

### Document Upload Flow

```
1. Exporter uploads document to IPFS
   → ipfs add bill-of-lading.pdf
   → Returns: QmXyz123... (IPFS CID)

2. Exporter computes SHA-256 hash
   → sha256sum bill-of-lading.pdf
   → Returns: 5a2e8f9c3b1d...

3. Exporter submits to Azaka
   → submit_document(
       trade_id: 1,
       doc_type: BillOfLading,
       doc_hash: 5a2e8f9c...,
       metadata_uri: "ipfs://QmXyz123..."
     )

4. Counter-signer retrieves document from IPFS
   → ipfs get QmXyz123...
   → Verifies hash matches
   → Signs in Azaka contract
```

### IPFS Pinning

To ensure documents remain available, they should be pinned by:
- Exporter
- Importer
- Both banks
- Azaka's IPFS pinning service (future feature)

### Alternative Storage

While IPFS is recommended, the `metadata_uri` field supports any URI scheme:
- `ipfs://QmXyz...` - IPFS
- `https://docs.example.com/...` - Centralized storage
- `ar://abc123...` - Arweave (permanent storage)

## Authorization and Access Control

### Participant Authorization

Only authorized participants can submit/sign documents:

```rust
// In document contract
pub fn submit_document(
    env: Env,
    trade_id: u64,
    doc_type: DocumentType,
    doc_hash: BytesN<32>,
    metadata_uri: String,
) -> Result<(), DocumentError> {
    let submitter = env.current_contract_address();
    
    // Check authorization via registry contract
    let registry: Address = env.storage().instance().get(&DataKey::RegistryContract).unwrap();
    let is_authorized = registry_contract::is_authorised(
        env.clone(),
        submitter.clone(),
        get_required_participant_type(doc_type.clone())
    );
    
    if !is_authorized {
        return Err(DocumentError::NotAuthorized);
    }
    
    // ... rest of function
}
```

### Participant Type Mapping

| Document Type | Allowed Submitters |
|--------------|-------------------|
| BillOfLading | FreightForwarder |
| CertificateOfOrigin | FreightForwarder, Inspector |
| InspectionCertificate | Inspector |
| PhytosanitaryCertificate | Inspector, PortAuthority |
| CustomsDeclaration | FreightForwarder, PortAuthority |

### Revocation

If a participant is revoked, they can no longer submit new documents, but existing signatures remain valid.

## Verification Queries

### Check Single Document

```rust
pub fn verify_document(env: Env, trade_id: u64, doc_type: DocumentType) -> bool {
    let key = DataKey::Document(trade_id, doc_type);
    
    if let Some(document) = env.storage().persistent().get::<DataKey, Document>(&key) {
        document.verified
    } else {
        false
    }
}
```

### Check All Documents

```rust
pub fn all_docs_verified(
    env: Env,
    trade_id: u64,
    required_docs: Vec<DocumentType>,
) -> bool {
    for doc_type in required_docs.iter() {
        if !Self::verify_document(env.clone(), trade_id, doc_type) {
            return false;
        }
    }
    true
}
```

This function is called by the trade contract before releasing escrow.

## Error Handling

### Common Errors

```rust
pub enum DocumentError {
    NotAuthorized = 1,        // Submitter not in registry
    DocumentNotFound = 2,     // Document doesn't exist
    DocumentAlreadyExists = 3, // Duplicate submission
    AlreadySigned = 4,        // Signer already signed this doc
    InvalidSignatureCount = 5, // Not enough signatures
}
```

### Error Scenarios

**NotAuthorized**: Freight forwarder tries to submit inspection certificate
```
Solution: Register as Inspector in registry contract
```

**DocumentAlreadyExists**: Submitting same document twice
```
Solution: Use get_document() to check if already submitted
```

**AlreadySigned**: Port authority tries to sign twice
```
Solution: Check signers list before signing
```

## Real-World Example

### Kenyan Coffee Export to USA

**Trade Details**:
- Exporter: Kenyan Coffee Cooperative
- Importer: US Coffee Roaster
- Amount: 50,000 USDC
- Required docs: Bill of Lading, Certificate of Origin, Inspection Certificate

**Timeline**:

**Day 0**: Trade created and confirmed
```
Issuing Bank → create_trade(...)
Confirming Bank → confirm_trade(trade_id: 1)
Importer → deposit(trade_id: 1, amount: 50000 USDC)
```

**Day 3**: Goods shipped from Mombasa
```
DHL Freight → Uploads B/L to IPFS
            → submit_document(
                trade_id: 1,
                doc_type: BillOfLading,
                doc_hash: 5a2e8f9c...,
                metadata_uri: "ipfs://QmBillOfLading123"
              )
```

**Day 3 + 2 hours**: Port authority verifies
```
Mombasa Port Authority → Downloads B/L from IPFS
                       → Verifies goods match manifest
                       → sign_document(
                           trade_id: 1,
                           doc_type: BillOfLading,
                           signer: port_authority_address
                         )
                       
Document Contract → Marks B/L as verified ✓
```

**Day 4**: Certificate of Origin submitted
```
Kenya Chamber of Commerce → submit_document(
                              trade_id: 1,
                              doc_type: CertificateOfOrigin,
                              doc_hash: 1b3c5d7e...,
                              metadata_uri: "ipfs://QmCoO456"
                            )

Kenya Revenue Authority → sign_document(
                            trade_id: 1,
                            doc_type: CertificateOfOrigin,
                            signer: kra_address
                          )

Document Contract → Marks CoO as verified ✓
```

**Day 5**: Inspection certificate submitted
```
SGS Inspection → submit_document(
                   trade_id: 1,
                   doc_type: InspectionCertificate,
                   doc_hash: 9f0e1d2c...,
                   metadata_uri: "ipfs://QmInspection789"
                 )

Exporter → sign_document(
             trade_id: 1,
             doc_type: InspectionCertificate,
             signer: exporter_address
           )

Document Contract → Marks inspection cert as verified ✓
```

**Day 5 + 1 hour**: All documents verified, settlement triggered
```
Anyone → settle_trade(trade_id: 1)

Trade Contract → all_docs_verified(trade_id: 1, required_docs)
               → Returns true

Trade Contract → Marks trade as Settled
               → Calls escrow.release(trade_id: 1, exporter)

Escrow Contract → Transfers 50,000 USDC to exporter

Exporter → Receives payment ✓
```

**Total time**: 5 days (vs. 60–90 days traditional)

**Total cost**: ~$0.0001 in transaction fees (vs. $500–1000 traditional)

## Security Considerations

### Hash Collision Attacks

**Risk**: Attacker finds two documents with same SHA-256 hash

**Likelihood**: Computationally infeasible (2^256 possible hashes)

**Mitigation**: Use SHA-256 (industry standard for document integrity)

### IPFS Availability

**Risk**: Document not available when counter-signer tries to verify

**Likelihood**: Medium (depends on pinning)

**Mitigation**: 
- Multiple parties pin documents
- Azaka runs IPFS pinning service
- Fall back to centralized storage if needed

### Malicious Counter-Signer

**Risk**: Counter-signer signs without actually verifying document

**Likelihood**: Low (reputation at stake)

**Mitigation**:
- Registry can revoke malicious participants
- Insurance products can cover fraud
- Reputation system (future feature)

### Compromised Private Key

**Risk**: Attacker gains access to participant's private key

**Likelihood**: Low (with proper key management)

**Mitigation**:
- Hardware security modules (HSMs) for banks
- Multi-sig wallets for high-value participants
- Immediate revocation upon compromise

## Future Enhancements

### Phase 2: Advanced Verification

- **Selective disclosure**: Zero-knowledge proofs for sensitive document fields
- **Automated verification**: AI/ML to check document compliance
- **Batch verification**: Verify multiple documents in one transaction

### Phase 3: Interoperability

- **Cross-chain verification**: Accept documents verified on other blockchains
- **Legacy system integration**: Bridge to SWIFT MT700 messages
- **API standardization**: Adopt emerging trade finance API standards

## Conclusion

Azaka's document verification system replaces slow, expensive, fraud-prone paper processes with:
- **Cryptographic integrity**: SHA-256 hash anchoring
- **Multi-party trust**: Multiple signatures required
- **Real-time transparency**: All parties see verification status
- **Cost efficiency**: ~$0.00001 per transaction
- **Privacy**: Documents stored off-chain

This is the foundation for instant, low-cost, fraud-resistant trade finance for African SME exporters.
