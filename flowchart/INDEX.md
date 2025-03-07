# PDA architecture

```mermaid
    flowchart TD
        n1["Contract"] -- "seeds = [b'smart_wallet', randomId]" --> n3["Wallet PDA<br>"]
        n3 -- "seed = [b'smart_wallet_authority', wallet.key, randomId]" --> n5["WalletAuthenticator"]
        n3 -- "seed = [b'smart_wallet_data', wallet.key]" --> n6["Wallet Data"]
```

# PDA Ownership

```mermaid
    flowchart LR
        n7["Device"] --> n8["Passkey"]
        n8 --> n9["WalletAuthenticator<br>"]
        n10["Wallet PDA"] --> n9 & n14["WalletAuthenticator<br>"]
        n12["Device"] --> n13["Passkey"]
        n13 --> n14 & n15["WalletAuthenticator<br>"]
        n11["Wallet PDA"] --> n15
        n7@{ shape: diamond}
        n12@{ shape: diamond}
        n9@{ shape: hexagon}
        n14@{ shape: hexagon}
        n15@{ shape: hexagon}
````
