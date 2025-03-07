# Smart-wallet with Secp256r1 Signature Verification

## Overview

This document explains how to create and use a smart-wallet with Secp256r1 signature verification to execute arbitrary instructions (like Raydium swaps) on Solana. The process is explained in two main flows: smart-wallet creation and instruction execution.

## Smart-wallet Creation Flow

### Off-chain Steps

1. Generate ECDSA keypair using `ecdsa-secp256r1`:

```typescript
const privateKey = ECDSA.generateKey();
const publicKeyBase64 = privateKey.toCompressedPublicKey();
const pubkey = Array.from(Buffer.from(publicKeyBase64, 'base64'));
```

2. Prepare initialization transaction by finding smart-wallet PDA and required accounts:

```typescript
const txn = await createInitSmartWalletTransaction({
  secp256k1PubkeyBytes: pubkey,
  connection: anchorProvider.connection,
  payer: wallet.publicKey,
});
```

### On-chain Steps

1. Smart contract validates initialization parameters
2. Creates smart-wallet account (PDA) with:
   - Owner field set to ECDSA compressed public key [u8; 33]
   - Authority field set to program ID
   - State initialized as active

## Instruction Execution Flow (e.g., Raydium Swap)

### Off-chain Preparation

1. Prepare the arbitrary instruction (e.g., Raydium swap):

```typescript
const swapInstruction = await prepareSwapInstruction({
  fromTokenAccount,
  toTokenAccount,
  amount,
  // ...other swap parameters
});
```

2. Sign message with ECDSA private key:

```typescript
const message = Buffer.from(swapInstruction.data);
const signatureBase64 = privateKey.sign(message);
const signature = Buffer.from(signatureBase64, 'base64');
```

3. Prepare verification and execution transaction:

```typescript
const transaction = await createVerifyAndExecuteTransaction({
  arbitraryInstruction: swapInstruction,
  pubkey: pubkey,
  signature: signature,
  message: message,
  connection: anchorProvider.connection,
  payer: wallet.publicKey,
  smartWalletPda: smartWalletPubkey,
});
```

### On-chain Verification and Execution

1. Smart contract receives:

   - Arbitrary instruction data
   - ECDSA signature
   - Message (instruction data)
   - ECDSA public key

2. Native Secp256r1 program verifies signature:

   - Validates signature against provided message
   - Confirms signer matches smart-wallet owner's public key

3. Smart contract processes instruction:

   - Finds smart-wallet PDA in instruction's account_metas
   - Verifies PDA matches derived address
   - Sets `is_signer` to true for smart-wallet PDA

4. Smart contract executes instruction:
   - Uses CPI (Cross-Program Invocation) to execute modified instruction
   - Smart-wallet PDA acts as signer
   - Original instruction executes with proper authorization

### Final Off-chain Step

Send transaction to network:

```typescript
const signature = await anchorProvider.connection.sendTransaction(transaction);
await anchorProvider.connection.confirmTransaction(signature);
```

## Technical Details

### Smart-wallet Account Structure

```rust
#[account]
pub struct SmartWallet {
    pub owner: [u8; 33],      // ECDSA compressed public key
    #[max_len(5)]
    pub authority: Vec<[u8; 33]>, // Vec of another ECDSA public key

    pub id: u64, // Unique identifier

    pub bump: u8, // PDA bump seed
}
```

### PDA Derivation

```typescript
const [smartWalletPda] = PublicKey.findProgramAddressSync(
  [Buffer.from('smart-wallet'), &id.to_le_bytes()],
  programId
);
```

### Signature Verification Process

1. Smart contract calls Secp256r1 program with:
   - Message (instruction data)
   - Signature
   - Public key
2. Native program returns verification result
3. Smart contract validates verification before proceeding

## Security Considerations

1. Smart-wallet ownership is proven through ECDSA signatures
2. Each instruction must be signed fresh - no replay protection needed
3. Smart-wallet PDA can only sign when signature is verified
4. Original instruction's integrity is maintained while adding authorization

## Error Handling

Common error cases:

- Invalid signature
- Incorrect public key format
- Smart-wallet not initialized
- Unauthorized instruction attempt
- Invalid PDA derivation

Each error returns a specific error code for proper client-side handling.
