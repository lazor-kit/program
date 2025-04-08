import {
  Connection,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from '@solana/web3.js';
import {
  AddAuthenticatorsParam,
  CreateInitSmartWalletTransactionParam,
  CreateVerifyAndExecuteTransactionParam,
  Message,
  PasskeyPubkey,
  SmartWalletAuthority,
  VerifyParam,
} from './types';
import { createSecp256r1Instruction, getID } from './utils';
import bs58 from 'bs58';
import {
  InitSmartWalletDiscriminator,
  LOOKUP_TABLE_ADDRESS,
  PROGRAM_ID,
  SMART_WALLET_SEED,
  SmartWalletAuthorityDiscriminator,
  SmartWalletDataSeeds,
  VerifyAndExecuteDiscriminator,
} from './constant';
import { sha256 } from 'js-sha256';
import * as anchor from '@coral-xyz/anchor';

export class SmartWalletContract {
  constructor(private readonly connection: Connection) {}

  private lookupTableAddress: PublicKey = LOOKUP_TABLE_ADDRESS;

  get programId(): PublicKey {
    return PROGRAM_ID;
  }

  async getListSmartWalletAuthorityByPasskeyPubkey(
    authority: PasskeyPubkey
  ): Promise<PublicKey[]> {
    const data = await this.connection.getProgramAccounts(this.programId, {
      dataSlice: {
        offset: 8,
        length: 33,
      },
      filters: [
        {
          memcmp: {
            offset: 0,
            bytes: bs58.encode(SmartWalletAuthorityDiscriminator),
          },
        },
        {
          memcmp: {
            offset: 8,
            bytes: bs58.encode(authority.data),
          },
        },
      ],
    });

    if (data.length <= 0) {
      throw new Error('This passkey pubkey does not have any smart-wallet');
    }

    return data.map((item) => item.pubkey);
  }

  async getSmartWalletAuthorityData(
    smartWalletAuthorityPubkey: PublicKey
  ): Promise<SmartWalletAuthority> {
    const accountInfo = await this.connection.getAccountInfo(
      smartWalletAuthorityPubkey
    );

    if (!accountInfo) {
      throw new Error('Account not found');
    }

    const data = accountInfo.data;
    const authorityData = SmartWalletAuthority.deserialize(data);

    if (!authorityData) {
      throw new Error('Failed to deserialize authority data');
    }

    return authorityData;
  }

  async getMessage(smartWalletAuthorityData: SmartWalletAuthority): Promise<{
    message: Message;
    messageBytes: Buffer<ArrayBufferLike>;
  }> {
    const slot = await this.connection.getSlot({ commitment: 'processed' });
    const timestamp = await this.connection.getBlockTime(slot);

    const message: Message = new Message(
      smartWalletAuthorityData.nonce,
      timestamp
    );

    const messageBytes = Message.serialize(message);

    return { message, messageBytes };
  }

  async createInitSmartWalletTransaction(
    param: CreateInitSmartWalletTransactionParam
  ): Promise<Transaction> {
    const { secp256r1PubkeyBytes, payer } = param;

    // check pubkey length
    if (secp256r1PubkeyBytes.length !== 33) {
      throw new Error('Invalid pubkey length');
    }

    const id = getID();

    const [smartWalletPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SMART_WALLET_SEED),
        new anchor.BN(id).toArrayLike(Buffer, 'le', 8),
      ],
      this.programId
    );

    const [smartWalletAuthorityPda] = PublicKey.findProgramAddressSync(
      [this.hashSeeds(secp256r1PubkeyBytes, smartWalletPda)],
      this.programId
    );

    const [smartWalletData] = PublicKey.findProgramAddressSync(
      [SmartWalletDataSeeds, smartWalletPda.toBuffer()],
      this.programId
    );

    const createSmartWalletIns = new TransactionInstruction({
      programId: this.programId,
      keys: [
        {
          pubkey: payer,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: smartWalletPda,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: smartWalletData,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: smartWalletAuthorityPda,
          isSigner: false,
          isWritable: true,
        },

        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ],
      data: Buffer.concat([
        Buffer.from(InitSmartWalletDiscriminator),
        Buffer.from(secp256r1PubkeyBytes),
        new anchor.BN(id).toArrayLike(Buffer, 'le', 8),
      ]),
    });

    const txn = new Transaction().add(createSmartWalletIns);

    txn.feePayer = payer;
    txn.recentBlockhash = (
      await this.connection.getLatestBlockhash()
    ).blockhash;

    return txn;
  }

  async createVerifyAndExecuteTransaction(
    params: CreateVerifyAndExecuteTransactionParam
  ): Promise<VersionedTransaction> {
    const {
      arbitraryInstruction,
      pubkey,
      signature,
      message,
      payer,
      smartWalletAuthority,
      smartWalletPubkey,
    } = params;

    // find signer and set isSigner to false
    let remainingAccounts = arbitraryInstruction.keys.map((key) => {
      return {
        pubkey: key.pubkey,
        isSigner: false,
        isWritable: key.isWritable,
      };
    });

    const messageBytes = Message.serialize(message);

    const verifySecp256r1Instruction = createSecp256r1Instruction(
      messageBytes,
      pubkey,
      signature
    );

    const passkeyPubkey = new PasskeyPubkey(Array.from(pubkey));

    const verifyParam: VerifyParam = {
      pubkey: passkeyPubkey,
      msg: message,
      sig: signature,
    };

    const [smartWalletData] = PublicKey.findProgramAddressSync(
      [SmartWalletDataSeeds, smartWalletPubkey.toBuffer()],
      this.programId
    );

    const executeInstruction = new TransactionInstruction({
      programId: this.programId,
      keys: [
        {
          pubkey: SYSVAR_INSTRUCTIONS_PUBKEY,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: smartWalletPubkey,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: smartWalletData,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: smartWalletAuthority,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: arbitraryInstruction.programId,
          isSigner: false,
          isWritable: false,
        },

        ...remainingAccounts,
        {
          pubkey: payer,
          isSigner: true,
          isWritable: true,
        },
      ],

      data: Buffer.concat([
        Buffer.from(VerifyAndExecuteDiscriminator),
        VerifyParam.serialize(verifyParam),
        arbitraryInstruction.data,
      ]),
    });

    const blockhash = (await this.connection.getLatestBlockhash()).blockhash;

    const lookupTableAccount = (
      await this.connection.getAddressLookupTable(this.lookupTableAddress)
    ).value;

    const messageV0 = new TransactionMessage({
      payerKey: payer,
      recentBlockhash: blockhash,
      instructions: [verifySecp256r1Instruction, executeInstruction], // note this is an array of instructions
    }).compileToV0Message([lookupTableAccount]);

    const transactionV0 = new VersionedTransaction(messageV0);

    return transactionV0;
  }

  // async addAuthenticatorsTxn(
  //   param: AddAuthenticatorsParam
  // ): Promise<VersionedTransaction> {
  //   const {
  //     pubkey,
  //     signature,
  //     message,
  //     payer,
  //     newPasskey,
  //     smartWalletPubkey,
  //     smartWalletAuthority,
  //   } = param;

  //   const messageBytes = this.program.coder.types.encode('message', message);

  //   const verifySecp256r1Instruction = createSecp256r1Instruction(
  //     messageBytes,
  //     pubkey,
  //     signature
  //   );

  //   const verifyParam: VerifyParam = {
  //     pubkey: { data: Array.from(pubkey) },
  //     msg: message,
  //     sig: Array.from(signature),
  //   };

  //   const [newSmartWalletAuthorityPda] = PublicKey.findProgramAddressSync(
  //     [this.hashSeeds(Array.from(newPasskey.data), smartWalletPubkey)],
  //     this.programId
  //   );

  //   const addAuthIns = await this.program.methods
  //     .addAuthenticator(verifyParam, newPasskey)
  //     .accountsPartial({
  //       payer,
  //       smartWallet: smartWalletPubkey,
  //       smartWalletAuthority,
  //       newWalletAuthority: newSmartWalletAuthorityPda,
  //     })
  //     .instruction();

  //   const blockhash = (await this.connection.getLatestBlockhash()).blockhash;

  //   const lookupTableAccount = (
  //     await this.connection.getAddressLookupTable(this.lookupTableAddress)
  //   ).value;

  //   const messageV0 = new TransactionMessage({
  //     payerKey: payer,
  //     recentBlockhash: blockhash,
  //     instructions: [verifySecp256r1Instruction, addAuthIns], // note this is an array of instructions
  //   }).compileToV0Message([lookupTableAccount]);

  //   const transactionV0 = new VersionedTransaction(messageV0);

  //   return transactionV0;
  // }

  async setLookupTableAddress(lookupTableAddress: PublicKey) {
    this.lookupTableAddress = lookupTableAddress;
  }

  // hash with crypto
  hashSeeds(passkey: number[], smartWallet: PublicKey): Buffer {
    const rawBuffer = Buffer.concat([
      Buffer.from(passkey),
      smartWallet.toBuffer(),
    ]);
    const hash = sha256.arrayBuffer(rawBuffer);
    return Buffer.from(hash).subarray(0, 32);
  }
}
