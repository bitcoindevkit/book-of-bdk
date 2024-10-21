// --8<-- [start:file]
import org.bitcoindevkit.*

fun main() {
    // --8<-- [start:descriptors]
    val descriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m"
    val changeDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr"
    // --8<-- [end:descriptors]

    try {
        // Create a new wallet
        val wallet = Wallet(
            descriptor = Descriptor(descriptor, Network.TESTNET),
            changeDescriptor = Descriptor(changeDescriptor, Network.TESTNET),
            network = Network.TESTNET,
            databaseConfig = DatabaseConfig.Memory
        )

        // --8<-- [start:client]
        // Create a blockchain
        val blockchain = Blockchain(BlockchainConfig.Esplora(
          EsploraConfig(
              baseUrl = "http://signet.bitcoindevkit.net",
              stopGap = 10u,
              timeout = 5u,
              proxy = null,
              concurrency = null
            )
        ))
        // --8<-- [end:client]

        // --8<-- [start:scan]
        // Sync the wallet
        wallet.sync(blockchain, LogProgress())
        // Get the balance
        val balance = wallet.getBalance()
        println("Wallet balance: $balance")

        // --8<-- [end:scan]

        // Get an address
        val address = wallet.getAddress(AddressIndex.New)
        println("New address: ${address.address}")

    } catch (e: Exception) {
        println("An error occurred: ${e.message}")
    }
}

class LogProgress : Progress {
    override fun update(progress: Float, message: String?) {
        println("Sync progress: $progress - $message")
    }
}
// --8<-- [end:file]