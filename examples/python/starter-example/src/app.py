import sys
from pathlib import Path
from bdkpython import (
    Descriptor,
    Network,
    Persister,
    Wallet,
    EsploraClient,
    KeychainKind,
    Address,
    Amount,
    TxBuilder,
    FeeRate,
)

SIGNET_ESPLORA_URL = "https://blockstream.info/signet/api/"
PERSISTENCE_FILE_PATH = str(Path.cwd() / "starter.sqlite")


def main():
    # --8<-- [start:descriptors]
    descriptor = Descriptor(
        "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m",
        Network.SIGNET,
    )
    change_descriptor = Descriptor(
        "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr",
        Network.SIGNET,
    )
    # --8<-- [end:descriptors]

    # --8<-- [start:wallet]
    db_path = Path(PERSISTENCE_FILE_PATH)
    persistence_exists = db_path.exists()
    persister = Persister.new_sqlite(PERSISTENCE_FILE_PATH)

    if persistence_exists:
        print("Loading up existing wallet")
        wallet = Wallet.load(
            descriptor=descriptor,
            change_descriptor=change_descriptor,
            persister=persister,
        )
    else:
        print("Creating new wallet")
        wallet = Wallet(
            descriptor=descriptor,
            change_descriptor=change_descriptor,
            network=Network.SIGNET,
            persister=persister,
        )
    # --8<-- [end:wallet]

    # --8<-- [start:client]
    esplora_client = EsploraClient(SIGNET_ESPLORA_URL)
    full_scan_request = wallet.start_full_scan().build()
    update = esplora_client.full_scan(
        request=full_scan_request, stop_gap=10, parallel_requests=1
    )
    wallet.apply_update(update)
    balance = wallet.balance().total.to_sat()
    print(f"Balance: {balance}")
    # --8<-- [end:client]

    # --8<-- [start:address]
    if balance < 5000:
        print("Your wallet does not have sufficient balance for the following steps!")
        address_info = wallet.reveal_next_address(KeychainKind.EXTERNAL)
        print(
            f"Send Signet coins to address {address_info.address} (address generated at index {address_info.index})"
        )
        wallet.persist(persister)
        sys.exit(0)
    # --8<-- [end:address]

    # --8<-- [start:recipient]
    faucet_address = Address(
        "tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4", Network.SIGNET
    )
    amount = Amount.from_sat(4000)
    # --8<-- [end:recipient]

    # --8<-- [start:transaction]
    psbt = (
        TxBuilder()
        .add_recipient(faucet_address.script_pubkey(), amount)
        .fee_rate(FeeRate.from_sat_per_vb(7))
        .finish(wallet)
    )

    wallet.sign(psbt)
    tx = psbt.extract_tx()
    esplora_client.broadcast(tx)
    print(f"Transaction broadcast successfully! Txid: {tx.compute_txid()}")
    # --8<-- [end:transaction]


if __name__ == "__main__":
    main()
