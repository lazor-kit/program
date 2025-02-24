import {
  ComputeBudgetProgram,
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from '@solana/web3.js';
import { createSecp256r1Instruction } from '../utils';
import * as anchor from '@coral-xyz/anchor';
import { Contract } from '../../target/types/contract';
import IDL from '../../target/idl/contract.json';

type CreateVerifyAndExecuteTransactionParams = {
  arbitraryInstruction: TransactionInstruction;
  pubkey: Buffer<ArrayBuffer>;
  signature: Buffer<ArrayBuffer>;
  message: Buffer<ArrayBuffer>;
  connection: Connection;
  payer: PublicKey;
  smartWalletPda: PublicKey;
};

export async function createVerifyAndExecuteTransaction(
  params: CreateVerifyAndExecuteTransactionParams
): Promise<Transaction> {
  const {
    arbitraryInstruction,
    pubkey,
    signature,
    message,
    connection,
    payer,
    smartWalletPda,
  } = params;

  const program = new anchor.Program(IDL as Contract, {
    connection: connection,
  });

  // find signer and set isSigner to false
  let remainingAccounts = arbitraryInstruction.keys.map((key) => {
    return {
      pubkey: key.pubkey,
      isSigner: false,
      isWritable: key.isWritable,
    };
  });

  // // check if the arbitraryInstruction.programId is not in the remainingAccounts
  // if (
  //   !remainingAccounts.find(
  //     (account) =>
  //       account.pubkey.toBase58() === arbitraryInstruction.programId.toBase58()
  //   )
  // )
  //   remainingAccounts.push({
  //     pubkey: arbitraryInstruction.programId,
  //     isSigner: false,
  //     isWritable: false,
  //   });

  const verifySecp256r1Instruction = createSecp256r1Instruction(
    message,
    pubkey,
    signature
  );

  const txn = new Transaction()
    .add(ComputeBudgetProgram.setComputeUnitLimit({ units: 200_000 }))
    .add(verifySecp256r1Instruction)
    .add(
      await program.methods
        .verifyAndExecuteInstruction(
          Array.from(pubkey),
          message,
          Array.from(signature),
          arbitraryInstruction.data
        )
        .accounts({
          smartWallet: smartWalletPda,
          cpiProgram: arbitraryInstruction.programId,
        })
        .remainingAccounts(remainingAccounts)
        .instruction()
    );

  txn.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

  txn.feePayer = payer;

  return txn;
}
