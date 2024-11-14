# Fiamma Committee CLI

Fiamma Committee CLI is a BTC Staking Command Line Tool for Validator Registration and BITVM2 Challenges on the Fiamma Chain.

## Installation

### From GitHub Releases

1. Visit the [GitHub Releases page](https://github.com/fiamma-chain/fiamma-committee-cli/releases).
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
git clone https://github.com/fiamma-chain/fiamma-committee-cli.git
cd fiamma-committee-cli
```

2. Build the project:
```
cargo build --release
```

3. After building, you can find the binary in the `target/release` directory.


## Usage

### Explanation of All Command Parameters
- `--network`: The network to use. Currently, only `testnet` is supported.
- `--private-key`: The signet btc private key for the validator.
- `--validator-key`: The new validator address for the fiamma chain, you can get the validator address refer to https://docs.fiammachain.io/our-product-suite/bitvm-powered-zkp-verification-layer/developer-guides/run-a-fiamma-node/become-a-validator.
- `--txid`: The signet btc transaction ID for the registration process.
- `--vout`: The signet btc output index for the registration process.
- `--proof-id`: The proof ID for the challenge process.
- `--vk-path`: The path to the verification key for the challenge process, you can get it from the fiamma committee cli repository [vk.bitvm](https://github.com/fiamma-chain/fiamma-committee-cli/blob/main/vk.bitvm).
- `--circuit-type`: The circuit type for the challenge process. Currently, only `groth16` is supported.
- `--script-index`: This is the bitvm2 challenge program split script index, this number cannot be modified at present.
- `--reward-address`: The reward signet btc address for the disprove process, if you challenge success, you will get the reward.

### Register as a BitVM2 Staker/Validator

If you want to run a fiamma node as a validator, you first need to register as a bitvm2 staker using fcli. The registration process requires the use of the following 2 commands to complete.

#### 1.Start the registration process

```
fcli register --network testnet start --validator-key <VALIDATOR_KEY> --txid <TXID> --vout <VOUT> --private-key <PRIVATE_KEY>
```

after executing the above command, you will get a registration number, please wait patiently until the registration is complete. it will take about 10 minutes depending on the bitcoin network.

#### 2.Finish the registration process

```
fcli register --network testnet finish --validator-key <VALIDATOR_KEY> --private-key <PRIVATE_KEY>
```

### Challenge Proofs

If you want to challenge a proof , you can use the following command:

#### 1.Start the challenge process

```
fcli challenge --network testnet start --proof-id <PROOF_ID> --vk-path <VK_PATH> --circuit-type groth16
```

#### 2.Finish the challenge process

```
fcli challenge --network testnet finish --proof-id <PROOF_ID> --vk-path <VK_PATH> --circuit-type groth16 --txid <TXID> --vout <VOUT> --private-key <PRIVATE_KEY>
```

#### 3.Disprove the challenge

```
fcli disprove --network testnet create_disprove_tx --proof-id <PROOF_ID> --script-index 977 --reward-address <REWARD_ADDRESS> 
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
