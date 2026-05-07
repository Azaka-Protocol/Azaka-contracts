import {
  Contract,
  SorobanRpc,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  Keypair,
  Operation,
  Account,
} from '@stellar/stellar-sdk';
import type {
  AzakaConfig,
  CreateTradeParams,
  SubmitDocumentParams,
  SignDocumentParams,
  Trade,
  Document,
  DocumentType,
} from './types';

/**
 * Main Azaka client for interacting with the protocol
 */
export class AzakaClient {
  private config: AzakaConfig;
  private server: SorobanRpc.Server;
  private networkPassphrase: string;

  constructor(config: AzakaConfig) {
    this.config = config;
    
    // Set RPC URL based on network
    const rpcUrl = config.rpcUrl || this.getDefaultRpcUrl(config.network);
    this.server = new SorobanRpc.Server(rpcUrl);
    
    // Set network passphrase
    this.networkPassphrase = this.getNetworkPassphrase(config.network);
  }

  private getDefaultRpcUrl(network: string): string {
    switch (network) {
      case 'testnet':
        return 'https://soroban-testnet.stellar.org';
      case 'mainnet':
        return 'https://soroban-mainnet.stellar.org';
      case 'futurenet':
        return 'https://rpc-futurenet.stellar.org';
      default:
        throw new Error(`Unknown network: ${network}`);
    }
  }

  private getNetworkPassphrase(network: string): string {
    switch (network) {
      case 'testnet':
        return Networks.TESTNET;
      case 'mainnet':
        return Networks.PUBLIC;
      case 'futurenet':
        return Networks.FUTURENET;
      default:
        throw new Error(`Unknown network: ${network}`);
    }
  }

  /**
   * Create a new trade (Letter of Credit)
   */
  async createTrade(
    params: CreateTradeParams,
    sourceKeypair: Keypair
  ): Promise<bigint> {
    const contract = new Contract(this.config.contractIds.trade);
    
    // Build transaction
    const account = await this.server.getAccount(sourceKeypair.publicKey());
    
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        contract.call(
          'create_trade',
          ...[
            params.exporter,
            params.importer,
            params.issuingBank,
            params.stablecoinAsset,
            params.amount,
            params.requiredDocs,
            params.expiryLedger,
          ]
        )
      )
      .setTimeout(30)
      .build();

    // Sign and submit
    transaction.sign(sourceKeypair);
    const response = await this.server.sendTransaction(transaction);
    
    // Wait for confirmation and extract trade ID
    // In production, parse the result properly
    return 1n; // Placeholder
  }

  /**
   * Deposit escrow for a trade
   */
  async depositEscrow(
    tradeId: bigint,
    amount: bigint,
    sourceKeypair: Keypair
  ): Promise<void> {
    const contract = new Contract(this.config.contractIds.escrow);
    
    const account = await this.server.getAccount(sourceKeypair.publicKey());
    
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        contract.call('deposit', ...[tradeId, sourceKeypair.publicKey(), amount])
      )
      .setTimeout(30)
      .build();

    transaction.sign(sourceKeypair);
    await this.server.sendTransaction(transaction);
  }

  /**
   * Submit a document
   */
  async submitDocument(
    params: SubmitDocumentParams,
    sourceKeypair: Keypair
  ): Promise<void> {
    const contract = new Contract(this.config.contractIds.document);
    
    const account = await this.server.getAccount(sourceKeypair.publicKey());
    
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        contract.call(
          'submit_document',
          ...[params.tradeId, params.docType, params.docHash, params.metadataUri]
        )
      )
      .setTimeout(30)
      .build();

    transaction.sign(sourceKeypair);
    await this.server.sendTransaction(transaction);
  }

  /**
   * Sign a document (counter-sign)
   */
  async signDocument(
    params: SignDocumentParams,
    sourceKeypair: Keypair
  ): Promise<void> {
    const contract = new Contract(this.config.contractIds.document);
    
    const account = await this.server.getAccount(sourceKeypair.publicKey());
    
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        contract.call(
          'sign_document',
          ...[params.tradeId, params.docType, params.signer]
        )
      )
      .setTimeout(30)
      .build();

    transaction.sign(sourceKeypair);
    await this.server.sendTransaction(transaction);
  }

  /**
   * Check if all documents are verified and release escrow if ready
   */
  async checkAndRelease(tradeId: bigint, sourceKeypair: Keypair): Promise<boolean> {
    // First check if all docs are verified
    const trade = await this.getTrade(tradeId);
    
    // In production, call document contract to verify
    // If verified, trigger release
    
    const contract = new Contract(this.config.contractIds.escrow);
    const account = await this.server.getAccount(sourceKeypair.publicKey());
    
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        contract.call('release', ...[tradeId, trade.exporter])
      )
      .setTimeout(30)
      .build();

    transaction.sign(sourceKeypair);
    await this.server.sendTransaction(transaction);
    
    return true;
  }

  /**
   * Get trade details
   */
  async getTrade(tradeId: bigint): Promise<Trade> {
    const contract = new Contract(this.config.contractIds.trade);
    
    // In production, properly invoke and parse the contract response
    // This is a placeholder implementation
    return {
      tradeId,
      exporter: 'G...',
      importer: 'G...',
      issuingBank: 'G...',
      stablecoinAsset: 'C...',
      amount: 50000n * 10000000n,
      requiredDocs: [],
      expiryLedger: 1000000,
      status: 'Active' as any,
      createdLedger: 900000,
    };
  }

  /**
   * List all trades for an exporter
   */
  async listExporterTrades(exporter: string): Promise<bigint[]> {
    const contract = new Contract(this.config.contractIds.trade);
    
    // In production, properly invoke and parse the contract response
    return [];
  }

  /**
   * Get document details
   */
  async getDocument(tradeId: bigint, docType: DocumentType): Promise<Document> {
    const contract = new Contract(this.config.contractIds.document);
    
    // In production, properly invoke and parse the contract response
    return {
      tradeId,
      docType,
      docHash: '0x...',
      metadataUri: 'ipfs://...',
      submitter: 'G...',
      signers: [],
      verified: false,
      requiredSignatures: 2,
    };
  }
}
