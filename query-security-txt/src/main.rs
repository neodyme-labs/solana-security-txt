use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    pubkey::Pubkey,
};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "mainnet-beta")]
    /// The rpc endpoint to connect to. Either a url or one of the following: [mainnet-beta (m), testnet (t), devnet (d), localhost (l)]
    url: String,
    #[clap()]
    /// The program to query
    program: Pubkey,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let url = match args.url.as_str() {
        "mainnet-beta" | "m" => "https://api.mainnet-beta.solana.com",
        "testnet" | "t" => "https://api.testnet.solana.com",
        "devnet" | "d" => "https://api.devnet.solana.com",
        "localhost" | "l" => "http://localhost:8899",
        s => s,
    };
    let client = RpcClient::new(url);

    let program_account = client
        .get_account(&args.program)
        .map_err(|_| eyre!("Couldn't fetch program account"))?;

    if !bpf_loader_upgradeable::check_id(&program_account.owner) {
        return Err(eyre!(
            "Only accounts owned by the bpf_loader_upgradeable program are supported at the moment"
        ));
    }

    let program: UpgradeableLoaderState = bincode::deserialize_from(&program_account.data[..])
        .map_err(|_| eyre!("Couldn't deserialize program data"))?;

    let program_data_address = if let UpgradeableLoaderState::Program {
        programdata_address,
    } = program
    {
        programdata_address
    } else {
        return Err(eyre!("Wrong program account type"));
    };

    let program_data_account = client
        .get_account(&program_data_address)
        .map_err(|_| eyre!("Couldn't fetch program data account"))?;

    let offset = UpgradeableLoaderState::programdata_data_offset()?;
    if program_data_account.data.len() < offset {
        return Err(eyre!("Account is too small to be a program data account"));
    }
    let program_data = &program_data_account.data[offset..];

    let security_txt = solana_security_txt::find_and_parse(program_data)?;
    println!("{}", security_txt);

    Ok(())
}
