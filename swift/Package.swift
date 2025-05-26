// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "MRZParserFFI",
    platforms: [
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "MRZParserFFI",
            targets: ["MRZParserFFI"]
        ),
    ],
    targets: [
        .binaryTarget(
			name: "mrz_parserFFI",
			path: "MRZParserFFI.xcframework"
		),
		.target(
			name: "MRZParserWrapper",
			dependencies: ["MRZParserFFI"],
			path: "Sources/MRZParserWrapper"
		),
        .target(
            name: "MRZParserFFI",
            dependencies: [
				.target(name: "mrz_parserFFI")
			],
            path: "Sources/MRZParserFFI",
            exclude: [],
            resources: [],
            publicHeadersPath: "include", // for SwiftPM header exposure
            cSettings: [
                .headerSearchPath("include")
            ],
            linkerSettings: [
                .linkedLibrary("mrz_parser", .when(platforms: [.macOS])),
            ]
        ),
        .testTarget(
            name: "MRZParserFFITests",
            dependencies: ["MRZParserWrapper"]
        ),
    ]
)
