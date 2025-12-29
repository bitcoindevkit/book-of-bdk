# About The Bitcoin Development Kit

The Bitcoin Development Kit (BDK) project was created to provide well engineered and reviewed components for building bitcoin-based applications.

The core components of BDK are written in the [Rust] language and live in the [`bitcoindevkit` GitHub org][bitcoindevkit GitHub org]. The core BDK components are built upon the excellent [`rust-bitcoin`][rust-bitcoin] and [`rust-miniscript`][rust-miniscript] crates.

The BDK team also maintains the [`bitcoindevkit/bdk-ffi`][bitcoindevkit/bdk-ffi] repository which provide cross-platform versions of the high level BDK APIs. Platforms currently supported by the BDK team include: [Kotlin] (Android, Linux, macOS), [Swift] (iOS, macOS), and [Python] (Linux, macOS, Windows). There are also various [3rd party supported bindings for other languages](./getting-started/3rd-party-bindings.md), including Flutter, React Native, and JavaScript (WASM bindings for Browser/Node/React Native).

[bitcoindevkit GitHub org]: https://github.com/bitcoindevkit/
[rust-bitcoin]: https://github.com/rust-bitcoin/rust-bitcoin
[rust-miniscript]: https://github.com/rust-bitcoin/rust-miniscript
[bitcoindevkit/bdk-ffi]: https://github.com/bitcoindevkit/bdk-ffi
[Rust]: https://www.rust-lang.org/
[Kotlin]: https://kotlinlang.org/
[Swift]: https://www.swift.org/
[Python]: https://www.python.org/
