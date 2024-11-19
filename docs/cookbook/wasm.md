# WASM Example

Because rust can compile to WASM, it is possible to use BDK in the browser. However, there are some limitations to keep in mind which will be highlighted in this example.

!!! warning
    There are several limitations to using BDK in WASM. Basically any functionality that requires OS access is not directly available in WASM and must therefore be handled in JavaScript. Some key limitations include:

    - No access to the file system
    - No access to the system time
    - Network access is limited to http(s)

## WASM Limitations Overview

### No access to the file system
With no direct access to the file system, persistence cannot be handled by BDK directly. Instead, an in memory wallet must be used in the WASM environment, and the data must be exported through a binding to the JavaScript environment to be persisted.

### No access to the system time
Any function that requires system time, such as any sort of timestamp, must access system time through a wasm binding to the JavaScript environment. This means some BDK functions that are commonly used in rust won't work in WASM and instead an alternate rust function that takes a timestamp as an argument must be used (I.E. instead of `.apply_update()` we must use `.apply_update_at()`).

### Network access is limited to http(s)
This effectively means that the blockchain client must be an Esplora instance. Both RPC and Electrum clients require sockets and will not work for BDK in a WASM environment out of the box.

## Troubleshooting
WASM errors can be quite cryptic, so it's important to understand the limitations of the WASM environment. One common error you might see while running a BDK function through a WASM binding in the browser is `unreachable`. This error likely will not point you to the actual BDK function that is causing the error. Instead you need to be able to assess whether you are calling a function that uses a rust feature that is unsupported in the WASM environment. For example, if you do a scan and then try to use `.apply_update()` you will get an `unreachable` error. This is because `.apply_update()` requires system time, which is not available in the WASM environment. Instead you need to use `.apply_update_at()` which takes an explicit timestamp as an argument (see below).

## Quickstart WASM

In this example we will cover basic BDK functionality in a WASM environment, similar to the [Quick Start Example](./quickstart.md). We will show code snippets for both the rust and JavaScript necessary, and we will highlight the key differenced from the rust quickstart example (due to WASM limitations).

!!! info
    The WASM example code is split into two project folders: a rust project that uses wasm-pack to compile rust code to WASM files, and a JavaScript project that pulls the WASM project as a dependency. The JS project represents the web app and the rust project is used to generate an npm module. For simple use cases the `bdk-wasm` package can be added as a dependency as is, but for more advanced use cases it may be necessary to build a custom WASM module.

### Initializing a Wallet

From JS running in our browser, we initialize a wallet like so:

```javascript
--8<-- "examples/wasm/js/index.js:new"
```

Notice we are including blockchain client details here (Signet, and the esplora url). This is because we are forced to use esplora, so we may as well initialize the client at the same time as the wallet. Here is the rust code that gets called:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:new"
```

Notice we are using an in-memory wallet with `.create_wallet_no_persist()`. If you try to use persistence through file or database you will get an error becuase those features require OS access. Instead we have to create a binding to pass the wallet data to the JavaScript environment where we can handle persistence.

### Scan and Apply Update

We can now scan the blockchain client for data relevant to our wallet. Here is the JS code:

```javascript
--8<-- "examples/wasm/js/index.js:scan"
```

Notice we've set the binding up to have a variable stop-gap, so we can modify this value directly in our webapp if desired. Here is the rust code that gets called:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:scan"
```

Notice we are using a JS binding to access system time with `js_sys::Date::now()`, then passing that timestamp to the `apply_update_at()` function, rather than attempting to use the `.apply_update()` function which would throw an error.

### Balance and Addresses

We can now get the balance of our wallet and generate a new address. Here is the JS code:

```javascript
--8<-- "examples/wasm/js/index.js:utils"
```

Here is the rust code that gets called:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:utils"
```
