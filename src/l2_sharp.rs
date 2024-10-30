use crate::{
    error::SharpSdkError,
    models::{ProverVersion, QueryResponse, SharpSdk},
};
use reqwest::multipart;

impl SharpSdk {
    pub async fn l2_submit_proof_generation(
        &self,
        pie_file: Vec<u8>, // PIE file as a byte array
        layout: &str,      // Layout parameter (e.g., "starknet_with_keccak")
        prover: &str,      // Prover type (e.g., "herodotus" or "starkware")
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .part(
                "pieFile",
                multipart::Part::bytes(pie_file)
                    .file_name("pie.zip")
                    .mime_str("application/zip")?,
            )
            .text("layout", layout.to_string())
            .text("prover", prover.to_string());

        let client = reqwest::Client::new();
        let url = format!("{}?apiKey={}", self.l2.proof_generation, self.api_key);
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }

    pub async fn l2_submit_sharp_query(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
        prover: ProverVersion,
        mock_fact_hash: bool,
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .text("programHash", program_hash.to_string())
            .part("programFile", multipart::Part::bytes(program_file))
            .part("inputFile", multipart::Part::bytes(input_file))
            .text("cairoVersion", 0.to_string())
            .text("prover", prover.to_string())
            .text("mockFactHash", mock_fact_hash.to_string());

        let client = reqwest::Client::new();
        let url = format!("{}?apiKey={}", self.l2.submit_sharp_query, self.api_key);
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }

    pub async fn l2_trace_generation(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .text("programHash", program_hash.to_string())
            .part("programFile", multipart::Part::bytes(program_file))
            .part("inputFile", multipart::Part::bytes(input_file))
            .text("cairoVersion", 0.to_string());

        let client = reqwest::Client::new();
        let url = format!("{}?apiKey={}", self.l2.trace_generation, self.api_key);
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }

    pub async fn l2_trace_generation_verification(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
        prover: ProverVersion,
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .text("programHash", program_hash.to_string())
            .part("programFile", multipart::Part::bytes(program_file))
            .part("inputFile", multipart::Part::bytes(input_file))
            .text("cairoVersion", 0.to_string())
            .text("prover", prover.to_string());

        let client = reqwest::Client::new();
        let url = format!(
            "{}?apiKey={}",
            self.l2.from_trace_to_proof_generation, self.api_key
        );
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }

    pub async fn l2_proof_generation_to_proof_verification(
        &self,
        pie_file: Vec<u8>,
        layout: &str,
        prover: ProverVersion,
        mock_fact_hash: bool,
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .part(
                "pieFile",
                multipart::Part::bytes(pie_file)
                    .file_name("pie.zip")
                    .mime_str("application/zip")?,
            )
            .text("layout", layout.to_string())
            .text("prover", prover.to_string())
            .text("mockFactHash", mock_fact_hash.to_string());

        let client = reqwest::Client::new();
        let url = format!(
            "{}?apiKey={}",
            self.l2.from_proof_generation_to_proof_verification, self.api_key
        );
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }

    pub async fn l2_proof_verification(
        &self,
        proof_file: Vec<u8>,
        mock_fact_hash: bool,
        stone_version: ProverVersion,
    ) -> Result<QueryResponse, SharpSdkError> {
        let form = multipart::Form::new()
            .part(
                "proofFile",
                multipart::Part::bytes(proof_file)
                    .file_name("proof.zip")
                    .mime_str("application/zip")?,
            )
            .text("mockFactHash", mock_fact_hash.to_string())
            .text("stoneVersion", stone_version.to_string());

        let client = reqwest::Client::new();
        let url = format!("{}?apiKey={}", self.l2.proof_verification, self.api_key);
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;

        Ok(response)
    }
}
