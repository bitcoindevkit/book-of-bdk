package org.starterexample

import org.bitcoindevkit.Address
import org.bitcoindevkit.Amount
import org.bitcoindevkit.Connection
import org.bitcoindevkit.Descriptor
import org.bitcoindevkit.EsploraClient
import org.bitcoindevkit.FeeRate
import org.bitcoindevkit.FullScanRequest
import org.bitcoindevkit.KeychainKind
import org.bitcoindevkit.Network
import org.bitcoindevkit.Psbt
import org.bitcoindevkit.Transaction
import org.bitcoindevkit.TxBuilder
import org.bitcoindevkit.Wallet
import java.io.File
import kotlin.system.exitProcess

private const val SIGNET_ESPLORA_URL = "https://blockstream.info/signet/api/"
private val PERSISTENCE_FILE_PATH = run {
    val currentDirectory = System.getProperty("user.dir")
    val dbFileName = "starter.sqlite"
    "$currentDirectory/$dbFileName"
}

fun main() {
// --8<-- [start:descriptors]
val descriptor = Descriptor("tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m", Network.SIGNET)
val changeDescriptor = Descriptor("tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr", Network.SIGNET)
// --8<-- [end:descriptors]

// --8<-- [start:wallet]
val persistenceExists = File(PERSISTENCE_FILE_PATH).exists()
val connection = Connection(PERSISTENCE_FILE_PATH)

val wallet = if (persistenceExists) {
    println("Loading up existing wallet")
    Wallet.load(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        connection = connection
    )
} else {
    println("Creating new wallet")
    Wallet(
        descriptor = descriptor,
        changeDescriptor = changeDescriptor,
        network = Network.SIGNET,
        connection = connection
    )
}
// --8<-- [end:wallet]

// --8<-- [start:client]
val esploraClient: EsploraClient = EsploraClient(SIGNET_ESPLORA_URL)
val fullScanRequest: FullScanRequest = wallet.startFullScan().build()
val update = esploraClient.fullScan(
    request = fullScanRequest,
    stopGap = 10uL,
    parallelRequests = 1uL
)
wallet.applyUpdate(update)
val balance = wallet.balance().total.toSat()
println("Balance: $balance")
// --8<-- [end:client]

// --8<-- [start:address]
if (balance < 5000uL) {
    println("Your wallet does not have sufficient balance for the following steps!");
    val address = wallet.revealNextAddress(KeychainKind.EXTERNAL)
    println("Send Signet coins to address ${address.address} (address generated at index ${address.index})")
    wallet.persist(connection)
    exitProcess(0)
}
// --8<-- [end:address]

// --8<-- [start:recipient]
val faucetAddress: Address = Address("tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4", Network.SIGNET)
val amount: Amount = Amount.fromSat(4000uL)
// --8<-- [end:recipient]

// --8<-- [start:transaction]
val psbt: Psbt = TxBuilder()
    .addRecipient(script = faucetAddress.scriptPubkey(), amount = amount)
    .feeRate(FeeRate.fromSatPerVb(7uL))
    .finish(wallet)

wallet.sign(psbt)
val tx: Transaction = psbt.extractTx()
esploraClient.broadcast(tx)
println("Transaction broadcast successfully! Txid: ${tx.computeTxid()}")
// --8<-- [end:transaction]
}
