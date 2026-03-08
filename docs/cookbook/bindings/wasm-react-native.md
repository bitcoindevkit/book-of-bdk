# WASM Example (React Native)

The [`bdk-wasm`](https://github.com/bitcoindevkit/bdk-wasm) library can be used in React Native
apps since React Native supports WebAssembly via the **Hermes** JS engine (enabled by default in
React Native 0.70+). This gives an extra layer of security compared to browser-based usage since
keys are never exposed to a browser environment.

!!! info
    This page covers using `bdk-wasm` specifically in a React Native context.
    For the browser/Node.js WASM example, see the [WASM Example](wasm.md) page.
    For other React Native options, see the [3rd Party Bindings](../../getting-started/3rd-party-bindings.md) page.

## WASM Limitations in React Native

The same WASM limitations from the browser apply here:

- **No file system access** — persistence must be handled via JS (e.g. AsyncStorage or MMKV)
- **No system time access** — use `.apply_update_at()` with a JS timestamp instead of `.apply_update()`
- **Network limited to http(s)** — only Esplora client works; RPC and Electrum require OS-level sockets

## Installation

```bash
npm install bdk-wasm
# or
yarn add bdk-wasm
```

!!! warning
    Make sure Hermes is enabled in your React Native project. It is on by default for React Native 0.70+.
    You can verify in `android/app/build.gradle`:
    ```
    hermesEnabled = true
    ```

## Initialize WASM

Call `init()` once at app startup before using any BDK functions:

```javascript
import init, { Mnemonic, Network } from 'bdk-wasm';

await init();
```

## Create a Wallet

```javascript
import init, {
  Mnemonic,
  DescriptorSecretKey,
  Descriptor,
  Wallet,
  KeychainKind,
  Network,
} from 'bdk-wasm';

async function createWallet() {
  await init();

  const network = Network.Signet;
  const mnemonic = Mnemonic.generate(12);
  const secretKey = new DescriptorSecretKey(network, mnemonic, undefined);

  const externalDescriptor = new Descriptor(
    `wpkh(${secretKey.asString()}/84'/1'/0'/0/*)`,
    network
  );
  const internalDescriptor = new Descriptor(
    `wpkh(${secretKey.asString()}/84'/1'/0'/1/*)`,
    network
  );

  const wallet = Wallet.create(network, externalDescriptor, internalDescriptor);

  const address = wallet.revealNextAddress(KeychainKind.External);
  console.log('Address:', address.address.toString());

  return wallet;
}
```

## Persistence with AsyncStorage

Since there is no file system access in the WASM environment, wallet data must be exported
to JavaScript and persisted manually. Here we use AsyncStorage as an example:

```javascript
import AsyncStorage from '@react-native-async-storage/async-storage';

// After every sync — save staged wallet changes
async function saveWallet(wallet) {
  const staged = wallet.takeStagedStr();
  if (staged) {
    const existing = await AsyncStorage.getItem('bdk_wallet');
    if (existing) {
      wallet.applyChangesetStr(existing);
    }
    await AsyncStorage.setItem('bdk_wallet', staged);
  }
}

// On startup — load wallet from persisted data
async function loadWallet(externalDescriptor, internalDescriptor) {
  const stored = await AsyncStorage.getItem('bdk_wallet');
  if (stored) {
    return Wallet.load(stored, externalDescriptor, internalDescriptor);
  }
  return null;
}
```

## Sync with Esplora

```javascript
import { EsploraClient } from 'bdk-wasm';

const client = new EsploraClient('https://mutinynet.com/api'); // Signet

// Full scan (first time)
const now = Math.floor(Date.now() / 1000);
const fullScanRequest = wallet.startFullScan();
const update = await client.fullScan(fullScanRequest, 20, 1);
wallet.applyUpdateAt(update, now);

// Sync (subsequent runs)
const syncRequest = wallet.startSync();
const syncUpdate = await client.sync(syncRequest, 1);
wallet.applyUpdateAt(syncUpdate, Math.floor(Date.now() / 1000));
```

!!! note "System Time Consideration"
    Notice we pass `Math.floor(Date.now() / 1000)` explicitly to `.applyUpdateAt()`.
    This is required because WASM has no access to system time. Using `.applyUpdate()`
    would throw an `unreachable` error.

## Get Balance and Address

```javascript
const balance = wallet.balance();
console.log('Total balance (sats):', balance.total.toSat());

const addressInfo = wallet.revealNextAddress(KeychainKind.External);
console.log('New address:', addressInfo.address.toString());
console.log('Address index:', addressInfo.index);

// Remember to save after revealing a new address
await saveWallet(wallet);
```

## Further Reading

- [`bdk-wasm` GitHub repository](https://github.com/bitcoindevkit/bdk-wasm)
- [WASM Example (Browser/Node)](wasm.md)
- [3rd Party Bindings](../../getting-started/3rd-party-bindings.md)
