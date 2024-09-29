# Fiamma Committee CLI

Fiamma Committee CLI is a BTC Staking Command Line Tool for BITVM2 Challenges on the Fiamma Chain.

## Installation

### From GitHub Releases

1. Visit the [GitHub Releases page](https://github.com/your-repo/fiamma-committee-cli/releases).
2. Download the binary file for your operating system:
   - Linux x86-64: `fcli-linux-x86-64`
   - macOS Intel: `fcli-mac-intel`
   - macOS Apple Silicon (M1/M2): `fcli-mac-arm`
   - Windows: `fcli-windows.exe`

3. Rename the downloaded file:
   - On Linux and macOS, rename to `fcli`
   - On Windows, rename to `fcli.exe`

4. Move the renamed file to a directory in your PATH.

For example, on Linux or macOS:

```bash
mv fcli-linux-x86-64 fcli # or fcli-mac-intel or fcli-mac-arm   
chmod +x fcli
sudo mv fcli /usr/local/bin/
```


On Windows, move `fcli.exe` to a directory in your PATH, such as `C:\Windows\System32\`.

### Building from Source

To build the CLI from source, ensure you have Rust and Cargo installed. Then follow these steps:

1. Clone the repository:
```
git clone https://github.com/your-repo/fiamma-committee-cli.git
cd fiamma-committee-cli
```

2. Build the project:
```
cargo build --release
```

3. After building, you can find the binary in the `target/release` directory.

## Usage

If you want to run a fiamma node as a validator, you first need to register as a bitvm2 staker using fcli. The registration process requires the use of the following 2 commands to complete.

```
fcli register start --validator-key <VALIDATOR_KEY> --txid <TXID> --vout <VOUT> --private-key <PRIVATE_KEY>
```

```
fcli register finish --validator-key <VALIDATOR_KEY> --txid <TXID> --vout <VOUT> --private-key <PRIVATE_KEY>
```


## Key Features

- Signer management
- Challenge proofs
- Transaction commands
- Disprove commands
- Register commands



## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
