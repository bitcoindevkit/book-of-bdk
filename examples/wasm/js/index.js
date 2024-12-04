import {  WalletWrapper, greet } from '../rust/pkg';

// --8<-- [start:store]
// needed to handle js Map serialization
const Store = {
    save: data => {
        if (!data) {
            console.log("No data to save");
            return;
        }
        const serializedStaged = JSON.stringify(data, (key, value) => {
            if (value instanceof Map) {
                return {
                    dataType: 'Map',
                    value: Array.from(value.entries())
                };
            }
            return value;
        });
        localStorage.setItem("walletData", serializedStaged);
    },
    load: () => {
        const walletDataString = localStorage.getItem("walletData");
        // Convert serialized Maps back to Map objects when loading
        const walletData = JSON.parse(walletDataString, (key, value) => {
            if (value?.dataType === 'Map') {
                return new Map(value.value);
            }
            return value;
        });
        return walletData;
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
    const walletData = Store.load();
    console.log("Wallet data:", walletData);

    let wallet;
    if (!walletData) {
        console.log("Creating new wallet");
        wallet = new WalletWrapper(
            "signet",
            externalDescriptor,
            internalDescriptor,
            "https://mutinynet.com/api"
        );

        console.log("Performing Full Scan...");
        await wallet.scan(2);

        const stagedData = wallet.take_staged();
        console.log("Staged:", stagedData);

        Store.save(stagedData);
        console.log("Wallet data saved to local storage");
    } else {
        console.log("Loading wallet");
        wallet = WalletWrapper.load(
            walletData,
            "https://mutinynet.com/api",
            externalDescriptor,
            internalDescriptor
        );

        console.log("Syncing...");
        await wallet.sync(2);
    }
    // --8<-- [end:wallet]
    
    // --8<-- [start:utils]
    // Test balance
    console.log("Balance:", wallet.balance());
    
    // Test address generation
    console.log("New address:", wallet.get_new_address());

    const mergedData = wallet.take_merged(walletData);
    console.log("Merged:", mergedData);

    Store.save(mergedData);
    console.log("new address saved");
    // --8<-- [end:utils]
}

run().catch(console.error);

// to clear local storage:
// localStorage.removeItem("walletData");
