// --8<-- [start:file]

let mnemonic = try Mnemonic(wordCount: .words12)
let secretKey = DescriptorSecretKey(
    network: Network.signet,
    mnemonic: mnemonic,
    password: nil
)

let descriptor = Descriptor.newBip86(
    secretKey: secretKey,
    keychain: .external,
    network: Network.signet
)
let changeDescriptor = Descriptor.newBip86(
    secretKey: secretKey,
    keychain: .internal,
    network: Network.signet
)

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

// TODO: tx build + broadcast, recovery, storage, etc.

// --8<-- [end:file]