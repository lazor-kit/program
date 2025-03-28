import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';
import { Connection, Keypair, VersionedTransaction } from '@solana/web3.js';

export async function signAndSendTxn({
  base58EncodedTransaction,
  relayerUrl,
}: {
  base58EncodedTransaction: string;
  relayerUrl: string;
}) {
  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'signAndSendTransaction',
    params: [base58EncodedTransaction],
  };

  try {
    const response = await fetch(relayerUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    const data = await response.json();
    console.log('Response:', data);
    return data;
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}

export async function signTransaction({
  base58EncodedTransaction,
  relayerUrl,
}: {
  base58EncodedTransaction: string;
  relayerUrl: string;
}) {
  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'signTransaction',
    params: [base58EncodedTransaction],
  };

  try {
    const response = await fetch(relayerUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    const data = await response.json();
    console.log('Response:', data);
    return data;
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}

export async function signAndSendVersionTxn({
  base58EncodedTransaction,
  relayerUrl,
}: {
  base58EncodedTransaction: string;
  relayerUrl: string;
}) {
  const payload = {
    jsonrpc: '2.0',
    id: 1,
    method: 'signAndSendVersionedTransaction',
    params: [base58EncodedTransaction],
  };

  try {
    const response = await fetch(relayerUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(payload),
    });

    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error:', error);
    throw error;
  }
}
