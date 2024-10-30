pub mod error;
pub mod l1_sharp;
pub mod l2_sharp;
pub mod models;
use cairo_proof_parser::{
    json_parser::proof_from_annotations,
    output::ExtractOutputResult,
    program::{CairoVersion, ExtractProgramResult},
    ProofJSON,
};
use error::SharpSdkError;
use models::{JobResponse, ProverResult, SharpQueriesResponse, SharpQueryResponse, SharpSdk};
use tracing::info;

impl SharpSdk {
    pub async fn get_is_alive(&self) -> Result<bool, SharpSdkError> {
        info!("Checking if SHARP API is alive");
        let res = reqwest::get(self.health_check.is_alive.clone()).await?;
        Ok(res.status().is_success())
    }

    pub async fn get_sharp_query_jobs(
        &self,
        sharp_query_id: String,
    ) -> Result<JobResponse, SharpSdkError> {
        info!("Checking job status for sharpQueryId: {}", sharp_query_id);
        let url = format!("{}{}", self.sharp_queries.get_query_jobs, sharp_query_id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("accept", "application/json")
            .send()
            .await?;
        let response_text = response.text().await?;
        let response: JobResponse = serde_json::from_str(&response_text)?;
        Ok(response)
    }

    pub async fn get_sharp_query(
        &self,
        sharp_query_id: &str,
    ) -> Result<SharpQueryResponse, SharpSdkError> {
        let url = format!("{}{}", self.sharp_queries.get_query, sharp_query_id);
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(&[("apiKey", &self.api_key)])
            .send()
            .await?
            .json::<SharpQueryResponse>()
            .await?;
        Ok(response)
    }
    pub async fn get_sharp_queries(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<SharpQueriesResponse, SharpSdkError> {
        let mut query_params = vec![("apiKey", self.api_key.as_str())];
        let limit_str = limit.to_string();
        query_params.push(("limit", &limit_str));
        let offset_str = offset.to_string();
        query_params.push(("offset", &offset_str));

        let client = reqwest::Client::new();
        let response = client
            .get(self.sharp_queries.get_queries.clone())
            .query(&query_params)
            .send()
            .await?
            .json::<SharpQueriesResponse>()
            .await?;

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
    let ExtractProgramResult { program_hash, .. } =
        proof_from_annotations.extract_program(CairoVersion::Cairo0)?;
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
