# Lazorkit | Program

## Overview

Onchain program for Secp256r1 verification and exercute transaction, management smart wallet

## Installation

Install anchor cli and solana cli
Link: [text](https://solana.com/docs/intro/installation)

## Setup with Raydium Swap

Dex smart-contract: `EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj`\
AMM smart-contract: `HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8`\
AMM config PDA: `8QN9yfKqWDoKjvZmqFsgCzAqwZBQuzVVnC388dN5RCPo`\
Fee collector: `3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR`

### Set solana validator to localnet

```bash
solana config set --url http://localhost:8899
```

### Run validator and clone all the smart-contracts and accounts

```bash
solana-test-validator --clone-upgradeable-program EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj --clone-upgradeable-program HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8 --clone 8QN9yfKqWDoKjvZmqFsgCzAqwZBQuzVVnC388dN5RCPo --clone 3XMrhbv989VxAMi3DErLV9eJht1pHppW5LbKxe9fkEFR --url devnet --reset
```

### Deploy the smart-contracts

```bash
anchor build && anchor deploy && anchor idl init -f ./target/idl/contract.json 3jq9oBWGCUWmBynC8TTBL9KWJdGegsChJ1c8ksybGhum

```

### Test the smart-contracts

```bash
anchor run test
```
