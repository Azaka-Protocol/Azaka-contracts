#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short};

/// Participant type in the trade finance ecosystem
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParticipantType {
    IssuingBank,
    ConfirmingBank,
    FreightForwarder,
    Inspector,
    PortAuthority,
}

/// Participant record
#[contracttype]
#[derive(Clone, Debug)]
pub struct Participant {
    pub address: Address,
    pub participant_type: ParticipantType,
    pub name: String,
    pub jurisdiction: String,
    pub active: bool,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Participant(Address),
    Admin,
}

/// Registry contract errors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryError {
    NotAuthorized = 1,
    ParticipantNotFound = 2,
    ParticipantAlreadyExists = 3,
    ParticipantRevoked = 4,
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Initialize the registry with an admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Register a new participant
    /// Only callable by admin
    pub fn register_participant(
        env: Env,
        address: Address,
        participant_type: ParticipantType,
        name: String,
        jurisdiction: String,
    ) -> Result<(), RegistryError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Participant(address.clone());
        
        if env.storage().persistent().has(&key) {
            return Err(RegistryError::ParticipantAlreadyExists);
        }

        let participant = Participant {
            address: address.clone(),
            participant_type,
            name,
            jurisdiction,
            active: true,
        };

        env.storage().persistent().set(&key, &participant);
        
        // Emit event
        env.events().publish(
            (symbol_short!("reg_part"), address),
            participant,
        );

        Ok(())
    }

    /// Check if an address is authorized for a specific participant type
    pub fn is_authorised(
        env: Env,
        address: Address,
        participant_type: ParticipantType,
    ) -> bool {
        let key = DataKey::Participant(address);
        
        if let Some(participant) = env.storage().persistent().get::<DataKey, Participant>(&key) {
            participant.active && participant.participant_type == participant_type
        } else {
            false
        }
    }

    /// Revoke a participant (mark as inactive)
    /// Only callable by admin
    pub fn revoke_participant(env: Env, address: Address) -> Result<(), RegistryError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Participant(address.clone());
        
        let mut participant = env
            .storage()
            .persistent()
            .get::<DataKey, Participant>(&key)
            .ok_or(RegistryError::ParticipantNotFound)?;

        participant.active = false;
        env.storage().persistent().set(&key, &participant);

        // Emit event
        env.events().publish(
            (symbol_short!("rev_part"), address),
            (),
        );

        Ok(())
    }

    /// Get participant details
    pub fn get_participant(env: Env, address: Address) -> Result<Participant, RegistryError> {
        let key = DataKey::Participant(address);
        
        env.storage()
            .persistent()
            .get::<DataKey, Participant>(&key)
            .ok_or(RegistryError::ParticipantNotFound)
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_register_and_authorize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RegistryContract);
        let client = RegistryContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let forwarder = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);
        
        client.register_participant(
            &forwarder,
            &ParticipantType::FreightForwarder,
            &String::from_str(&env, "DHL Freight"),
            &String::from_str(&env, "Kenya"),
        );

        assert!(client.is_authorised(&forwarder, &ParticipantType::FreightForwarder));
        assert!(!client.is_authorised(&forwarder, &ParticipantType::Inspector));
    }

    #[test]
    fn test_revoke_participant() {
        let env = Env::default();
        let contract_id = env.register_contract(None, RegistryContract);
        let client = RegistryContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let inspector = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);
        
        client.register_participant(
            &inspector,
            &ParticipantType::Inspector,
            &String::from_str(&env, "SGS Inspection"),
            &String::from_str(&env, "Nigeria"),
        );

        assert!(client.is_authorised(&inspector, &ParticipantType::Inspector));

        client.revoke_participant(&inspector);

        assert!(!client.is_authorised(&inspector, &ParticipantType::Inspector));
    }
}
