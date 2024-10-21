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
    databaseConfig: DatabaseConfig.memory
)

let addressInfo = try wallet.getAddress(addressIndex: AddressIndex.new)
print("Generated address \(addressInfo.address) at index \(addressInfo.index)")

// --8<-- [start:client]
let esploraConfig = EsploraConfig(
    baseUrl: "http://signet.bitcoindevkit.net",//"https://mutinynet.com/api",
    proxy: nil,
    concurrency: nil,
    stopGap: 10,
    timeout: nil
)
let esploraClient = BlockchainConfig.esplora(config: esploraConfig)
let blockchain = try Blockchain(config: esploraClient)
// --8<-- [end:client]

// --8<-- [start:scan]
try wallet.sync(blockchain: blockchain, progress: nil)
print("Wallet synced successfully.")

let balance = try wallet.getBalance()
print("Wallet balance: \(balance.total) sat")
// --8<-- [end:scan]

// --8<-- [end:file]
