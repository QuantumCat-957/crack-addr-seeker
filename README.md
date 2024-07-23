# Address Generator

This application generates Ethereum and TRON addresses based on a given mnemonic phrase and BIP39 password. It supports multi-threaded address generation and writes the generated addresses to CSV files. You can configure the maximum file size and log rotation interval via command-line arguments.

## Features

- Generates Ethereum and TRON addresses
- Multi-threaded address generation
- Writes addresses to CSV files
- Configurable maximum file size and log rotation interval
- Real-time logging of generated addresses

## Prerequisites

- Rust (1.70 or later)
- Cargo

## Installation

Clone the repository and build the application using Cargo:

```bash
git clone <repository_url>
cd <repository_directory>
cargo build --release
```

## Usage

The application requires the following command-line arguments:

- `--phrase`: The BIP39 mnemonic phrase (default: `"fan swamp loop mesh enact tennis priority artefact canal hour skull joy"`).
- `--password`: The BIP39 password (default: `""`).
- `--max-file-size`: The maximum size of the output CSV file in bytes (default: `104857600` or 100 MB).
- `--rotation-interval-secs`: The interval for rotating the CSV file in seconds (default: `3600` or 1 hour).

### Example

To run the application with a custom phrase, password, file size, and rotation interval:

```bash
./target/release/address-generator --phrase "your mnemonic phrase" --password "your password" --max-file-size 52428800 --rotation-interval-secs 1800
```

### Command-Line Arguments

- `--phrase`  
  Type: `String`  
  Description: The BIP39 mnemonic phrase used for address generation.

- `--password`  
  Type: `String`  
  Description: The BIP39 password used for address generation.

- `--max-file-size`  
  Type: `usize`  
  Description: The maximum size of the CSV file before rotating (in bytes).

- `--rotation-interval-secs`  
  Type: `u64`  
  Description: The time interval (in seconds) between log file rotations.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Clap](https://docs.rs/clap/latest/clap/index.html) for command-line argument parsing.
- [Tracing Subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html) for logging.

---

Feel free to adjust the example and details based on your specific needs and preferences.