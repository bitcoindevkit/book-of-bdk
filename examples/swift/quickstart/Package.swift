import PackageDescription

let package = Package(
    name: "QuickstartExample",
    platforms: [
        .macOS(.v12)
    ],
    dependencies: [
        .package(url: "https://github.com/bitcoindevkit/bdk-swift", from: "0.30.0")
    ],
    targets: [
        .executableTarget(
            name: "QuickstartExample",
            dependencies: [
                .product(name: "BitcoinDevKit", package: "bdk-swift")
            ],
            path: "Sources"
        )
    ]
)