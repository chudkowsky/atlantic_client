use clap::Parser;
use herodotus_sharp_playground::{SharpSdk, SharpSdkError};
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
    let sharp_sdk = SharpSdk {
        api_key: args.api_key,
    };

    let is_alive = sharp_sdk.get_is_alive().await?;
    if !is_alive {
        error!("Sharp API is not alive");
        return Ok(());
    }

    let is_proof_generation = sharp_sdk
        .proof_generation(args.layout, args.is_offchain_proof, args.pie_file_path)
        .await?;

    let proof_path = loop {
        let status = sharp_sdk
            .get_sharp_query_jobs(is_proof_generation.sharp_query_id.clone())
            .await?;

        if let Some(context) = &status.jobs[0].context {
            if let Some(proof_path) = &context.proof_path {
                info!("Status: {}", status.jobs[0].status);
                break proof_path.clone();
            }
        }
        info!("Current status: {}", status.jobs[0].status);
        info!("Waiting for proof generation to complete...");
        tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
    };

    let proof = sharp_sdk.get_proof(proof_path).await?;
    info!("Program output: {:?}", proof.program_output);
    info!("Program hash: {:?}", proof.program_hash);
    info!("Program output hash: {:?}", proof.program_output_hash);
    info!("Serialized proof lenght: {:?}", proof.serialized_proof.len());
    fs::write("proof.json", serde_json::to_string(&proof)?)?;
    Ok(())
}
