# Introduction
The Bitcoin Development Kit (BDK) project was created to provide well engineered and reviewed components for building bitcoin based applications. The core components of BDK are written in the [`Rust`] language and live in the [`bitcoindevkit/bdk`] repository. The core BDK components are built upon the excellent [`rust-bitcoin`] and [`rust-miniscript`] crates.

The BDK team also maintains the [`bitcoindevkit/bdk-ffi`] repository which provide cross platform versions of the high level BDK APIs. Current supported platforms are: [`Kotlin`] (android, linux, MacOS), [`Swift`] (iOS, MacOS), and [`Python`] (linux, MacOS, Windows).

:::warning
The BDK developers are in the process of rewriting major components of the software to be release in an upcoming `1.0` version. `BDK 1.0` is a still under active development and should be considered "alpha" quality. This means APIs may change and full testing and documentation has not been completed. For current status and release timeline please see the [`BDK 1.0`] project page.

The [`bitcoindevkit/bdk-ffi`] project has not yet been updated to use the new `BDK 1.0` crates. For current status and timeline for `BDK-FFI 1.0` see the [`BDK-FFI`] project page.
:::

[`bitcoindevkit/bdk`]: https://github.com/bitcoindevkit/bdk
[`rust-bitcoin`]: https://github.com/rust-bitcoin/rust-bitcoin
[`rust-miniscript`]: https://github.com/rust-bitcoin/rust-miniscript
[`bitcoindevkit/bdk-ffi`]: https://github.com/bitcoindevkit/bdk-ffi
[`Rust`]: https://www.rust-lang.org/
[`Kotlin`]: https://kotlinlang.org/
[`Swift`]: https://www.swift.org/
[`Python`]: https://www.python.org/