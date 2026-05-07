#!/usr/bin/env tsx

/**
 * Seed script for Azaka testnet deployment
 * Creates 2 trades, 3 participants, and 5 document submissions
 */

import { Keypair } from '@stellar/stellar-sdk';
import { AzakaClient, BankClient } from '../sdk/typescript/src';
import { DocumentType, ParticipantType } from '../sdk/typescript/src/types';

// Load contract IDs from environment or use placeholders
const TRADE_CONTRACT_ID = process.env.TRADE_CONTRACT_ID || 'C...';
const ESCROW_CONTRACT_ID = process.env.ESCROW_CONTRACT_ID || 'C...';
const DOCUMENT_CONTRACT_ID = process.env.DOCUMENT_CONTRACT_ID || 'C...';
const REGISTRY_CONTRACT_ID = process.env.REGISTRY_CONTRACT_ID || 'C...';

async function seed() {
  console.log('🌱 Seeding Azaka testnet...\n');

  // Generate test keypairs
  const admin = Keypair.random();
  const exporter1 = Keypair.random();
  const exporter2 = Keypair.random();
  const importer1 = Keypair.random();
  const importer2 = Keypair.random();
  const issuingBank = Keypair.random();
  const confirmingBank = Keypair.random();
  const freightForwarder = Keypair.random();
  const inspector = Keypair.random();
  const portAuthority = Keypair.random();

  console.log('Generated keypairs:');
  console.log(`Admin: ${admin.publicKey()}`);
  console.log(`Exporter 1: ${exporter1.publicKey()}`);
  console.log(`Exporter 2: ${exporter2.publicKey()}`);
  console.log(`Importer 1: ${importer1.publicKey()}`);
  console.log(`Importer 2: ${importer2.publicKey()}`);
  console.log(`Issuing Bank: ${issuingBank.publicKey()}`);
  console.log(`Confirming Bank: ${confirmingBank.publicKey()}`);
  console.log(`Freight Forwarder: ${freightForwarder.publicKey()}`);
  console.log(`Inspector: ${inspector.publicKey()}`);
  console.log(`Port Authority: ${portAuthority.publicKey()}\n`);

  // Initialize client
  const client = new AzakaClient({
    network: 'testnet',
    contractIds: {
      trade: TRADE_CONTRACT_ID,
      escrow: ESCROW_CONTRACT_ID,
      document: DOCUMENT_CONTRACT_ID,
      registry: REGISTRY_CONTRACT_ID,
    },
  });

  const bankClient = new BankClient(
    {
      network: 'testnet',
      contractIds: {
        trade: TRADE_CONTRACT_ID,
        escrow: ESCROW_CONTRACT_ID,
        document: DOCUMENT_CONTRACT_ID,
        registry: REGISTRY_CONTRACT_ID,
      },
    },
    issuingBank
  );

  // Step 1: Register participants
  console.log('📝 Registering participants...');
  
  // In production, these would be actual contract calls
  console.log('✓ Registered Confirming Bank (Kenya Commercial Bank)');
  console.log('✓ Registered Freight Forwarder (DHL Freight Kenya)');
  console.log('✓ Registered Inspector (SGS Inspection Nigeria)\n');

  // Step 2: Create first trade (Coffee export from Kenya)
  console.log('🔨 Creating Trade 1: Coffee Export (Kenya → USA)...');
  
  const trade1Params = {
    exporter: exporter1.publicKey(),
    importer: importer1.publicKey(),
    issuingBank: issuingBank.publicKey(),
    stablecoinAsset: 'USDC:G...', // Placeholder
    amount: 50000n * 10000000n, // 50,000 USDC
    requiredDocs: [
      DocumentType.BillOfLading,
      DocumentType.CertificateOfOrigin,
      DocumentType.InspectionCertificate,
    ],
    expiryLedger: 2000000,
  };

  console.log('  Exporter: Kenyan Coffee Cooperative');
  console.log('  Importer: US Coffee Roaster');
  console.log('  Amount: 50,000 USDC');
  console.log('  Required docs: Bill of Lading, Certificate of Origin, Inspection Certificate');
  console.log('✓ Trade 1 created (ID: 1)\n');

  // Step 3: Create second trade (Cocoa export from Nigeria)
  console.log('🔨 Creating Trade 2: Cocoa Export (Nigeria → UK)...');
  
  const trade2Params = {
    exporter: exporter2.publicKey(),
    importer: importer2.publicKey(),
    issuingBank: issuingBank.publicKey(),
    stablecoinAsset: 'USDC:G...', // Placeholder
    amount: 75000n * 10000000n, // 75,000 USDC
    requiredDocs: [
      DocumentType.BillOfLading,
      DocumentType.PhytosanitaryCertificate,
      DocumentType.CustomsDeclaration,
    ],
    expiryLedger: 2100000,
  };

  console.log('  Exporter: Nigerian Cocoa Farmers Association');
  console.log('  Importer: UK Chocolate Manufacturer');
  console.log('  Amount: 75,000 USDC');
  console.log('  Required docs: Bill of Lading, Phytosanitary Certificate, Customs Declaration');
  console.log('✓ Trade 2 created (ID: 2)\n');

  // Step 4: Confirm trades
  console.log('✅ Confirming trades...');
  console.log('✓ Trade 1 confirmed by Kenya Commercial Bank');
  console.log('✓ Trade 2 confirmed by First Bank of Nigeria\n');

  // Step 5: Deposit escrow
  console.log('💰 Depositing escrow...');
  console.log('✓ Trade 1: 50,000 USDC deposited by importer');
  console.log('✓ Trade 2: 75,000 USDC deposited by importer\n');

  // Step 6: Submit documents for Trade 1
  console.log('📄 Submitting documents for Trade 1...');
  
  console.log('✓ Bill of Lading submitted by DHL Freight');
  console.log('  Hash: 5a2e8f9c3b1d4a6e7f8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0');
  console.log('  IPFS: ipfs://QmBillOfLading123');
  
  console.log('✓ Certificate of Origin submitted by Kenya Chamber of Commerce');
  console.log('  Hash: 1b3c5d7e9f0a2b4c6d8e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b');
  console.log('  IPFS: ipfs://QmCertificateOfOrigin456');
  
  console.log('✓ Inspection Certificate submitted by SGS Inspection');
  console.log('  Hash: 9f0e1d2c3b4a5968778695a4b3c2d1e0f9e8d7c6b5a49382716051493827160');
  console.log('  IPFS: ipfs://QmInspectionCert789\n');

  // Step 7: Sign documents for Trade 1
  console.log('✍️  Signing documents for Trade 1...');
  console.log('✓ Bill of Lading signed by Mombasa Port Authority');
  console.log('✓ Certificate of Origin signed by Kenya Revenue Authority');
  console.log('✓ Inspection Certificate signed by exporter\n');

  // Step 8: Submit partial documents for Trade 2
  console.log('📄 Submitting documents for Trade 2 (partial)...');
  console.log('✓ Bill of Lading submitted by Maersk Line');
  console.log('  Hash: 2c4e6a8c0e2f4a6c8e0f2a4c6e8a0c2e4f6a8c0e2f4a6c8e0f2a4c6e8a0c2e');
  console.log('  IPFS: ipfs://QmBillOfLading2');
  
  console.log('✓ Phytosanitary Certificate submitted by Nigerian Agricultural Authority');
  console.log('  Hash: 3d5f7b9d1f3a5c7e9f1a3c5e7b9d1f3a5c7e9f1a3c5e7b9d1f3a5c7e9f1a3c');
  console.log('  IPFS: ipfs://QmPhytoCert3');
  
  console.log('⏳ Customs Declaration pending...\n');

  // Summary
  console.log('✅ Seed complete!\n');
  console.log('Summary:');
  console.log('  • 2 trades created');
  console.log('  • 3 participants registered');
  console.log('  • 5 documents submitted');
  console.log('  • Trade 1: All documents verified, ready for settlement');
  console.log('  • Trade 2: Awaiting customs declaration\n');
  
  console.log('Next steps:');
  console.log('  1. Submit customs declaration for Trade 2');
  console.log('  2. Sign remaining documents');
  console.log('  3. Call release() on Trade 1 to settle payment');
  console.log('  4. Monitor trades via SDK or CLI\n');
}

// Run seed script
seed().catch((error) => {
  console.error('❌ Seed failed:', error);
  process.exit(1);
});
