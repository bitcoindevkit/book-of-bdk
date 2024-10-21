// --8<-- [start:file]

val mnemonic = Mnemonic(WordCount.WORDS12)
val secretKey = DescriptorSecretKey(
  Network.SIGNET, 
  mnemonic, 
  null
)

val descriptor = Descriptor.newBip86(
  descriptorSecretKey, 
  KeychainKind.EXTERNAL, 
  Network.SIGNET
)
val changeDescriptor = Descriptor.newBip86(
  secretKey, 
  KeychainKind.INTERNAL, 
  Network.SIGNET
)

val wallet = Wallet(
  descriptor, 
  changeDescriptor, 
  Network.SIGNET
)

val addressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)


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
// --8<-- [end:scan]

// TODO: tx build + broadcast, recovery, storage, etc.

// --8<-- [end:file]