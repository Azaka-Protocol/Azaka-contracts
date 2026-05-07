import { Keypair } from '@stellar/stellar-sdk';
import { AzakaClient } from './client';
import type { AzakaConfig, CreateTradeParams, Trade } from './types';

/**
 * Specialized client for bank operations
 * Provides higher-level abstractions for issuing and confirming banks
 */
export class BankClient {
  private client: AzakaClient;
  private bankKeypair: Keypair;

  constructor(config: AzakaConfig, bankKeypair: Keypair) {
    this.client = new AzakaClient(config);
    this.bankKeypair = bankKeypair;
  }

  /**
   * Issue a Letter of Credit (create trade)
   * Called by issuing bank on behalf of importer
   */
  async issueLc(params: CreateTradeParams): Promise<bigint> {
    return await this.client.createTrade(params, this.bankKeypair);
  }

  /**
   * Confirm a Letter of Credit
   * Called by confirming bank (exporter's local bank)
   */
  async confirmLc(tradeId: bigint): Promise<void> {
    // In production, this would call the trade contract's confirm_trade method
    console.log(`Confirming LC for trade ${tradeId}`);
  }

  /**
   * Query all trades in the bank's trade book
   * Returns trades where this bank is either issuing or confirming bank
   */
  async queryTradeBook(): Promise<Trade[]> {
    // In production, this would query the trade contract
    // and filter by bank address
    return [];
  }

  /**
   * Get trade status for monitoring
   */
  async getTradeStatus(tradeId: bigint): Promise<Trade> {
    return await this.client.getTrade(tradeId);
  }

  /**
   * Monitor document submission progress
   */
  async getDocumentProgress(tradeId: bigint): Promise<{
    submitted: number;
    required: number;
    verified: number;
  }> {
    const trade = await this.client.getTrade(tradeId);
    
    // In production, query document contract for each required doc
    return {
      submitted: 0,
      required: trade.requiredDocs.length,
      verified: 0,
    };
  }
}
