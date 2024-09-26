pub mod models;

use cairo_proof_parser::{
    json_parser::proof_from_annotations,
    output::ExtractOutputResult,
    program::{CairoVersion, ExtractProgramResult},
    ProofJSON,
};
use models::{ProverResult, Response, SharpQueryResponse};
use reqwest::multipart;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum SharpSdkError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
}

pub struct SharpSdk {
    pub api_key: String,
}

impl SharpSdk {
    pub async fn get_is_alive(&self) -> Result<bool, SharpSdkError> {
        info!("Checking if SHARP API is alive");
        let res = reqwest::get("https://sharp.api.herodotus.cloud/is-alive").await?;
        Ok(res.status().is_success())
    }

    pub async fn proof_generation(
        &self,
        layout: String,
        is_offchain_proof: bool,
        pie_file_path: String,
    ) -> Result<SharpQueryResponse, SharpSdkError> {
        info!(
            "Starting proof generation: layout = {}, is_offchain_proof = {}",
            layout, is_offchain_proof
        );
        let url = format!(
            "https://sharp.api.herodotus.cloud/submit-sharp-query/proof_generation?apiKey={}",
            self.api_key
        );
        let client = reqwest::Client::new();

        let mut file = File::open(&pie_file_path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;
        info!("Read pie file: {}", pie_file_path);
        let form = multipart::Form::new()
            .part(
                "pieFile",
                multipart::Part::bytes(buffer)
                    .file_name("pie.zip")
                    .mime_str("application/zip")?,
            )
            .text("layout", layout.clone())
            .text("isOffchainProof", is_offchain_proof.to_string());

        let response = client
            .post(&url)
            .header("accept", "application/json")
            .multipart(form)
            .send()
            .await?;

        let response_text = response.text().await?;
        Ok(serde_json::from_str(&response_text)?)
    }

    pub async fn get_sharp_query_jobs(
        &self,
        sharp_query_id: String,
    ) -> Result<Response, SharpSdkError> {
        info!("Checking job status for sharpQueryId: {}", sharp_query_id);
        let url = format!(
            "https://sharp.api.herodotus.cloud/sharp-query-jobs/{}",
            sharp_query_id
        );
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", "application/json")
            .send()
            .await?;
        let response_text = response.text().await?;
        let response: Response = serde_json::from_str(&response_text)?;
        Ok(response)
    }

    pub async fn get_proof(&self, proof_path: String) -> Result<ProverResult, SharpSdkError> {
        let url = format!("https://sharp.api.herodotus.cloud/{}", proof_path);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let response_text = response.text().await?;
        let proof = prover_result(response_text)?;
        Ok(proof)
    }
}
pub fn prover_result(proof: String) -> Result<ProverResult, SharpSdkError> {
    let proof_json = serde_json::from_str::<ProofJSON>(&proof)?;
    let proof_from_annotations = proof_from_annotations(proof_json)?;
    let ExtractProgramResult { program_hash, .. } = proof_from_annotations.extract_program(CairoVersion::Cairo0)?;
    let ExtractOutputResult {
        program_output,
        program_output_hash,
    } = proof_from_annotations.extract_output()?;
    let serialized_proof = proof_from_annotations.to_felts();
    let prover_result = ProverResult {
        proof: proof.clone(),
        program_hash,
        program_output,
        program_output_hash,
        serialized_proof,
    };
    Ok(prover_result)
}