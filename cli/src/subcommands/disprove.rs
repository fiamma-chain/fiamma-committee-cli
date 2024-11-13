use clap::Parser;
use types::disprove::DisproveRequest;
use wallet::{provider::ProviderParams, signer::Signer, Wallet};
use web3_decl::jsonrpsee::http_client::HttpClientBuilder;

#[derive(Debug, Parser, Clone)]
#[clap(name = "Disprove", about = "CLI for Fiamma validator disprove")]
pub struct Disprove {
    #[clap(
        short,
        long,
        help = "Network name, can be local or dev",
        default_value = "local"
    )]
    network: Option<String>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Parser, Clone)]
pub enum Action {
    #[clap(name = "create_disprove_tx")]
    CreateDisproveTx(DisproveTxParams),
}

#[derive(Debug, Parser, Clone)]
pub struct DisproveTxParams {
    #[clap(short, long, help = "Proof id of challenged proof")]
    proof_id: String,
    #[clap(short, long, help = "Index of the disprove script")]
    script_index: usize,
    #[clap(short, long, help = "Reward address of the disprove")]
    reward_address: String,
}

impl Disprove {
    pub async fn run(self) -> anyhow::Result<()> {
        let ctx = if let Some(network) = self.network {
            match network.as_str() {
                "local" => ProviderParams::local(),
                "dev" => ProviderParams::dev(),
                "testnet" => ProviderParams::testnet(),
                _ => {
                    anyhow::bail!("invalid network name")
                }
            }
        } else {
            ProviderParams::local()
        };

        let client = HttpClientBuilder::default()
            .build(ctx.http_endpoint.as_str())
            .unwrap();
        let signer = Signer::new();
        let wallet = Wallet::new(client, signer);

        match self.action {
            Action::CreateDisproveTx(args) => {
                let request =
                    DisproveRequest::new(&args.proof_id, args.script_index, &args.reward_address);
                wallet
                    .disprove(request)
                    .await
                    .expect("Failed to challenge a proof");
                println!(
                    "You have send a disprove request with proof_id {:?}, please wait for the result",
                    args.proof_id
                );
            }
        }
        Ok(())
    }
}
