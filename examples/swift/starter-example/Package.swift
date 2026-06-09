// swift-tools-version: 5.10

import PackageDescription

let package = Package(
    name: "QuickstartExample",
    platforms: [
        .macOS(.v12)
    ],
    dependencies: [
        .package(url: "https://github.com/bitcoindevkit/bdk-swift", from: "3.0.0")
    ],
    targets: [
        .executableTarget(
            name: "StarterExample",
            dependencies: [
                .product(name: "BitcoinDevKit", package: "bdk-swift")
            ],
            path: "Sources"
        )
    ]
)