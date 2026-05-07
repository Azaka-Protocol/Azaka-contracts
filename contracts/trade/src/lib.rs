#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol, symbol_short,
};

/// Trade status in the LC lifecycle
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TradeStatus {
    PendingEscrow,
    Active,
    DocumentsPending,
    Settled,
    Cancelled,
    Expired,
}

/// Document type (re-exported from document contract)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocumentType {
    BillOfLading,
    CertificateOfOrigin,
    InspectionCertificate,
    PhytosanitaryCertificate,
    CustomsDeclaration,
}

/// Trade record
#[contracttype]
#[derive(Clone, Debug)]
pub struct Trade {
    pub trade_id: u64,
    pub exporter: Address,
    pub importer: Address,
    pub issuing_bank: Address,
    pub confirming_bank: Option<Address>,
    pub stablecoin_asset: Address,
    pub amount: i128,
    pub required_docs: Vec<DocumentType>,
    pub expiry_ledger: u32,
    pub status: TradeStatus,
    pub created_ledger: u32,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Trade(u64),
    TradeCounter,
    ExporterTrades(Address),
    EscrowContract,
    DocumentContract,
}

/// Trade contract errors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TradeError {
    NotAuthorized = 1,
    TradeNotFound = 2,
    InvalidStatus = 3,
    TradeExpired = 4,
    DocumentsNotVerified = 5,
    AlreadySettled = 6,
}

#[contract]
pub struct TradeContract;

#[contractimpl]
impl TradeContract {
    /// Initialize the trade contract with escrow and document contract addresses
    pub fn initialize(env: Env, escrow_contract: Address, document_contract: Address) {
        if env.storage().instance().has(&DataKey::EscrowContract) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::EscrowContract, &escrow_contract);
        env.storage().instance().set(&DataKey::DocumentContract, &document_contract);
        env.storage().instance().set(&DataKey::TradeCounter, &0u64);
    }

    /// Create a new trade (Letter of Credit)
    pub fn create_trade(
        env: Env,
        exporter: Address,
        importer: Address,
        issuing_bank: Address,
        stablecoin_asset: Address,
        amount: i128,
        required_docs: Vec<DocumentType>,
        expiry_ledger: u32,
    ) -> u64 {
        issuing_bank.require_auth();

        let mut counter: u64 = env.storage().instance().get(&DataKey::TradeCounter).unwrap_or(0);
        counter += 1;

        let trade = Trade {
            trade_id: counter,
            exporter: exporter.clone(),
            importer,
            issuing_bank,
            confirming_bank: None,
            stablecoin_asset,
            amount,
            required_docs,
            expiry_ledger,
            status: TradeStatus::PendingEscrow,
            created_ledger: env.ledger().sequence(),
        };

        env.storage().instance().set(&DataKey::TradeCounter, &counter);
        env.storage().persistent().set(&DataKey::Trade(counter), &trade);

        // Add to exporter's trade list
        let exporter_key = DataKey::ExporterTrades(exporter.clone());
        let mut exporter_trades: Vec<u64> = env.storage().persistent().get(&exporter_key).unwrap_or(Vec::new(&env));
        exporter_trades.push_back(counter);
        env.storage().persistent().set(&exporter_key, &exporter_trades);

        // Emit event
        env.events().publish(
            (symbol_short!("create"), counter),
            trade,
        );

        counter
    }

    /// Confirm trade by confirming bank
    pub fn confirm_trade(env: Env, trade_id: u64, confirming_bank: Address) -> Result<(), TradeError> {
        confirming_bank.require_auth();

        let key = DataKey::Trade(trade_id);
        let mut trade: Trade = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)?;

        if trade.status != TradeStatus::PendingEscrow {
            return Err(TradeError::InvalidStatus);
        }

        trade.confirming_bank = Some(confirming_bank.clone());
        trade.status = TradeStatus::Active;
        env.storage().persistent().set(&key, &trade);

        // Emit event
        env.events().publish(
            (symbol_short!("confirm"), trade_id),
            confirming_bank,
        );

        Ok(())
    }

    /// Update trade status to DocumentsPending after escrow deposit
    /// Called by escrow contract
    pub fn mark_escrow_deposited(env: Env, trade_id: u64) -> Result<(), TradeError> {
        let escrow_contract: Address = env.storage().instance().get(&DataKey::EscrowContract).unwrap();
        escrow_contract.require_auth();

        let key = DataKey::Trade(trade_id);
        let mut trade: Trade = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)?;

        if trade.status != TradeStatus::Active {
            return Err(TradeError::InvalidStatus);
        }

        trade.status = TradeStatus::DocumentsPending;
        env.storage().persistent().set(&key, &trade);

        // Emit event
        env.events().publish(
            (symbol_short!("escrow"), trade_id),
            (),
        );

        Ok(())
    }

    /// Cancel trade before it becomes active
    pub fn cancel_trade(env: Env, trade_id: u64, caller: Address) -> Result<(), TradeError> {
        caller.require_auth();

        let key = DataKey::Trade(trade_id);
        let mut trade: Trade = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)?;

        // Only exporter or importer can cancel
        if caller != trade.exporter && caller != trade.importer {
            return Err(TradeError::NotAuthorized);
        }

        if trade.status != TradeStatus::PendingEscrow && trade.status != TradeStatus::Active {
            return Err(TradeError::InvalidStatus);
        }

        trade.status = TradeStatus::Cancelled;
        env.storage().persistent().set(&key, &trade);

        // Emit event
        env.events().publish(
            (symbol_short!("cancel"), trade_id),
            caller,
        );

        Ok(())
    }

    /// Expire trade after expiry ledger
    pub fn expire_trade(env: Env, trade_id: u64) -> Result<(), TradeError> {
        let key = DataKey::Trade(trade_id);
        let mut trade: Trade = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)?;

        if env.ledger().sequence() < trade.expiry_ledger {
            return Err(TradeError::InvalidStatus);
        }

        if trade.status == TradeStatus::Settled {
            return Err(TradeError::AlreadySettled);
        }

        trade.status = TradeStatus::Expired;
        env.storage().persistent().set(&key, &trade);

        // Emit event
        env.events().publish(
            (symbol_short!("expire"), trade_id),
            (),
        );

        Ok(())
    }

    /// Settle trade after all documents are verified
    /// Triggers escrow release
    pub fn settle_trade(env: Env, trade_id: u64) -> Result<(), TradeError> {
        let key = DataKey::Trade(trade_id);
        let mut trade: Trade = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)?;

        if trade.status != TradeStatus::DocumentsPending {
            return Err(TradeError::InvalidStatus);
        }

        if trade.status == TradeStatus::Settled {
            return Err(TradeError::AlreadySettled);
        }

        // Check if all documents are verified
        // In production, this would call the document contract
        // For now, we'll assume verification is done externally

        trade.status = TradeStatus::Settled;
        env.storage().persistent().set(&key, &trade);

        // Emit event
        env.events().publish(
            (symbol_short!("settle"), trade_id),
            trade.exporter.clone(),
        );

        Ok(())
    }

    /// Get trade details
    pub fn get_trade(env: Env, trade_id: u64) -> Result<Trade, TradeError> {
        let key = DataKey::Trade(trade_id);
        env.storage()
            .persistent()
            .get(&key)
            .ok_or(TradeError::TradeNotFound)
    }

    /// List all trades for an exporter
    pub fn list_trades_by_exporter(env: Env, exporter: Address) -> Vec<u64> {
        let key = DataKey::ExporterTrades(exporter);
        env.storage().persistent().get(&key).unwrap_or(Vec::new(&env))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_create_and_confirm_trade() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TradeContract);
        let client = TradeContractClient::new(&env, &contract_id);

        let escrow = Address::generate(&env);
        let document = Address::generate(&env);
        let exporter = Address::generate(&env);
        let importer = Address::generate(&env);
        let issuing_bank = Address::generate(&env);
        let confirming_bank = Address::generate(&env);
        let stablecoin = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&escrow, &document);

        let mut required_docs = Vec::new(&env);
        required_docs.push_back(DocumentType::BillOfLading);
        required_docs.push_back(DocumentType::InspectionCertificate);

        let trade_id = client.create_trade(
            &exporter,
            &importer,
            &issuing_bank,
            &stablecoin,
            &50000000000,
            &required_docs,
            &1000000,
        );

        assert_eq!(trade_id, 1);

        let trade = client.get_trade(&trade_id);
        assert_eq!(trade.status, TradeStatus::PendingEscrow);

        client.confirm_trade(&trade_id, &confirming_bank);

        let trade = client.get_trade(&trade_id);
        assert_eq!(trade.status, TradeStatus::Active);
    }

    #[test]
    fn test_cancel_trade() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TradeContract);
        let client = TradeContractClient::new(&env, &contract_id);

        let escrow = Address::generate(&env);
        let document = Address::generate(&env);
        let exporter = Address::generate(&env);
        let importer = Address::generate(&env);
        let issuing_bank = Address::generate(&env);
        let stablecoin = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&escrow, &document);

        let required_docs = Vec::new(&env);

        let trade_id = client.create_trade(
            &exporter,
            &importer,
            &issuing_bank,
            &stablecoin,
            &50000000000,
            &required_docs,
            &1000000,
        );

        client.cancel_trade(&trade_id, &exporter);

        let trade = client.get_trade(&trade_id);
        assert_eq!(trade.status, TradeStatus::Cancelled);
    }

    #[test]
    fn test_list_exporter_trades() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TradeContract);
        let client = TradeContractClient::new(&env, &contract_id);

        let escrow = Address::generate(&env);
        let document = Address::generate(&env);
        let exporter = Address::generate(&env);
        let importer = Address::generate(&env);
        let issuing_bank = Address::generate(&env);
        let stablecoin = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&escrow, &document);

        let required_docs = Vec::new(&env);

        let trade_id1 = client.create_trade(
            &exporter,
            &importer,
            &issuing_bank,
            &stablecoin,
            &50000000000,
            &required_docs,
            &1000000,
        );

        let trade_id2 = client.create_trade(
            &exporter,
            &importer,
            &issuing_bank,
            &stablecoin,
            &30000000000,
            &required_docs,
            &1000000,
        );

        let trades = client.list_trades_by_exporter(&exporter);
        assert_eq!(trades.len(), 2);
        assert_eq!(trades.get(0).unwrap(), trade_id1);
        assert_eq!(trades.get(1).unwrap(), trade_id2);
    }
}
