
# Generating public keys from descriptors

BDK wallets require the use of descriptors, and it's not always obvious on how to generate raw publics key from them.
In this example we generate public keys in order to create a simple 2-of-2 multisig setup:

```rust 
// Create new wallet.

const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/1'/0'/1/*)";

let wallet = Wallet::create(INTERNAL_DESCRIPTOR, EXTERNAL_DESCRIPTOR)
    .network(Network::Testnet)
    .create_wallet_no_persist()
    .unwrap();

// Get the public key at the specified derivation index.
let my_key_1 = wallet
    .public_key_at_index(KeychainKind::External, 0)
    .unwrap();

let my_key_2 = wallet
    .public_key_at_index(KeychainKind::External, 1)
    .unwrap();

let (descriptor, _, _) = crate::descriptor! {
        wsh (
            multi(2, my_key_1, my_key_2)
        )
}
.unwrap();

println!("Descriptor {}", descriptor.to_string());
```
