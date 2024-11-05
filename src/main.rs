use clap::Parser;
use herodotus_sharp_playground::{
    error::SharpSdkError,
    models::{ProverVersion, SharpSdk},
};
use std::{fs, thread::sleep, time::Duration};
use tracing::{error, info};
use tracing_subscriber::{self};

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
    let id = sharp_sdk
        .proof_generation(pie_file, "all_cairo", ProverVersion::Starkware)
        .await?
        .sharp_query_id;
    info!("ID: {:?}", id);
    sleep(Duration::from_secs(5));
    let mut status = sharp_sdk.get_sharp_query_jobs(&id).await?;
    println!("Status: {:?}", status);
    while status.jobs[0].status != "COMPLETED" {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        status = sharp_sdk.get_sharp_query_jobs(&id).await?;
        println!("Status: {:?}", status);
    }
    let proof_path = status.jobs[0].context.clone().unwrap().proof_path.unwrap();
    let proof = sharp_sdk.get_proof(proof_path).await?;
    let proof = format!("{{\n\t\"proof\": {}\n}}", proof);
    fs::write("proof_wrapped.json", proof.clone()).unwrap();
    let program_file = include_bytes!("../cairo_verifier.json");

    let query = sharp_sdk
        .l2_atlantic_query(
            program_file.to_vec(),
            proof.as_bytes().to_vec(),
            ProverVersion::Starkware,
            false,
        )
        .await?;
    dbg!(query);
    Ok(())
}
