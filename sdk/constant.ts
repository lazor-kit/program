import { PublicKey } from '@solana/web3.js';
import { Buffer } from 'buffer';

export const SMART_WALLET_SEED = Buffer.from('smart_wallet');

export const PROGRAM_ID: PublicKey = new PublicKey(
  '3jq9oBWGCUWmBynC8TTBL9KWJdGegsChJ1c8ksybGhum'
);

export const SECP256R1_NATIVE_PROGRAM: PublicKey = new PublicKey(
  'Secp256r1SigVerify1111111111111111111111111'
);

export const LOOKUP_TABLE_ADDRESS: PublicKey = new PublicKey(
  'AhUtjWCVWJZkF4XjhVo7Y2TK2m6RiX3ritwEyxKKnL19'
); // https://rpc.lazorkit.xyz

export const SmartWalletAuthorityDiscriminator: number[] = [
  164, 179, 94, 28, 254, 200, 86, 148,
];

export const SmartWalletDataSeeds: Buffer<ArrayBuffer> =
  Buffer.from('smart_wallet_data');

export const InitSmartWalletDiscriminator: number[] = [
  229, 38, 158, 24, 6, 73, 94, 101,
];

export const VerifyAndExecuteDiscriminator: number[] = [
  48, 18, 40, 40, 75, 74, 147, 110,
];
