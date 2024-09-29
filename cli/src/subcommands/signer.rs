use clap::Parser;

#[derive(Debug, Parser)]
pub struct Signer {
    #[clap(long, help = "")]
    private_key: String,
    #[clap(long, help = "stake、assert、disprove")]
    tx_type: String,
}

impl Signer {
    pub async fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}
