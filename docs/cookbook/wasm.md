# WASM Example

WASM bindings can be used to access rust code from virtually any JavaScript environment: browsers, Node.js, ReactNative, etc.

!!! info
    This page details how to build a custom WASM package that uses BDK rust crate under the hood. If you want an out of the box solution for JS(WASM) bindings for BDK which don't require writing any rust code, see the WASM section on the [3rd Party Bindings](https://bitcoindevkit.github.io/book-of-bdk/getting-started/3rd-party-bindings/) page for a pre-packaged npm module.

Because rust can compile to WASM, it is possible to use BDK in the browser. However, there are a few limitations to keep in mind which will be highlighted in this example. That being said, there are perfectly viable work-arounds for these limitations that should suffice for most use cases.

!!! warning
    There are several limitations to using BDK in WASM. Basically any functionality that requires OS access is not directly available in WASM and must therefore be handled in JavaScript. Some key limitations include:

    - No access to the file system
    - No access to the system time
    - Network access is limited to http(s)

## WASM Considerations Overview

### No access to the file system
With no direct access to the file system, persistence cannot be handled by BDK directly. Instead, an in memory wallet must be used in the WASM environment, and the data must be exported through a binding to the JavaScript environment to be persisted.

### No access to the system time
Any function that requires system time, such as any sort of timestamp, must access system time through a wasm binding to the JavaScript environment. This means some BDK functions that are commonly used in rust won't work in WASM and instead an alternate rust function that takes a timestamp as an argument must be used (I.E. instead of `.apply_update()` we must use `.apply_update_at()`).

### Network access is limited to http(s)
This effectively means that the blockchain client must be an Esplora instance. Both RPC and Electrum clients require sockets and will not work for BDK in a WASM environment out of the box.

## Troubleshooting
WASM errors can be quite cryptic, so it's important to understand the limitations of the WASM environment. One common error you might see while running a BDK function through a WASM binding in the browser is `unreachable`. This error likely will not point you to the actual BDK function that is causing the error. Instead you need to be able to assess whether you are calling a function that uses a rust feature that is unsupported in the WASM environment. For example, if you do a scan and then try to use `.apply_update()` you will get an `unreachable` error. This is because `.apply_update()` requires system time, which is not available in the WASM environment. Instead you need to use `.apply_update_at()` which takes an explicit timestamp as an argument (see below).

## WASM App Example

In this example we will cover basic BDK functionality in a WASM environment. We will show code snippets for both the rust and JavaScript necessary to create a custom WASM package, and we will highlight the key differences from the plain rust examples (due to WASM limitations).

!!! info
    The WASM example code is split into two project folders: a rust project that uses wasm-pack to compile rust code to WASM files, and a JavaScript project that pulls the WASM project as a dependency. The JS project represents the web app and the rust project is used to generate an npm module.

### Initializing a Wallet

From JS running in our browser, first we need our descriptors:

```javascript
--8<-- "examples/wasm/js/index.js:descriptors"
```

Then we can initialize the wallet, we'll use some conditional logic here to either 1) create a new wallet and perform a full scan, or 2) load a wallet from stored data and sync it to get recent updates.

```javascript
--8<-- "examples/wasm/js/index.js:wallet"
```

#### Network Consideration
Notice we are including blockchain client details in wallet initialization (Signet, and the esplora url). This is because we are forced to use esplora, so we may as well initialize the client at the same time as the wallet.

Here is the relevant rust code:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:wallet"
```

The first time you load the page in your browser, you should see info in the console confirming that a new wallet was created and a full scan was performed. If you then reload the page you should see that the wallet was loaded from the previously saved data and a sync was performed instead of a full scan.

#### System Time Consideration
Notice we are using a JS binding to access system time with `js_sys::Date::now()`, then passing that timestamp to the `apply_update_at()` function, rather than attempting to use the `.apply_update()` function which would throw an error.

#### Persistence Consideration
Also notice we are using an in-memory wallet with `.create_wallet_no_persist()`. If you try to use persistence through file or database you will get an error becuase those features require OS access. Instead we have to create a binding to pass the wallet data to the JavaScript environment where we can handle persistence. We have a method to grab the new updates to the wallet data, and a method to merge new updates with existing data. With this simple approach to persistence we must always merge existing data with the updates unless there is no existing data (i.e. after new wallet creation). The rust side methods to extract the wallet data are:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:store"
```

Notice we're converting the wallet data to a JSON string so that it plays nicely with WASM; and on the JS side we'll save our data string with a minimal custom browser store:

```javascript
--8<-- "examples/wasm/js/index.js:store"
```

This is just to show an example of how the wallet data can be persisted. We're using local storage here, but in practice a wallet app would generally use cloud storage of some sort since browser local storage tends to be temporary.

### Balance and Addresses

We can now get the balance of our wallet and generate a new address. Here is the JS code:

```javascript
--8<-- "examples/wasm/js/index.js:utils"
```

Here is the rust code that gets called:

```rust
--8<-- "examples/wasm/rust/src/lib.rs:utils"
```

Notice we call `take_merged()` and `Store.save()` after generating a new address so our wallet keeps track of generated addresses (so we don't re-use them). If you reload the browser you can see the generated address value updated along with the index.