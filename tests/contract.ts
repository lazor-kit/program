import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Contract } from '../target/types/contract';
import ECDSA from 'ecdsa-secp256r1';
import { Keypair } from '@solana/web3.js';
import dotenv from 'dotenv';
import bs58 from 'bs58';
import { createInitSmartWalletTransaction } from '../script/api/init';
import { getSmartWalletPdaByCreator } from '../script/api/getSmartWalletPda';
import { createVerifyAndExecuteTransaction } from '../script/api/verifyAndExecute';
import { setup } from './raydium-swap/swap';
dotenv.config();

describe('contract', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const anchorProvider = anchor.getProvider() as anchor.AnchorProvider;

  const wallet = Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY!));

  xit('Init smart-wallet', async () => {
    const privateKey = ECDSA.generateKey();

     

    const publicKeyBase64 = privateKey.toCompressedPublicKey();

    const pubkey = Array.from(Buffer.from(publicKeyBase64, 'base64'));

    const txn = await createInitSmartWalletTransaction({
      secp256k1PubkeyBytes: pubkey,
      connection: anchorProvider.connection,
      payer: wallet.publicKey,
    });

    const sig = await anchorProvider.sendAndConfirm(txn, [wallet]);

    console.log('Init smart-wallet', sig);
  });

  it('Verify and execute transfer token instruction', async () => {
    // create smart wallet
    const privateKey = ECDSA.generateKey();
    const publicKeyBase64 = privateKey.toCompressedPublicKey();
    const pubkey = Buffer.from(publicKeyBase64, 'base64');

    await anchorProvider.sendAndConfirm(
      await createInitSmartWalletTransaction({
        secp256k1PubkeyBytes: Array.from(pubkey),
        connection: anchorProvider.connection,
        payer: wallet.publicKey,
      }),
      [wallet]
    );

    const smartWalletPubkey = await getSmartWalletPdaByCreator(
      anchorProvider.connection,
      Array.from(pubkey)
    );

    console.log('Smart wallet pubkey', smartWalletPubkey.toBase58());

    const swapIns = await setup({
      smartWalletPubkey,
      wallet,
      anchorProvider,
    });

    const message = 'Hello';

    const messageBytes = Buffer.from(message);

    const signatureBase64 = privateKey.sign(Buffer.from(message).toString());

    let signature = Buffer.from(signatureBase64, 'base64');

    const txn = await createVerifyAndExecuteTransaction({
      arbitraryInstruction: swapIns,
      pubkey: pubkey,
      signature: signature,
      message: messageBytes,
      connection: anchorProvider.connection,
      payer: wallet.publicKey,
      smartWalletPda: smartWalletPubkey,
    });

    txn.partialSign(wallet);

    // sleep for 5 seconds
    await new Promise((resolve) => setTimeout(resolve, 5000));

    const sig = await anchorProvider.connection.sendRawTransaction(
      txn.serialize()
    );

    console.log('Verify and execute transfer token instruction', sig);
  });
});
