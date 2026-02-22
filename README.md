# zeiss_inspect_api_rust-2026.3.0.343/README.md

# Zeiss Inspect API Rust

This project provides a Rust implementation of the Zeiss Inspect API client library. It enables communication with Zeiss Inspect AUTOMATION servers using the GOM (Zeiss Object Model) protocol over WebSocket connections.

## Project Structure

- `Cargo.toml`: Configuration file for the Rust project.
- `README.md`: Documentation for the project.
- `src/lib.rs`: Contains the main Rust library code.
- `src/network.rs`: WebSocket communication and protocol handling.
- `src/encoding.rs`: GOM protocol encoding and decoding.
- `tests/`: Test suite for the library.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.

### Building the Project

To build the Rust library, run the following command in the project root:

```bash
cargo build
```

For a release build with optimizations:

```bash
cargo build --release
```

### Running Tests

To run the tests for the "Hello World" function, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.