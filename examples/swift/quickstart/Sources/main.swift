// --8<-- [start:file]
import BitcoinDevKit

// --8<-- [start:descriptors]
let descriptor = try Descriptor(descriptor: "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m", network: Network.signet)
let changeDescriptor = try Descriptor(descriptor: "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr", network: Network.signet)
// --8<-- [end:descriptors]

let wallet = try Wallet(
    descriptor: descriptor,
    changeDescriptor: changeDescriptor,
    network: Network.signet,
    connection: Connection.newInMemory()
)

let addressInfo = wallet.revealNextAddress(keychain: .external)
print("Generated address \(addressInfo.address) at index \(addressInfo.index)")

// --8<-- [start:client]
let esploraClient = EsploraClient(url: "https://mutinynet.com/api")
// --8<-- [end:client]

// --8<-- [start:scan]
let syncRequest = try wallet.startSyncWithRevealedSpks().build()
let update = try esploraClient.sync(
    syncRequest: syncRequest,
    parallelRequests: UInt64(5)
)
try wallet.applyUpdate(update: update)
let balance = wallet.balance()
print("Wallet balance: \(balance.total.toSat()) sat")
// --8<-- [end:scan]

// --8<-- [end:file]
