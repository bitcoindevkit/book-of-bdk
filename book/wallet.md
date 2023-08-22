# The `Wallet` struct
To create a simple wallet using the Bitcoin Development Kit, you'll need two main things:

1. One or more [output descriptors](https://github.com/bitcoin/bips/blob/master/bip-0380.mediawiki) to tell the wallet the keys to use when deriving new addresses and scanning the chain for receive transactions.
2. A block data source, either of [Electrum](), [Esplora](), or [Bitcoin Core]() so as to let the wallet sync itself and scan for incoming and outgoing transactions.

This chapter of the book will use the following testnet descriptors:

| Item                               | Data                                                                                                                                                     |
|:-----------------------------------|:---------------------------------------------------------------------------------------------------------------------------------------------------------|
| Fingerprint                        | `b5c61721`                                                                                                                                               |
| Mnemonic                           | `father excess argue gesture hero aware member gain dawn palace must subject`                                                                            |
| Root xprv                          | `tprv8ZgxMBicQKsPexSuK1ZFxrLohpDe9UbhepMka1DkF6721uZXXGjj4XDzrT4p9LNUVcdfMX6x8f2RZJhjmMjTvBCUCrZsTUJzHDg6MdYAbpE`                                        |
| BIP-84 private external descriptor | `wpkh(tprv8ZgxMBicQKsPexSuK1ZFxrLohpDe9UbhepMka1DkF6721uZXXGjj4XDzrT4p9LNUVcdfMX6x8f2RZJhjmMjTvBCUCrZsTUJzHDg6MdYAbpE/84h/1h/0h/0/*)#tfq86fm0`           |
| BIP-84 public external descriptor  | `wpkh([b5c61721/84'/1'/0']tpubDDfKhgyDAwCgTvnJq6KJN45Vw5zNvxTTZx85v4qEnVpwJDkk9zCYDvg1UDiSnGs8N9rBjjUgYBQN4xHuguF3T3wFgGq266DgLG3fMKzujUR/0/*)#mclqz3ms` |
