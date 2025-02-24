import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';
import { Connection, PublicKey } from '@solana/web3.js';
import IDL from '../../target/idl/contract.json';

export async function getSmartWalletPdaByCreator(
  connection: Connection,
  pubkey: number[]
) {
  const accounts = await connection.getProgramAccounts(
    new PublicKey('3jq9oBWGCUWmBynC8TTBL9KWJdGegsChJ1c8ksybGhum'),
    {
      dataSlice: {
        offset: 8,
        length: 33,
      },
      filters: [
        {
          memcmp: {
            offset: 0,
            bytes: bs58.encode(
              IDL?.accounts.find(
                (acc: { name: string; discriminator: number[] }) =>
                  acc.name === 'SmartWallet'
              )?.discriminator as number[]
            ),
          },
        },
        {
          memcmp: {
            offset: 8,
            bytes: bs58.encode(pubkey),
          },
        },
      ],
    }
  );

  if (accounts.length === 0) {
    throw new Error('Smart wallet not found');
  }

  const account = accounts[0];

  return account.pubkey;
}
