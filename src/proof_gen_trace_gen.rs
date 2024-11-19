use reqwest::multipart;

use crate::{
    error::AtlanticSdkError,
    models::{Layout, ProverVersion, QueryResponse},
};

use super::AtlanticSdk;
impl AtlanticSdk {
    pub async fn trace_generation(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
    ) -> Result<QueryResponse, AtlanticSdkError> {
        let form = multipart::Form::new()
            .text("programHash", program_hash.to_string())
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
            .text("cairoVersion", 0.to_string());

        let client = reqwest::Client::new();
        let url = format!(
            "{}?apiKey={}",
            self.proof_generation_trace_generation.trace_generation, self.api_key
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

    pub async fn trace_gen_to_proof_gen(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
        prover: ProverVersion,
    ) -> Result<QueryResponse, AtlanticSdkError> {
        let form = multipart::Form::new()
            .text("programHash", program_hash.to_string())
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
            .text("prover", prover.to_string());

        let client = reqwest::Client::new();
        let url = format!(
            "{}?apiKey={}",
            self.proof_generation_trace_generation
                .trace_gen_to_proof_gen,
            self.api_key
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
    pub async fn proof_generation(
        &self,
        pie_file: Vec<u8>,
        layout: Layout,
        prover: ProverVersion,
    ) -> Result<QueryResponse, AtlanticSdkError> {
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
        let url = format!(
            "{}?apiKey={}",
            self.proof_generation_trace_generation.proof_generation, self.api_key
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
}
