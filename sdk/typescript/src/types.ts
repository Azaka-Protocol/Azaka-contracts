/**
 * Trade status in the LC lifecycle
 */
export enum TradeStatus {
  PendingEscrow = 'PendingEscrow',
  Active = 'Active',
  DocumentsPending = 'DocumentsPending',
  Settled = 'Settled',
  Cancelled = 'Cancelled',
  Expired = 'Expired',
}

/**
 * Document types in trade finance
 */
export enum DocumentType {
  BillOfLading = 'BillOfLading',
  CertificateOfOrigin = 'CertificateOfOrigin',
  InspectionCertificate = 'InspectionCertificate',
  PhytosanitaryCertificate = 'PhytosanitaryCertificate',
  CustomsDeclaration = 'CustomsDeclaration',
}

/**
 * Participant types in the ecosystem
 */
export enum ParticipantType {
  IssuingBank = 'IssuingBank',
  ConfirmingBank = 'ConfirmingBank',
  FreightForwarder = 'FreightForwarder',
  Inspector = 'Inspector',
  PortAuthority = 'PortAuthority',
}

/**
 * Trade record
 */
export interface Trade {
  tradeId: bigint;
  exporter: string;
  importer: string;
  issuingBank: string;
  confirmingBank?: string;
  stablecoinAsset: string;
  amount: bigint;
  requiredDocs: DocumentType[];
  expiryLedger: number;
  status: TradeStatus;
  createdLedger: number;
}

/**
 * Document record
 */
export interface Document {
  tradeId: bigint;
  docType: DocumentType;
  docHash: string;
  metadataUri: string;
  submitter: string;
  signers: string[];
  verified: boolean;
  requiredSignatures: number;
}

/**
 * Participant record
 */
export interface Participant {
  address: string;
  participantType: ParticipantType;
  name: string;
  jurisdiction: string;
  active: boolean;
}

/**
 * Contract IDs configuration
 */
export interface ContractIds {
  trade: string;
  escrow: string;
  document: string;
  registry: string;
}

/**
 * SDK configuration
 */
export interface AzakaConfig {
  network: 'testnet' | 'mainnet' | 'futurenet';
  contractIds: ContractIds;
  rpcUrl?: string;
}

/**
 * Create trade parameters
 */
export interface CreateTradeParams {
  exporter: string;
  importer: string;
  issuingBank: string;
  stablecoinAsset: string;
  amount: bigint;
  requiredDocs: DocumentType[];
  expiryLedger: number;
}

/**
 * Submit document parameters
 */
export interface SubmitDocumentParams {
  tradeId: bigint;
  docType: DocumentType;
  docHash: string;
  metadataUri: string;
}

/**
 * Sign document parameters
 */
export interface SignDocumentParams {
  tradeId: bigint;
  docType: DocumentType;
  signer: string;
}
