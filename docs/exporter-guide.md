# Exporter Guide: Getting Started with Azaka

## Welcome!

This guide will help you, as an African SME exporter, use Azaka to receive faster, cheaper payments for your exports. No blockchain experience required.

## What is Azaka?

Azaka is a digital platform that makes international trade payments faster and cheaper. Instead of waiting 60–90 days for payment through traditional banks, you can receive payment in hours using Azaka.

### How It Works (Simple Version)

1. Your buyer (importer) creates a trade agreement on Azaka
2. Buyer locks payment in a secure digital escrow
3. You ship your goods and upload shipping documents
4. When documents are verified, payment is automatically released to you
5. You receive payment in stablecoins (digital dollars) that you can convert to local currency

### Benefits for You

- **Faster payment**: Hours instead of 60–90 days
- **Lower fees**: ~$0.01 instead of $500–1000
- **No bank rejection**: Access trade finance even if banks won't serve you
- **Full transparency**: See your payment status in real-time
- **Global reach**: Trade with any country without correspondent banks

## Step 1: Set Up Your Stellar Wallet

### What is a Stellar Wallet?

A Stellar wallet is like a digital bank account that holds your payment. It's secured by a secret key that only you control.

### Option A: Mobile Wallet (Easiest)

**Recommended**: Lobstr (available on iOS and Android)

1. Download Lobstr from App Store or Google Play
2. Create new wallet
3. **CRITICAL**: Write down your 24-word recovery phrase on paper
4. Store recovery phrase in a safe place (NOT on your phone)
5. Set up PIN or biometric lock

Your Stellar address will look like: `GEXPORTER...` (starts with G)

### Option B: Hardware Wallet (Most Secure)

For large trade values (>$50,000), use a hardware wallet:

1. Buy Ledger Nano X or Trezor Model T
2. Set up device following manufacturer instructions
3. Install Stellar app on device
4. Generate Stellar address

### Option C: Web Wallet

**For testing only** (not recommended for large amounts):

1. Visit https://stellar.org/laboratory
2. Generate keypair
3. Save secret key securely

### Security Tips

- **Never share your secret key** with anyone
- **Never store secret key** on your computer or phone
- **Write recovery phrase on paper**, not digitally
- **Make multiple copies** and store in different safe places
- **Test with small amounts** first

## Step 2: Get USDC on Testnet (For Testing)

Before using real money, test with Testnet (fake money):

### Create Testnet Account

```bash
# If you have Stellar CLI installed
stellar keys generate my-test-wallet --network testnet
stellar account fund my-test-wallet --network testnet
```

### Get Testnet USDC

1. Visit https://stellar.org/laboratory
2. Create trustline to USDC
3. Request testnet USDC from faucet

## Step 3: Create Your First Trade

### Prerequisites

- Buyer (importer) has agreed to use Azaka
- Buyer has created trade on Azaka
- You have received trade ID from buyer

### Accept Trade

Once buyer creates trade, you'll receive notification with:
- Trade ID
- Amount
- Required documents
- Expiry date

**Example**:
```
Trade ID: 1
Amount: 50,000 USDC
Required Documents:
  - Bill of Lading
  - Certificate of Origin
  - Inspection Certificate
Expiry: 30 days
```

### Confirm Trade Details

Check that trade details match your agreement:
- Correct amount
- Correct documents required
- Reasonable expiry date

If anything is wrong, contact buyer to cancel and recreate trade.

## Step 4: Ship Your Goods

Ship goods as normal through your freight forwarder. Make sure to:

1. Use a freight forwarder registered on Azaka
2. Request digital copies of all shipping documents
3. Keep copies of all documents for your records

### Required Documents

Depending on your trade, you may need:

#### Bill of Lading (B/L)
- Issued by freight forwarder or shipping line
- Proves goods were loaded onto vessel
- Most important document

#### Certificate of Origin (CoO)
- Issued by Chamber of Commerce
- Certifies country of manufacture
- Required for customs clearance

#### Inspection Certificate
- Issued by independent inspector (SGS, Bureau Veritas, etc.)
- Proves goods meet specifications
- Required for quality assurance

#### Phytosanitary Certificate
- For agricultural products only
- Issued by national phytosanitary authority
- Proves products are pest-free

#### Customs Declaration
- Issued by customs broker
- Proves goods cleared export customs
- Required for legal export

### Working with Freight Forwarders

**Tell your freight forwarder**:
- You're using Azaka for payment
- They need to submit documents to Azaka
- Provide them with your trade ID

**Azaka-registered forwarders** (as of 2026):
- DHL Freight (Kenya, Nigeria, Ghana)
- Maersk Line (Pan-African)
- Bollore Logistics (West Africa)

If your forwarder isn't registered, they can register at our website

## Step 5: Track Document Submission

### Check Document Status

You can check document status at our website(still in production)

**Status indicators**:
- ⏳ **Pending**: Document not yet submitted
- 📄 **Submitted**: Document submitted, awaiting signature
- ✅ **Verified**: Document signed and verified
- ❌ **Rejected**: Document rejected (rare)

### What If Documents Are Delayed?

If documents aren't submitted within a few days of shipping:

1. Contact your freight forwarder
2. Verify they have correct trade ID
3. Check they're registered on Azaka
4. If still delayed, contact Azaka support

### Document Verification Timeline

Typical timeline:
- **Bill of Lading**: 1–2 days after shipping
- **Certificate of Origin**: 2–3 days (depends on Chamber of Commerce)
- **Inspection Certificate**: 1 day (if pre-shipment inspection)
- **Phytosanitary Certificate**: 3–5 days (government processing)
- **Customs Declaration**: 1 day (after customs clearance)

## Step 6: Receive Payment

### Automatic Release

Once all documents are verified, payment is **automatically released** to your wallet. No action needed from you!

You'll receive notification:
```
✅ Trade #1 Settled
💰 50,000 USDC sent to your wallet
```

### Check Your Balance

**Lobstr app**:
1. Open Lobstr
2. Check USDC balance
3. Should show new payment

**Stellar Laboratory**:
1. Visit https://stellar.org/laboratory
2. Enter your address
3. View balances

### What If Payment Doesn't Arrive?

If all documents are verified but payment hasn't arrived after 1 hour:

1. Check trade status
2. Verify all documents show ✅ Verified
3. Check your wallet address is correct
4. Contact Azaka support: support@azaka.finance

## Step 7: Convert USDC to Local Currency

### Option A: Stellar Anchor (Recommended)

**Nigeria**:
1. Use Flutterwave or Chipper Cash
2. Send USDC to anchor's address
3. Receive Naira in your bank account
4. Typical fee: 1–2%
5. Time: 1–24 hours

**Kenya**:
1. Use Chipper Cash or BitPesa
2. Send USDC to anchor's address
3. Receive Kenyan Shillings in M-Pesa or bank
4. Typical fee: 1–2%
5. Time: 1–24 hours

**Ghana**:
1. Use Chipper Cash
2. Send USDC to anchor's address
3. Receive Cedis in mobile money or bank
4. Typical fee: 1–2%
5. Time: 1–24 hours

### Option B: Local Exchange

1. Create account on local crypto exchange
2. Send USDC to exchange
3. Sell USDC for local currency
4. Withdraw to bank account

**Recommended exchanges**:
- **Nigeria**: Quidax, Luno, Busha
- **Kenya**: Binance, Paxful
- **Ghana**: Luno, Binance
- **South Africa**: Luno, VALR

### Option C: Peer-to-Peer (P2P)

1. Find buyer on P2P platform (LocalBitcoins, Paxful)
2. Agree on exchange rate
3. Send USDC to buyer
4. Receive local currency in bank or mobile money

**Caution**: Only use reputable P2P platforms with escrow protection.

### Fees Comparison

| Method | Fee | Time | Ease |
|--------|-----|------|------|
| Stellar Anchor | 1–2% | 1–24 hours | Easy |
| Local Exchange | 0.5–1.5% | 1–3 days | Medium |
| P2P | 2–5% | Minutes–hours | Hard |

## Real Example: Kenyan Coffee Export

**Exporter**: Kenyan Coffee Cooperative  
**Importer**: US Coffee Roaster  
**Amount**: $50,000  
**Product**: 1000 bags of Arabica coffee

### Timeline

**Day 0**: Trade created
- Importer creates trade on Azaka
- Importer deposits $50,000 USDC into escrow
- Exporter receives notification

**Day 1**: Goods shipped
- Exporter ships coffee from Mombasa port
- DHL Freight issues Bill of Lading
- DHL submits B/L to Azaka

**Day 2**: B/L verified
- Mombasa Port Authority signs B/L
- B/L marked as verified ✅

**Day 3**: Certificate of Origin submitted
- Kenya Chamber of Commerce submits CoO
- Kenya Revenue Authority signs CoO
- CoO marked as verified ✅

**Day 4**: Inspection certificate submitted
- SGS Inspection submits certificate
- Exporter signs certificate
- Certificate marked as verified ✅

**Day 4 + 1 hour**: Payment released
- All documents verified
- Azaka automatically releases $50,000 USDC to exporter
- Exporter receives notification

**Day 5**: Convert to local currency
- Exporter sends USDC to Chipper Cash
- Receives 6,500,000 KES in M-Pesa
- Total time: 5 days (vs. 60–90 days traditional)

### Cost Comparison

**Traditional LC**:
- Bank fees: $800
- Correspondent bank fees: $500
- Document courier: $100
- Amendment fees: $200
- **Total**: $1,600 (3.2% of trade value)

**Azaka**:
- Transaction fees: $0.01
- Anchor conversion fee: $500 (1%)
- **Total**: $500.01 (1% of trade value)

**Savings**: $1,099.99 (69% cheaper)

## Troubleshooting

### Problem: Trade Expired Before Documents Verified

**Cause**: Documents took too long to submit

**Solution**:
- Contact buyer to create new trade with longer expiry
- Work with faster freight forwarder
- Pre-arrange documents before shipping

### Problem: Document Rejected

**Cause**: Document doesn't match LC terms

**Solution**:
- Review rejection reason
- Correct document and resubmit
- Contact buyer if LC terms are wrong

### Problem: Freight Forwarder Won't Use Azaka

**Cause**: Forwarder not familiar with Azaka

**Solution**:
- Share this guide with forwarder
- Ask forwarder to register on Azaka
- Consider switching to Azaka-registered forwarder

### Problem: Can't Convert USDC to Local Currency

**Cause**: No anchor or exchange in your country

**Solution**:
- Use international exchange (Binance, Kraken)
- Use P2P platform
- Contact Azaka to request anchor in your country

### Problem: Wallet Hacked

**Cause**: Secret key compromised

**Solution**:
- **Immediately** create new wallet
- Transfer any remaining funds to new wallet
- Report to Azaka support
- Review security practices

**Prevention**:
- Never share secret key
- Use hardware wallet for large amounts
- Enable 2FA on all accounts

## FAQ

### Is Azaka safe?

Yes. Azaka uses smart contracts on Stellar blockchain, which is:
- Audited by security firms
- Used by major financial institutions
- Secured by cryptography

However, **you** are responsible for securing your wallet.

### What if buyer doesn't deposit escrow?

Trade cannot proceed without escrow. If buyer doesn't deposit within agreed time, cancel trade and use traditional payment method.

### Can I cancel a trade?

Yes, but only before escrow is deposited. After escrow deposit, trade can only be cancelled by mutual agreement or expiry.

### What if documents are lost?

Documents are stored on IPFS (decentralized storage) and cannot be lost. Even if your freight forwarder loses their copy, the document is still on IPFS.

### Do I need to pay taxes on USDC?

**Yes**. USDC is income and must be reported to tax authorities. Consult local tax advisor.

### Can I use Azaka for domestic trades?

Currently, Azaka is designed for international trade. Domestic trade support coming soon.

### What if my country bans cryptocurrency?

Azaka uses stablecoins, which are often treated differently than cryptocurrency. However, you should comply with local laws. Consult legal advisor.



## Next Steps

1. ✅ Set up Stellar wallet
2. ✅ Test with small trade on Testnet
3. ✅ Register with Azaka
4. ✅ Onboard your freight forwarder
5. ✅ Create your first real trade
6. ✅ Join Azaka community


---

**Disclaimer**: This guide is for informational purposes only. Azaka is experimental software. Conduct your own due diligence before using for large trade values. Not financial or legal advice.
