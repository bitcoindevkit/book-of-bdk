// --8<-- [start:file]

// --8<-- [start:descriptors]
let descriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m"
let changeDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr"
// --8<-- [end:descriptors]

let wallet = try Wallet(
    descriptor: descriptor, 
    changeDescriptor: changeDescriptor, 
    network: Network.signet
)

// --8<-- [start:address]
let addressInfo = wallet.revealNextAddress(keychain: .external)
print("Generated address \(addressInfo.address) at index \(addressInfo.index)")
// --8<-- [end:address]

// --8<-- [start:client]  
let esploraClient = EsploraClient(url: "https://mutinynet.com/api")
// --8<-- [end:client]

// --8<-- [start:scan]
let syncRequest = wallet.startSyncWithRevealedSpks()
let update = try esploraClient.sync(
    syncRequest: syncRequest,
    parallelRequests: UInt64(5)
)
try wallet.applyUpdate(update: update)
let balance = wallet.balance()
print("Wallet balance: \(balance.total) sat")
// --8<-- [end:scan]

// --8<-- [end:file]