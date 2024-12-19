import {  WalletWrapper, greet } from '../rust/pkg';

// --8<-- [start:store]
// simple string storage example
const Store = {
    save: data => {
        if (!data) {
            console.log("No data to save");
            return;
        }
        localStorage.setItem("walletData", data);  // data is already a JSON string
    },
    load: () => {
        return localStorage.getItem("walletData");  // return the JSON string directly
    }
}
// --8<-- [end:store]

// --8<-- [start:descriptors]
const externalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m";
const internalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr";
// --8<-- [end:descriptors]

async function run() {    
    console.log(greet()); // Should print "Hello, bdk-wasm!"
    
    // --8<-- [start:wallet]
    let walletDataString = Store.load();
    console.log("Wallet data:", walletDataString);

    let wallet;
    if (!walletDataString) {
        console.log("Creating new wallet");
        wallet = new WalletWrapper(
            "signet",
            externalDescriptor,
            internalDescriptor,
            "https://mutinynet.com/api"
        );

        console.log("Performing Full Scan...");
        await wallet.scan(2);

        const stagedDataString = wallet.take_staged();
        console.log("Staged:", stagedDataString);

        Store.save(stagedDataString);
        console.log("Wallet data saved to local storage");
        walletDataString = stagedDataString;
    } else {
        console.log("Loading wallet");
        wallet = WalletWrapper.load(
            walletDataString,
            "https://mutinynet.com/api",
            externalDescriptor,
            internalDescriptor
        );

        console.log("Syncing...");
        await wallet.sync(2);

        const stagedDataString = wallet.take_staged();
        console.log("Staged:", stagedDataString);

        Store.save(stagedDataString);
        console.log("Wallet data saved to local storage");
    }
    // --8<-- [end:wallet]
    
    // --8<-- [start:utils]
    // Test balance
    console.log("Balance:", wallet.balance());
    
    // Test address generation
    console.log("New address:", wallet.reveal_next_address());

    // handle changeset merge on rust side
    const mergedDataString = wallet.take_merged(walletDataString);
    
    console.log("Merged:", mergedDataString);

    Store.save(mergedDataString);
    console.log("new address saved");
    // --8<-- [end:utils]
}

run().catch(console.error);

// to clear local storage:
// localStorage.removeItem("walletData");
