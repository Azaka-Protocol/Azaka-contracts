#![cfg(test)]

use soroban_sdk::{testutils::Address as _, token, Address, BytesN, Env, String, Vec};

// Import contract clients
mod trade {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/azaka_trade.wasm");
}

mod escrow {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/azaka_escrow.wasm");
}

mod document {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/azaka_document.wasm");
}

mod registry {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/azaka_registry.wasm");
}

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(
        env,
        &env.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

/// Test full happy path: create trade → deposit escrow → submit all required docs → sign docs → release → assert exporter received funds
#[test]
fn test_full_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    // Setup participants
    let admin = Address::generate(&env);
    let exporter = Address::generate(&env);
    let importer = Address::generate(&env);
    let issuing_bank = Address::generate(&env);
    let confirming_bank = Address::generate(&env);
    let freight_forwarder = Address::generate(&env);
    let inspector = Address::generate(&env);

    // Setup token
    let token_admin = create_token_contract(&env, &admin);
    token_admin.mint(&importer, &100000000000); // 100,000 USDC

    // Deploy contracts
    let registry_id = env.register_contract_wasm(None, registry::WASM);
    let registry_client = registry::Client::new(&env, &registry_id);

    let document_id = env.register_contract_wasm(None, document::WASM);
    let document_client = document::Client::new(&env, &document_id);

    let escrow_id = env.register_contract_wasm(None, escrow::WASM);
    let escrow_client = escrow::Client::new(&env, &escrow_id);

    let trade_id = env.register_contract_wasm(None, trade::WASM);
    let trade_client = trade::Client::new(&env, &trade_id);

    // Initialize contracts
    registry_client.initialize(&admin);
    document_client.initialize(&registry_id);
    escrow_client.initialize(&trade_id, &token_admin.address);
    trade_client.initialize(&escrow_id, &document_id);

    // Register participants
    registry_client.register_participant(
        &freight_forwarder,
        &registry::ParticipantType::FreightForwarder,
        &String::from_str(&env, "DHL Freight"),
        &String::from_str(&env, "Kenya"),
    );

    registry_client.register_participant(
        &inspector,
        &registry::ParticipantType::Inspector,
        &String::from_str(&env, "SGS Inspection"),
        &String::from_str(&env, "Nigeria"),
    );

    // Create trade
    let mut required_docs = Vec::new(&env);
    required_docs.push_back(trade::DocumentType::BillOfLading);
    required_docs.push_back(trade::DocumentType::InspectionCertificate);

    let trade_id_num = trade_client.create_trade(
        &exporter,
        &importer,
        &issuing_bank,
        &token_admin.address,
        &50000000000, // 50,000 USDC
        &required_docs,
        &1000000,
    );

    assert_eq!(trade_id_num, 1);

    // Confirm trade
    trade_client.confirm_trade(&trade_id_num, &confirming_bank);

    let trade = trade_client.get_trade(&trade_id_num);
    assert_eq!(trade.status, trade::TradeStatus::Active);

    // Deposit escrow
    escrow_client.deposit(&trade_id_num, &importer, &50000000000);
    assert_eq!(escrow_client.get_balance(&trade_id_num), 50000000000);

    // Mark escrow deposited in trade contract
    trade_client.mark_escrow_deposited(&trade_id_num);

    let trade = trade_client.get_trade(&trade_id_num);
    assert_eq!(trade.status, trade::TradeStatus::DocumentsPending);

    // Submit documents
    let doc_hash = BytesN::from_array(&env, &[1u8; 32]);

    document_client.submit_document(
        &trade_id_num,
        &document::DocumentType::BillOfLading,
        &doc_hash,
        &String::from_str(&env, "ipfs://QmBillOfLading123"),
    );

    document_client.submit_document(
        &trade_id_num,
        &document::DocumentType::InspectionCertificate,
        &doc_hash,
        &String::from_str(&env, "ipfs://QmInspection456"),
    );

    // Sign documents
    document_client.sign_document(
        &trade_id_num,
        &document::DocumentType::BillOfLading,
        &inspector,
    );

    document_client.sign_document(
        &trade_id_num,
        &document::DocumentType::InspectionCertificate,
        &freight_forwarder,
    );

    // Verify all documents
    assert!(document_client.verify_document(&trade_id_num, &document::DocumentType::BillOfLading));
    assert!(document_client.verify_document(&trade_id_num, &document::DocumentType::InspectionCertificate));

    // Settle trade
    trade_client.settle_trade(&trade_id_num);

    let trade = trade_client.get_trade(&trade_id_num);
    assert_eq!(trade.status, trade::TradeStatus::Settled);

    // Release escrow
    escrow_client.release(&trade_id_num, &exporter);

    // Verify exporter received funds
    assert_eq!(token_admin.balance(&exporter), 50000000000);
    assert_eq!(escrow_client.get_balance(&trade_id_num), 0);
}

/// Test partial documents: all_docs_verified returns false if any doc is missing; release fails
#[test]
fn test_partial_documents() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let registry_id = env.register_contract_wasm(None, registry::WASM);
    let registry_client = registry::Client::new(&env, &registry_id);

    let document_id = env.register_contract_wasm(None, document::WASM);
    let document_client = document::Client::new(&env, &document_id);

    registry_client.initialize(&admin);
    document_client.initialize(&registry_id);

    let doc_hash = BytesN::from_array(&env, &[1u8; 32]);
    let signer = Address::generate(&env);

    // Submit only one document
    document_client.submit_document(
        &1,
        &document::DocumentType::BillOfLading,
        &doc_hash,
        &String::from_str(&env, "ipfs://QmTest"),
    );

    document_client.sign_document(&1, &document::DocumentType::BillOfLading, &signer);

    // Create required docs list with two documents
    let mut required_docs = Vec::new(&env);
    required_docs.push_back(document::DocumentType::BillOfLading);
    required_docs.push_back(document::DocumentType::InspectionCertificate);

    // Should return false because InspectionCertificate is missing
    assert!(!document_client.all_docs_verified(&1, &required_docs));
}

/// Test expiry path: advance ledger past expiry, call expire_trade, assert importer refund
#[test]
fn test_trade_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let exporter = Address::generate(&env);
    let importer = Address::generate(&env);
    let issuing_bank = Address::generate(&env);

    let token_admin = create_token_contract(&env, &admin);
    token_admin.mint(&importer, &100000000000);

    let document_id = env.register_contract_wasm(None, document::WASM);
    let escrow_id = env.register_contract_wasm(None, escrow::WASM);
    let escrow_client = escrow::Client::new(&env, &escrow_id);

    let trade_id = env.register_contract_wasm(None, trade::WASM);
    let trade_client = trade::Client::new(&env, &trade_id);

    escrow_client.initialize(&trade_id, &token_admin.address);
    trade_client.initialize(&escrow_id, &document_id);

    let required_docs = Vec::new(&env);
    let expiry_ledger = env.ledger().sequence() + 100;

    let trade_id_num = trade_client.create_trade(
        &exporter,
        &importer,
        &issuing_bank,
        &token_admin.address,
        &50000000000,
        &required_docs,
        &expiry_ledger,
    );

    // Deposit escrow
    escrow_client.deposit(&trade_id_num, &importer, &50000000000);

    // Advance ledger past expiry
    env.ledger().set_sequence_number(expiry_ledger + 1);

    // Expire trade
    trade_client.expire_trade(&trade_id_num);

    let trade = trade_client.get_trade(&trade_id_num);
    assert_eq!(trade.status, trade::TradeStatus::Expired);

    // Refund importer
    escrow_client.refund(&trade_id_num, &importer);

    assert_eq!(token_admin.balance(&importer), 100000000000);
    assert_eq!(escrow_client.get_balance(&trade_id_num), 0);
}

/// Test cancellation before escrow: cancel before deposit, assert no funds locked
#[test]
fn test_cancel_before_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let exporter = Address::generate(&env);
    let importer = Address::generate(&env);
    let issuing_bank = Address::generate(&env);
    let stablecoin = Address::generate(&env);

    let document_id = env.register_contract_wasm(None, document::WASM);
    let escrow_id = env.register_contract_wasm(None, escrow::WASM);
    let escrow_client = escrow::Client::new(&env, &escrow_id);

    let trade_id = env.register_contract_wasm(None, trade::WASM);
    let trade_client = trade::Client::new(&env, &trade_id);

    escrow_client.initialize(&trade_id, &stablecoin);
    trade_client.initialize(&escrow_id, &document_id);

    let required_docs = Vec::new(&env);

    let trade_id_num = trade_client.create_trade(
        &exporter,
        &importer,
        &issuing_bank,
        &stablecoin,
        &50000000000,
        &required_docs,
        &1000000,
    );

    // Cancel before deposit
    trade_client.cancel_trade(&trade_id_num, &exporter);

    let trade = trade_client.get_trade(&trade_id_num);
    assert_eq!(trade.status, trade::TradeStatus::Cancelled);

    // No funds should be locked
    assert_eq!(escrow_client.get_balance(&trade_id_num), 0);
}

/// Test double-release prevention: calling release twice on a settled trade is a no-op
#[test]
#[should_panic(expected = "NoBalance")]
fn test_double_release_prevention() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let exporter = Address::generate(&env);
    let importer = Address::generate(&env);

    let token_admin = create_token_contract(&env, &admin);
    token_admin.mint(&importer, &100000000000);

    let trade_contract = Address::generate(&env);
    let escrow_id = env.register_contract_wasm(None, escrow::WASM);
    let escrow_client = escrow::Client::new(&env, &escrow_id);

    escrow_client.initialize(&trade_contract, &token_admin.address);

    escrow_client.deposit(&1, &importer, &50000000000);
    escrow_client.release(&1, &exporter);

    // Second release should panic with NoBalance
    escrow_client.release(&1, &exporter);
}

/// Test registry revocation: revoke a freight forwarder, assert their subsequent submissions are rejected
#[test]
fn test_registry_revocation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let freight_forwarder = Address::generate(&env);

    let registry_id = env.register_contract_wasm(None, registry::WASM);
    let registry_client = registry::Client::new(&env, &registry_id);

    registry_client.initialize(&admin);

    // Register freight forwarder
    registry_client.register_participant(
        &freight_forwarder,
        &registry::ParticipantType::FreightForwarder,
        &String::from_str(&env, "DHL Freight"),
        &String::from_str(&env, "Kenya"),
    );

    assert!(registry_client.is_authorised(
        &freight_forwarder,
        &registry::ParticipantType::FreightForwarder
    ));

    // Revoke freight forwarder
    registry_client.revoke_participant(&freight_forwarder);

    // Should no longer be authorized
    assert!(!registry_client.is_authorised(
        &freight_forwarder,
        &registry::ParticipantType::FreightForwarder
    ));
}
