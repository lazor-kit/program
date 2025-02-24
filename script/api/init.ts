import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { Contract } from '../../target/types/contract';
import IDL from '../../target/idl/contract.json';
import { getID } from '../utils';

export type CreateInitSmartWalletTransactionParam = {
  secp256k1PubkeyBytes: number[];
  connection: Connection;
  payer: PublicKey;
};

export async function createInitSmartWalletTransaction(
  param: CreateInitSmartWalletTransactionParam
): Promise<Transaction> {
  const { secp256k1PubkeyBytes, connection, payer } = param;

  // check pubkey length
  if (secp256k1PubkeyBytes.length !== 33) {
    throw new Error('Invalid pubkey length');
  }

  const program = new anchor.Program(IDL as Contract, {
    connection: connection,
  });

  const id = new anchor.BN(getID());

  const createSmartWalletIns = await program.methods
    .initSmartWallet(secp256k1PubkeyBytes, id)
    .accounts({
      signer: payer,
    })
    .instruction();

  const txn = new Transaction().add(createSmartWalletIns);

  txn.feePayer = payer;
  txn.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  return txn;
}
