import { Program } from '@coral-xyz/anchor';
import { Connection, PublicKey } from '@solana/web3.js';

export interface BirthdayEvent {
  id: string;
  name: string;
  date: number;
  coming: number;
  busy: number;
  totalComments: number;
  address: string;
  creator: string;
}

export interface Comment {
  id: number;
  author: string;
  text: string;
  timestamp: number;
}

export interface RSVPStatus {
  status: 'none' | 'coming' | 'busy';
  timestamp?: number;
}

export interface Wallet {
  publicKey: PublicKey;
  signTransaction: (tx: any) => Promise<any>;
  signAllTransactions: (txs: any[]) => Promise<any[]>;
}

export interface ProgramContextType {
  program: Program | null;
  wallet: Wallet | null;
  connection: Connection | null;
  loading: boolean;
  error: string | null;
}