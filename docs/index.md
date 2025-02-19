# The Bitcoin Development Kit

The Bitcoin Development Kit (BDK) project was created to provide well engineered and reviewed components for building bitcoin-based applications.

The core components of BDK are written in the [Rust] language and live in the [`bitcoindevkit/bdk`][bitcoindevkit/bdk] repository. The core BDK components are built upon the excellent [`rust-bitcoin`][rust-bitcoin] and [`rust-miniscript`][rust-miniscript] crates.

The BDK team also maintains the [`bitcoindevkit/bdk-ffi`][bitcoindevkit/bdk-ffi] repository which provide cross-platform versions of the high level BDK APIs. Platforms currently supported by the BDK team include: [Kotlin] (Android, Linux, macOS), [Swift] (iOS, macOS), and [Python] (Linux, macOS, Windows). There are also various [3rd party supported bindings for other languages](./getting-started/3rd-party-bindings.md), including Flutter, ReactNative, and JavaScript (WASM bindings for Browser/Node/ReactNative).

!!! warning
    The [`bitcoindevkit/bdk-ffi`][bitcoindevkit/bdk-ffi] project has not yet been updated to use the new `BDK 1.0` crates. For current status and timeline for bdk-ffi, see the [`bdk-ffi`][bitcoindevkit/bdk-ffi] project repository.

[bitcoindevkit/bdk]: https://github.com/bitcoindevkit/bdk
[rust-bitcoin]: https://github.com/rust-bitcoin/rust-bitcoin
[rust-miniscript]: https://github.com/rust-bitcoin/rust-miniscript
[bitcoindevkit/bdk-ffi]: https://github.com/bitcoindevkit/bdk-ffi
[Rust]: https://www.rust-lang.org/
[Kotlin]: https://kotlinlang.org/
[Swift]: https://www.swift.org/
[Python]: https://www.python.org/
