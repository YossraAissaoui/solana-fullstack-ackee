// src/lib/program.ts

import { Program, AnchorProvider, setProvider } from '@coral-xyz/anchor';
import { Connection, PublicKey } from '@solana/web3.js';
import { PROGRAM_ID } from './constants';
import idl from '@/idl/birthday_invite.json';

const commitment = 'confirmed' as const;

interface Wallet {
  publicKey: PublicKey;
  signTransaction: (tx: any) => Promise<any>;
  signAllTransactions: (txs: any[]) => Promise<any[]>;
}

/**
 * Main initialization - Use this for your Birthday Invite app
 */
export function initializeProgram(
  connection: Connection,
  wallet: Wallet
): Program {
  if (!wallet?.publicKey) {
    throw new Error('Wallet not connected');
  }

  const provider = new AnchorProvider(connection, wallet, {
    commitment,
    preflightCommitment: commitment,
  });

  setProvider(provider);

  // Pass provider, not just connection
  return new Program(idl as unknown as any, provider);
}