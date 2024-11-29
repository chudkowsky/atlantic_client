use crate::{
    error::AtlanticSdkError,
    models::{AtlanticSdk, Layout, ProverVersion, QueryResponse},
};
use reqwest::multipart;

impl AtlanticSdk {
    pub async fn l2_atlantic_query(
        &self,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
        prover: ProverVersion,
        mock_fact_hash: bool,
    ) -> Result<QueryResponse, AtlanticSdkError> {
        let form = multipart::Form::new()
            .part(
                "programFile",
                multipart::Part::bytes(program_file)
                    .file_name("program.json")
                    .mime_str("application/json")?,
            )
            .part(
                "inputFile",
                multipart::Part::bytes(input_file)
                    .file_name("input.json")
                    .mime_str("application/json")?,
            )
            .text("cairoVersion", 0.to_string())
            .text("prover", prover.to_string())
            .text("mockFactHash", mock_fact_hash.to_string());
        let client = reqwest::Client::new();
        let url = format!("{}?apiKey={}", self.l2.atlantic_query, self.api_key);
        let response = client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;
        Ok(response)
    }

    //Works
    pub async fn l2_proof_generation_to_proof_verification(
        &self,
        pie_file: Vec<u8>,
        layout: Layout,
        prover: ProverVersion,
        mock_fact_hash: bool,
    ) -> Result<QueryResponse, AtlanticSdkError> {
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
    ) -> Result<QueryResponse, AtlanticSdkError> {
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
