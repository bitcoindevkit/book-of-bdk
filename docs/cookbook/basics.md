# Basics

Add BDK to a project

=== "Rust"

    ``` toml
    [dependencies]
    bdk = { version = "{VERSION}", features = ["key-value-db", "electrum"] }
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
Get Wallet Mnemonic

=== "Rust"

    // This includes error, might want to do something different here.
    ``` toml
    let mnemonic: GeneratedKey<_, Tap> =
    Mnemonic::generate((WordCount::Words12, Language::English))
        .map_err(|_| anyhow!("Mnemonic generation error"))?;
    ```


=== "Swift"

    ``` swift
    let mnemonic = try Mnemonic(wordCount: .words12)
    ```

=== "Kotlin"

    ``` kotlin
    val mnemonic = Mnemonic(WordCount.WORDS12)
    ```

Setup Wallet

=== "Rust"

    ``` toml
    TBD
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
    let wallet = try Wallet.init(
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

Create Address

=== "Rust"

    ``` toml
    TBD
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
    TBD
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