import { __wbg_set_wasm, WalletWrapper, greet } from '../rust/pkg/bdk_wasm_bg.js';
import * as wasm from '../rust/pkg/bdk_wasm_bg.wasm';

async function run() {
    // Initialize WASM
    __wbg_set_wasm(wasm);
    
    console.log(greet()); // Should print "Hello, bdk-wasm!"
    
    // Test wallet creation
    // --8<-- [start:new]
    const wallet = new WalletWrapper(
        "signet",
        "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m",
        "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr",
        "https://mutinynet.com/api"
    );
    // --8<-- [end:new]

    // --8<-- [start:scan]
    // Test sync
    await wallet.sync(2);
    // --8<-- [end:scan]
    
    // --8<-- [start:utils]
    // Test balance
    console.log("Balance:", wallet.balance());
    
    // Test address generation
    console.log("New address:", wallet.get_new_address());
    // --8<-- [end:utils]
}

run().catch(console.error);