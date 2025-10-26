// swift-tools-version:6.2
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-plugin-foundation-models",
    platforms: [
        // swiftPM is not compatible with v26 yet
        .macOS(.v15),
        .iOS(.v18),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-foundation-models",
            type: .static,
            targets: ["tauri-plugin-foundation-models"]),
    ],
    dependencies: [
        .package(url: "https://github.com/Brendonovich/swift-rs", from: "1.0.7")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-foundation-models",
            dependencies: [
                .product(name: "SwiftRs", package: "swift-rs")
            ],
            path: "Sources")
    ]
)
