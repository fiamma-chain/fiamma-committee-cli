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
- `--validator-key`: The new validator address for the fiamma chain, you can get the validator address refer to [become a validator](https://docs.fiammachain.io/our-product-suite/bitvm-powered-zkp-verification-layer/developer-guides/run-a-fiamma-node/become-a-validator).
- `--proof-id`: The proof ID for the challenge process, we provide a test proof id `1735e881fa5e58408e4710a4e8cbea0a7995f029eefdf85d7e59775b0b6c44c5` for you to challenge, you can use it to test the challenge process.
- `--vk-path`: The path to the verification key for the challenge process, you can get it from the fiamma committee cli repository [vk.bitvm](https://github.com/fiamma-chain/fiamma-committee-cli/blob/main/vk.bitvm).
- `--circuit-type`: The circuit type for the challenge process. Currently, only `groth16` is supported.
- `--script-index`: This is the bitvm2 challenge program split script index, this number is fixed and cannot be modified at present.
- `--reward-address`: The reward signet btc address for the disprove process, if you challenge success, you will get the reward.
- `--txid`: The signet btc transaction ID for the registration process.
- `--vout`: The signet btc output index for the registration process.
   you can get the txid and vout use the following command:
   ```bash
   curl -sSL "https://mempool.space/signet/api/address/{signet_btc_address}/utxo"
   ```

### Register as a BitVM2 Staker/Validator

If you want to run a fiamma node as a validator, you first need to register as a bitvm2 staker using fcli. The registration process requires the use of the following 2 commands to complete.

#### 1.Start the registration process

```
fcli register --network testnet start --validator-key <VALIDATOR_KEY> --txid <TXID> --vout <VOUT> --private-key <PRIVATE_KEY>
```

after executing the above command, you will get a registration number, the committee will generate some tx and bitvm2 challenge scripts, it will take about 5 minutes.
#### 2.Finish the registration process

```
fcli register --network testnet finish --validator-key <VALIDATOR_KEY> --private-key <PRIVATE_KEY>
```

after executing the above command, the register tx will be broadcasted to the bitcoin network, it will take about 10 minutes for the registration to be complete depending on the bitcoin network.

after the registration is complete, you can become a validator.
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

#### 3. Monitor the challenge process

After you execute the challenge finish command, you can monitor the challenge process by the following command:

```
fcli challenge --network testnet info --proof-id <PROOF_ID> --vk-path <VK_PATH> --circuit-type groth16
```

You can see that the console will print out the challenge tx id and committe generated assert tx id and the challenge status
You should use the tx id to query the [signet explorer](https://mempool.space/signet/tx/d81eccdca492ad1c9e9b4e9dd48fb181eb566bed2949d3b8f13d28ff015e489b) to see if the challenge tx and assert tx is confirmed. 

#### 4.Disprove the challenge

After executing the challenge finish command, if you challenge success, you will get the reward, you can use the following command to create the disprove tx and broadcast it to the bitcoin network:

```
fcli disprove --network testnet create_disprove_tx --proof-id <PROOF_ID> --script-index 977 --reward-address <REWARD_ADDRESS> 
```

You should wait until the disprove tx is confirmed, after the disprove tx is confirmed, you will get the reward to your reward address.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
