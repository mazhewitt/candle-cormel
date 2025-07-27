// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "objc-coreml-test",
    platforms: [.macOS(.v10_15)],
    products: [
        .executable(name: "objc-test", targets: ["objc-test"]),
    ],
    targets: [
        .executableTarget(
            name: "objc-test",
            dependencies: [],
            path: "Sources"
        )
    ]
)