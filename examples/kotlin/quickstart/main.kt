// --8<-- [start:file]

// --8<-- [start:descriptors]
val descriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m"
val changeDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr"
// --8<-- [end:descriptors]

val wallet = Wallet(
  descriptor, 
  changeDescriptor, 
  Network.SIGNET
)

// --8<-- [start:address]
val addressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
println("Generated address ${addressInfo.address} at index ${addressInfo.index}")
// --8<-- [end:address]

// --8<-- [start:client]
val esploraClient: EsploraClient = EsploraClient("https://mutinynet.com/api")
// --8<-- [end:client]

// --8<-- [start:scan]
val syncRequest = wallet.startSyncWithRevealedSpks()
val update = try esploraClient.sync(
    syncRequest,
    5uL
)
wallet.applyUpdate(update)
val balance = wallet.balance()
println("Wallet balance: ${balance.total()} sat")
// --8<-- [end:scan]

// --8<-- [end:file]