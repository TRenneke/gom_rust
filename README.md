# zeiss_inspect_api_rust-2026.3.0.343/README.md

# Zeiss Inspect API Rust

This project provides a Rust library that can be used as a Python extension via PyO3. It includes a simple "Hello World" function that demonstrates how to expose Rust functionality to Python.

## Project Structure

- `Cargo.toml`: Configuration file for the Rust project.
- `pyproject.toml`: Python packaging configuration.
- `README.md`: Documentation for the project.
- `.gitignore`: Specifies files to be ignored by Git.
- `src/lib.rs`: Contains the main Rust library code.
- `tests/hello.rs`: Contains tests for the "Hello World" function.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Python installed (preferably Python 3.6 or later).
- `maturin` for building the Python package.

### Building the Project

To build the Rust library and create the Python extension, run the following command in the project root:

```bash
maturin develop
```

### Using the Library in Python

Once the library is built, you can use it in your Python code as follows:

```python
import zeiss_inspect_api_rust

print(zeiss_inspect_api_rust.hello_world())
```

### Running Tests

To run the tests for the "Hello World" function, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.