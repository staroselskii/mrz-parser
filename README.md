[![codecov](https://codecov.io/gh/staroselskii/mrz-parser/branch/main/graph/badge.svg)](https://codecov.io/gh/staroselskii/mrz-parser)

# MRZ Parser

A strict, cross-platform, compliance-friendly MRZ parsing library written in Rust.

# MRZ Parser

[![codecov](https://codecov.io/gh/staroselskii/mrz-parser/branch/main/graph/badge.svg)](https://codecov.io/gh/staroselskii/mrz-parser)

A strict, cross-platform, compliance-friendly MRZ parsing library written in Rust.

## Overview

`mrz-parser` is a robust, zero-dependency Rust library for parsing Machine Readable Zone (MRZ) codes from identity documents, including:

- Passports (TD3),
- ID cards (TD1),
- EU Driving Licenses,
- and more, with extensible format support.

## Features

- Zero unsafe code, no heap allocations in the core.
- Cross-platform (runs on embedded systems, servers, and desktop).
- Fully tested with real-world MRZ samples.
- Strict compliance with ICAO Doc 9303.
- Final check digit verification.
- Optional data support (e.g., nationality, sex, optional fields).
- Plugin-style format extensibility.

## Use Cases

Ideal for:

- Border control and e-gates.
- Airport check-in kiosks.
- eID/NFC scanning applications.
- Embedded access control systems.
- Compliance-driven integrations.

## Getting Started

Add to your `Cargo.toml`:

```toml
mrz-parser = "0.1"
```

Basic usage:

```rust
use mrz_parser::parse_any;

let mrz = "\
P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\n\
L898902C36UTO7408122F1204159ZE184226B<<<<<<";

match parse_any(mrz.as_bytes()) {
    Ok(parsed) => println!("{:?}", parsed),
    Err(e) => eprintln!("Error parsing MRZ: {:?}", e),
}
```

## Integration

For systems requiring C or FFI support, a C-compatible API can be provided via `mrz-host`. Contact us if your project needs stable bindings.

## License

MIT