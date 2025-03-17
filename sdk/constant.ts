import { PublicKey } from '@solana/web3.js';

export const SMART_WALLET_SEED = Buffer.from('smart_wallet');

export const SECP256R1_NATIVE_PROGRAM = new PublicKey(
  'Secp256r1SigVerify1111111111111111111111111'
);

export const LOOKUP_TABLE_ADDRESS = new PublicKey(
  '6GxBfXPQxVV17tdvpXLD7uz2tGqyEhRYYVWw8rKHMFw1'
); // https://rpc.lazorkit.xyz
