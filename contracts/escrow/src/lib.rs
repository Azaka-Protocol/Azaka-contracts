#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env, Symbol, symbol_short,
};

/// Storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Balance(u64),           // trade_id -> amount
    TradeContract,
    TokenContract,
}

/// Escrow contract errors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EscrowError {
    NotAuthorized = 1,
    InsufficientBalance = 2,
    AlreadyDeposited = 3,
    NoBalance = 4,
    TransferFailed = 5,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Initialize the escrow contract with trade contract address
    pub fn initialize(env: Env, trade_contract: Address, token_contract: Address) {
        if env.storage().instance().has(&DataKey::TradeContract) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::TradeContract, &trade_contract);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
    }

    /// Deposit stablecoin into escrow for a trade
    /// Callable by importer after trade creation
    pub fn deposit(
        env: Env,
        trade_id: u64,
        from: Address,
        amount: i128,
    ) -> Result<(), EscrowError> {
        from.require_auth();

        let key = DataKey::Balance(trade_id);
        
        if env.storage().persistent().has(&key) {
            return Err(EscrowError::AlreadyDeposited);
        }

        let token_contract: Address = env.storage().instance().get(&DataKey::TokenContract).unwrap();
        let token_client = token::Client::new(&env, &token_contract);

        // Transfer tokens from depositor to this contract
        token_client.transfer(&from, &env.current_contract_address(), &amount);

        env.storage().persistent().set(&key, &amount);

        // Emit event
        env.events().publish(
            (symbol_short!("deposit"), trade_id),
            (from, amount),
        );

        Ok(())
    }

    /// Release escrowed funds to exporter
    /// Only callable by trade contract after document verification
    pub fn release(
        env: Env,
        trade_id: u64,
        to: Address,
    ) -> Result<(), EscrowError> {
        let trade_contract: Address = env.storage().instance().get(&DataKey::TradeContract).unwrap();
        
        // Only trade contract can trigger release
        trade_contract.require_auth();

        let key = DataKey::Balance(trade_id);
        
        let amount: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(EscrowError::NoBalance)?;

        let token_contract: Address = env.storage().instance().get(&DataKey::TokenContract).unwrap();
        let token_client = token::Client::new(&env, &token_contract);

        // Transfer tokens to exporter
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        // Remove balance entry
        env.storage().persistent().remove(&key);

        // Emit event
        env.events().publish(
            (symbol_short!("release"), trade_id),
            (to, amount),
        );

        Ok(())
    }

    /// Refund escrowed funds to importer
    /// Callable on trade cancellation or expiry
    pub fn refund(
        env: Env,
        trade_id: u64,
        to: Address,
    ) -> Result<(), EscrowError> {
        let trade_contract: Address = env.storage().instance().get(&DataKey::TradeContract).unwrap();
        
        // Only trade contract can trigger refund
        trade_contract.require_auth();

        let key = DataKey::Balance(trade_id);
        
        let amount: i128 = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(EscrowError::NoBalance)?;

        let token_contract: Address = env.storage().instance().get(&DataKey::TokenContract).unwrap();
        let token_client = token::Client::new(&env, &token_contract);

        // Transfer tokens back to importer
        token_client.transfer(&env.current_contract_address(), &to, &amount);

        // Remove balance entry
        env.storage().persistent().remove(&key);

        // Emit event
        env.events().publish(
            (symbol_short!("refund"), trade_id),
            (to, amount),
        );

        Ok(())
    }

    /// Get escrowed balance for a trade
    pub fn get_balance(env: Env, trade_id: u64) -> i128 {
        let key = DataKey::Balance(trade_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, token, Env};

    fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
        token::StellarAssetClient::new(env, &env.register_stellar_asset_contract_v2(admin.clone()).address())
    }

    #[test]
    fn test_deposit_and_release() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let trade_contract = Address::generate(&env);
        let importer = Address::generate(&env);
        let exporter = Address::generate(&env);

        let token_admin = token::StellarAssetClient::new(&env, &env.register_stellar_asset_contract_v2(admin.clone()).address());
        token_admin.mint(&importer, &1000000);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        client.initialize(&trade_contract, &token_admin.address);

        // Deposit
        client.deposit(&1, &importer, &500000);
        assert_eq!(client.get_balance(&1), 500000);

        // Release
        client.release(&1, &exporter);
        assert_eq!(client.get_balance(&1), 0);
        assert_eq!(token_admin.balance(&exporter), 500000);
    }

    #[test]
    fn test_deposit_and_refund() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let trade_contract = Address::generate(&env);
        let importer = Address::generate(&env);

        let token_admin = token::StellarAssetClient::new(&env, &env.register_stellar_asset_contract_v2(admin.clone()).address());
        token_admin.mint(&importer, &1000000);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        client.initialize(&trade_contract, &token_admin.address);

        // Deposit
        client.deposit(&1, &importer, &500000);
        assert_eq!(client.get_balance(&1), 500000);

        // Refund
        client.refund(&1, &importer);
        assert_eq!(client.get_balance(&1), 0);
        assert_eq!(token_admin.balance(&importer), 1000000);
    }

    #[test]
    #[should_panic(expected = "AlreadyDeposited")]
    fn test_double_deposit_fails() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let trade_contract = Address::generate(&env);
        let importer = Address::generate(&env);

        let token_admin = token::StellarAssetClient::new(&env, &env.register_stellar_asset_contract_v2(admin.clone()).address());
        token_admin.mint(&importer, &1000000);

        let contract_id = env.register_contract(None, EscrowContract);
        let client = EscrowContractClient::new(&env, &contract_id);

        client.initialize(&trade_contract, &token_admin.address);

        client.deposit(&1, &importer, &500000);
        client.deposit(&1, &importer, &500000); // Should panic
    }
}
