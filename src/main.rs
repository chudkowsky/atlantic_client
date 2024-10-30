use clap::Parser;
use herodotus_sharp_playground::{error::SharpSdkError, models::{ProverVersion, SharpSdk}};
use std::fs;
use tracing::{error, info};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env)]
    api_key: String,
    #[arg(short, long, env)]
    pie_file_path: String,
    #[arg(short, long, env)]
    layout: String,
    #[arg(short, long)]
    is_offchain_proof: bool,
}

#[tokio::main]
async fn main() -> Result<(), SharpSdkError> {
    tracing_subscriber::fmt().with_target(false).init();
    let args = Args::parse();
    let sharp_sdk = SharpSdk::new(args.api_key, "https://sharp.api.herodotus.cloud")?;

    let is_alive = sharp_sdk.get_is_alive().await?;
    if !is_alive {
        error!("Sharp API is not alive");
        return Ok(());
    }
    let pie_file = fs::read(&args.pie_file_path)?;
    let response = sharp_sdk.l1_proof_generation_verification(pie_file, &args.layout,true).await?;
    info!("Response: {:?}", response);
    let status = sharp_sdk
        .get_sharp_query_jobs(response.sharp_query_id.clone())
        .await?;
    println!("Status: {:?}", status);
    let test = sharp_sdk
        .get_sharp_query(response.sharp_query_id.as_str())
        .await?;
    println!("Test: {:?}", test);

    let test2 = sharp_sdk.get_sharp_queries(0, 0).await?;
    println!("Test2: {:?}", test2);
    Ok(())
}
