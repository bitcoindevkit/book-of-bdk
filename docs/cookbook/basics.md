# Basics

!!! tip
    This page is up-to-date with version `1.0.0-beta.1` of `bdk_wallet`.

Add BDK

=== "Rust"

    ``` toml
    [dependencies]
    bdk_wallet = { version = "1.0.0-beta.1", features = ["keys-bip39"] }
    bdk_esplora = {  version = "0.16.0" , features = ["blocking"] }
    ```

=== "Swift"

    1. From the Xcode File menu, select Add Package Dependencies...
    2. Enter `https://github.com/bitcoindevkit/bdk-swift` into the package repository URL text field

=== "Kotlin"

    ``` kotlin
    repositories {
        mavenCentral()
    }

    dependencies {
    
        // for jvm
        implementation 'org.bitcoindevkit:bdk-jvm:<version>'
        // OR for android
        implementation 'org.bitcoindevkit:bdk-android:<version>'
    
    }
    ```

Create Wallet Mnemonic

=== "Rust"

    ``` toml
    let mnemonic: GeneratedKey<_, Tap> =
        Mnemonic::generate((WordCount::Words12, Language::English))
            .expect("Failed to generate mnemonic");
    ```

=== "Swift"

    ``` swift
    let mnemonic = try Mnemonic(wordCount: .words12)
    ```

=== "Kotlin"

    ``` kotlin
    val mnemonic = Mnemonic(WordCount.WORDS12)
    ```

Create Wallet

=== "Rust"

    ``` toml
    let seed = mnemonic.to_seed("");
    let xprv: Xpriv =
        Xpriv::new_master(Network::Signet, &seed).expect("Failed to create master key");
    let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
        .build(Network::Signet)
        .expect("Failed to build external descriptor");
    let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
        .build(Network::Signet)
        .expect("Failed to build internal descriptor");
    let mut wallet = Wallet::create(descriptor, change_descriptor)
        .network(Network::Signet)
        .create_wallet_no_persist()
        .expect("Failed to create wallet");
    ```

=== "Swift"

    ``` swift
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
    ```

=== "Kotlin"

    ``` kotlin
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
    ```

Get Address

=== "Rust"

    ``` toml
    let address_info = wallet.reveal_next_address(KeychainKind::External);
    ```

=== "Swift"

    ``` swift
    let addressInfo = wallet.revealNextAddress(keychain: .external)
    ```

=== "Kotlin"

    ``` kotlin
    val addressInfo = wallet.revealNextAddress(KeychainKind.EXTERNAL)
    ```

Get Balance

=== "Rust"

    ``` toml
    let client = esplora_client::Builder::new("https://mutinynet.com/api").build_blocking();
    let request = wallet.start_full_scan();
    let mut update = client
        .full_scan(request, 5, 5)
        .expect("Failed to perform full scan");
    let now = UNIX_EPOCH
        .elapsed()
        .expect("Failed to get current time")
        .as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);
    wallet.apply_update(update).expect("Failed to apply update");
    let balance = wallet.balance();
    ```

=== "Swift"

    ``` swift
    let esploraClient = EsploraClient(url: "https://mutinynet.com/api")
    let syncRequest = wallet.startSyncWithRevealedSpks()
    let update = try esploraClient.sync(
        syncRequest: syncRequest,
        parallelRequests: UInt64(5)
    )
    try wallet.applyUpdate(update: update)
    let balance = wallet.balance()
    ```

=== "Kotlin"

    ``` kotlin
    val esploraClient: EsploraClient = EsploraClient("https://mutinynet.com/api")
    val syncRequest = wallet.startSyncWithRevealedSpks()
    val update = try esploraClient.sync(
        syncRequest,
        5uL
    )
    wallet.applyUpdate(update)
    val balance = wallet.balance()
    ```