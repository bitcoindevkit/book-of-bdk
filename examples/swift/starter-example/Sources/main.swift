import Foundation
import BitcoinDevKit

func dbResourcePath() -> URL {
    let thisFilePath = URL(fileURLWithPath: #file)
    let db = thisFilePath
        .deletingLastPathComponent()
        .appendingPathComponent("starter.sqlite")
    
    return db
}

let dbFilePath = dbResourcePath()

// --8<-- [start:descriptors]
let descriptor = try Descriptor(descriptor: "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m", network: Network.signet)
let changeDescriptor = try Descriptor(descriptor: "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr", network: Network.signet)
// --8<-- [end:descriptors]

// --8<-- [start:wallet]
let wallet: Wallet
let connection: Connection

if FileManager.default.fileExists(atPath: dbFilePath.path) {
    print("Loading up existing wallet")
    connection = try Connection(path: dbFilePath.path)
    wallet = try Wallet.load(
        descriptor: descriptor,
        changeDescriptor: changeDescriptor,
        connection: connection
    )
} else {
    print("Creating new wallet")
    connection = try Connection(path: dbFilePath.path)
    wallet = try Wallet(
        descriptor: descriptor,
        changeDescriptor: changeDescriptor,
        network: Network.signet,
        connection: connection
    )
}
// --8<-- [end:wallet]

// --8<-- [start:client]
let esploraClient = EsploraClient(url: "https://blockstream.info/signet/api/")
let fullScanRequest = try wallet.startFullScan().build()
let update = try esploraClient.fullScan(
    request: fullScanRequest,
    stopGap: UInt64(10),
    parallelRequests: UInt64(1)
)
try wallet.applyUpdate(update: update)
let balance = wallet.balance()
print("Wallet balance: \(balance.total.toSat()) sat")
// --8<-- [end:client]

// --8<-- [start:address]
if (balance.total.toSat() < UInt64(5000)) {
    print("Your wallet does not have sufficient balance for the following steps!");
    let address = wallet.revealNextAddress(keychain: KeychainKind.external)
    print("Send Signet coins to address \(address.address) (address generated at index \(address.index))")
    try wallet.persist(connection: connection)
    exit(0)
}
// --8<-- [end:address]

// --8<-- [start:recipient]
let faucetAddress: Address = try Address(address: "tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4", network: Network.signet)
let amount: Amount = Amount.fromSat(fromSat: UInt64(4000))
// --8<-- [end:recipient]

// --8<-- [start:transaction]
let psbt: Psbt = try TxBuilder()
    .addRecipient(script: faucetAddress.scriptPubkey(), amount: amount)
    .feeRate(feeRate: try FeeRate.fromSatPerVb(satPerVb: UInt64(7)))
    .finish(wallet: wallet)

try wallet.sign(psbt: psbt)
let tx: Transaction = try psbt.extractTx()
esploraClient.broadcast(tx)
print("Transaction broadcast successfully! Txid: \(tx.computeTxid())")
// --8<-- [end:transaction]
