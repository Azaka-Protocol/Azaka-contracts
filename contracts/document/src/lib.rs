#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, BytesN, Env, String, Vec, Symbol, symbol_short,
};

/// Document type in trade finance
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocumentType {
    BillOfLading,
    CertificateOfOrigin,
    InspectionCertificate,
    PhytosanitaryCertificate,
    CustomsDeclaration,
}

/// Document record with hash, metadata, and signatures
#[contracttype]
#[derive(Clone, Debug)]
pub struct Document {
    pub trade_id: u64,
    pub doc_type: DocumentType,
    pub doc_hash: BytesN<32>,
    pub metadata_uri: String,
    pub submitter: Address,
    pub signers: Vec<Address>,
    pub verified: bool,
    pub required_signatures: u32,
}

/// Storage keys
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Document(u64, DocumentType), // (trade_id, doc_type)
    RegistryContract,
    RequiredSignatures(DocumentType),
}

/// Document contract errors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocumentError {
    NotAuthorized = 1,
    DocumentNotFound = 2,
    DocumentAlreadyExists = 3,
    AlreadySigned = 4,
    InvalidSignatureCount = 5,
}

#[contract]
pub struct DocumentContract;

#[contractimpl]
impl DocumentContract {
    /// Initialize the document contract with registry contract address
    pub fn initialize(env: Env, registry_contract: Address) {
        if env.storage().instance().has(&DataKey::RegistryContract) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::RegistryContract, &registry_contract);
        
        // Set default required signatures for each document type
        env.storage().instance().set(&DataKey::RequiredSignatures(DocumentType::BillOfLading), &2u32);
        env.storage().instance().set(&DataKey::RequiredSignatures(DocumentType::CertificateOfOrigin), &2u32);
        env.storage().instance().set(&DataKey::RequiredSignatures(DocumentType::InspectionCertificate), &2u32);
        env.storage().instance().set(&DataKey::RequiredSignatures(DocumentType::PhytosanitaryCertificate), &2u32);
        env.storage().instance().set(&DataKey::RequiredSignatures(DocumentType::CustomsDeclaration), &2u32);
    }

    /// Submit a document hash and metadata
    /// Callable by authorized participants only
    pub fn submit_document(
        env: Env,
        trade_id: u64,
        doc_type: DocumentType,
        doc_hash: BytesN<32>,
        metadata_uri: String,
    ) -> Result<(), DocumentError> {
        let submitter = env.current_contract_address();
        
        // TODO: Implement authorization check via registry contract
        // let registry_contract: Address = env.storage().instance().get(&DataKey::RegistryContract).unwrap();
        // let authorized = registry_contract.is_authorized(submitter, get_required_participant_type(doc_type));
        // if !authorized {
        //     return Err(DocumentError::NotAuthorized);
        // }
        
        let key = DataKey::Document(trade_id, doc_type.clone());
        
        if env.storage().persistent().has(&key) {
            return Err(DocumentError::DocumentAlreadyExists);
        }

        let required_sigs: u32 = env
            .storage()
            .instance()
            .get(&DataKey::RequiredSignatures(doc_type.clone()))
            .unwrap_or(2);

        let mut signers = Vec::new(&env);
        signers.push_back(submitter.clone());

        let document = Document {
            trade_id,
            doc_type: doc_type.clone(),
            doc_hash: doc_hash.clone(),
            metadata_uri: metadata_uri.clone(),
            submitter: submitter.clone(),
            signers,
            verified: false,
            required_signatures: required_sigs,
        };

        env.storage().persistent().set(&key, &document);

        // Emit event
        env.events().publish(
            (symbol_short!("sub_doc"), trade_id, doc_type),
            (submitter, doc_hash, metadata_uri),
        );

        // TODO: Notify trade contract of document submission
        // This would trigger status updates in the trade lifecycle

        Ok(())
    }

    /// Sign a document (counter-sign)
    /// Callable by authorized participants only
    pub fn sign_document(
        env: Env,
        trade_id: u64,
        doc_type: DocumentType,
        signer: Address,
    ) -> Result<(), DocumentError> {
        signer.require_auth();

        let key = DataKey::Document(trade_id, doc_type.clone());
        
        let mut document = env
            .storage()
            .persistent()
            .get::<DataKey, Document>(&key)
            .ok_or(DocumentError::DocumentNotFound)?;

        // Check if already signed by this address
        for existing_signer in document.signers.iter() {
            if existing_signer == signer {
                return Err(DocumentError::AlreadySigned);
            }
        }

        document.signers.push_back(signer.clone());

        // Check if we have enough signatures
        if document.signers.len() >= document.required_signatures {
            document.verified = true;
        }

        env.storage().persistent().set(&key, &document);

        // Emit event
        env.events().publish(
            (symbol_short!("sign_doc"), trade_id, doc_type.clone()),
            signer.clone(),
        );

        if document.verified {
            env.events().publish(
                (symbol_short!("ver_doc"), trade_id, doc_type),
                (),
            );
        }

        Ok(())
    }

    /// Verify if a document has all required signatures
    pub fn verify_document(env: Env, trade_id: u64, doc_type: DocumentType) -> bool {
        let key = DataKey::Document(trade_id, doc_type);
        
        if let Some(document) = env.storage().persistent().get::<DataKey, Document>(&key) {
            document.verified
        } else {
            false
        }
    }

    /// Get document details
    pub fn get_document(
        env: Env,
        trade_id: u64,
        doc_type: DocumentType,
    ) -> Result<Document, DocumentError> {
        let key = DataKey::Document(trade_id, doc_type);
        
        env.storage()
            .persistent()
            .get::<DataKey, Document>(&key)
            .ok_or(DocumentError::DocumentNotFound)
    }

    /// Check if all required documents for a trade are verified
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
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String, BytesN};

    #[test]
    fn test_submit_and_sign_document() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DocumentContract);
        let client = DocumentContractClient::new(&env, &contract_id);

        let registry = Address::generate(&env);
        let submitter = Address::generate(&env);
        let signer = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&registry);

        let doc_hash = BytesN::from_array(&env, &[1u8; 32]);
        
        client.submit_document(
            &1,
            &DocumentType::BillOfLading,
            &doc_hash,
            &String::from_str(&env, "ipfs://QmTest123"),
        );

        assert!(!client.verify_document(&1, &DocumentType::BillOfLading));

        client.sign_document(&1, &DocumentType::BillOfLading, &signer);

        assert!(client.verify_document(&1, &DocumentType::BillOfLading));
    }

    #[test]
    fn test_all_docs_verified() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DocumentContract);
        let client = DocumentContractClient::new(&env, &contract_id);

        let registry = Address::generate(&env);
        let signer = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&registry);

        let doc_hash = BytesN::from_array(&env, &[1u8; 32]);
        
        // Submit two documents
        client.submit_document(
            &1,
            &DocumentType::BillOfLading,
            &doc_hash,
            &String::from_str(&env, "ipfs://QmTest1"),
        );
        
        client.submit_document(
            &1,
            &DocumentType::InspectionCertificate,
            &doc_hash,
            &String::from_str(&env, "ipfs://QmTest2"),
        );

        let mut required_docs = Vec::new(&env);
        required_docs.push_back(DocumentType::BillOfLading);
        required_docs.push_back(DocumentType::InspectionCertificate);

        // Not all verified yet
        assert!(!client.all_docs_verified(&1, &required_docs));

        // Sign both
        client.sign_document(&1, &DocumentType::BillOfLading, &signer);
        client.sign_document(&1, &DocumentType::InspectionCertificate, &signer);

        // Now all verified
        assert!(client.all_docs_verified(&1, &required_docs));
    }
}
