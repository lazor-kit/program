import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import * as borsh from 'borsh';
import { Buffer } from 'buffer';

export class PasskeyPubkey {
  data: number[];
  constructor(data: number[]) {
    this.data = data;
  }
  static schema = {
    struct: {
      data: { array: { type: 'u8', len: 33 } },
    },
  };

  static deserialize(buffer?: Buffer): PasskeyPubkey | null {
    if (!buffer) {
      return null;
    }
    try {
      const data: any = borsh.deserialize(this.schema, buffer);
      return new PasskeyPubkey(data.data);
    } catch (error) {
      console.error('Deserialization error:', error);
      console.error('Buffer length:', buffer.length);
      console.error('Buffer data:', buffer.toString('hex'));
      return null;
    }
  }
  static serialize(passkeyPubkey: PasskeyPubkey): Buffer {
    return Buffer.from(
      borsh.serialize(this.schema, { data: passkeyPubkey.data })
    );
  }
}

export class Message {
  nonce: bigint;
  timestamp: bigint;

  constructor(nonce: number | bigint, timestamp: number | bigint) {
    this.nonce = BigInt(nonce);
    this.timestamp = BigInt(timestamp);
  }

  static schema = {
    struct: {
      nonce: 'u64',
      timestamp: 'i64',
    },
  };

  static deserialize(buffer?: Buffer): Message | null {
    if (!buffer) {
      return null;
    }

    try {
      const data: any = borsh.deserialize(this.schema, buffer);
      return new Message(data.nonce, data.timestamp);
    } catch (error) {
      console.error('Deserialization error:', error);
      console.error('Buffer length:', buffer.length);
      console.error('Buffer data:', buffer.toString('hex'));
      return null;
    }
  }

  static serialize(message: Message): Buffer {
    return Buffer.from(borsh.serialize(this.schema, message));
  }
}

export class SmartWalletAuthority {
  nonce: bigint;
  passkeyPubkey: PasskeyPubkey;
  smartWalletPubkey: PublicKey;

  constructor(
    nonce: number | bigint,
    passkeyPubkey: PasskeyPubkey,
    smartWalletPubkey: PublicKey
  ) {
    this.nonce = BigInt(nonce);
    this.passkeyPubkey = passkeyPubkey;
    this.smartWalletPubkey = smartWalletPubkey;
  }

  static schema = {
    struct: {
      nonce: 'u64',
      passkeyPubkey: PasskeyPubkey.schema,
      smartWalletPubkey: { array: { type: 'u8', len: 32 } }, // PublicKey is 32 bytes
    },
  };

  static deserialize(buffer?: Buffer): SmartWalletAuthority | null {
    if (!buffer) {
      return null;
    }
    try {
      const decoded: any = borsh.deserialize(this.schema, buffer);
      return new SmartWalletAuthority(
        decoded.nonce,
        new PasskeyPubkey(decoded.passkeyPubkey.data),
        new PublicKey(decoded.smartWalletPubkey)
      );
    } catch (error) {
      console.error('Deserialization error:', error);
      console.error('Buffer length:', buffer.length);
      console.error('Buffer data:', buffer.toString('hex'));
      return null;
    }
  }
}

export class VerifyParam {
  msg: Message;
  sig: Buffer<ArrayBuffer>;
  pubkey: PasskeyPubkey;

  constructor(msg: Message, sig: Buffer<ArrayBuffer>, pubkey: PasskeyPubkey) {
    this.msg = msg;
    this.sig = sig;
    this.pubkey = pubkey;
  }

  static schema = {
    struct: {
      msg: Message.schema,
      sig: { array: { type: 'u8', len: 64 } },
      pubkey: PasskeyPubkey.schema,
    },
  };

  static serialize(verifyParam: VerifyParam): Buffer {
    return Buffer.from(
      borsh.serialize(this.schema, {
        msg: {
          nonce: verifyParam.msg.nonce,
          timestamp: verifyParam.msg.timestamp,
        },
        sig: verifyParam.sig,
        pubkey: { data: verifyParam.pubkey.data },
      })
    );
  }
}

export type CreateVerifyAndExecuteTransactionParam = {
  arbitraryInstruction: TransactionInstruction;
  pubkey: Buffer<ArrayBuffer>;
  signature: Buffer<ArrayBuffer>;
  message: Message;
  payer: PublicKey;
  smartWalletPubkey: PublicKey;
  smartWalletAuthority: PublicKey;
};

export type CreateInitSmartWalletTransactionParam = {
  secp256r1PubkeyBytes: number[];
  payer: PublicKey;
};

export type AddAuthenticatorsParam = {
  pubkey: Buffer<ArrayBuffer>;
  signature: Buffer<ArrayBuffer>;
  message: Message;
  payer: PublicKey;
  newPasskey: PasskeyPubkey;
  smartWalletPubkey: PublicKey;
  smartWalletAuthority: PublicKey;
};
