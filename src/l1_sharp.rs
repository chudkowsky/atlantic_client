use crate::{
    error::AtlanticSdkError,
    models::{AtlanticSdk, CairoVersion, Layout, QueryResponse},
};
use reqwest::{multipart, Response};

impl AtlanticSdk {
    pub async fn submit_l1_atlantic_query(
        &self,
        program_hash: &str,
        program_file: Vec<u8>,
        input_file: Vec<u8>,
        cairo_version: CairoVersion,
        mock_fact_hash: bool,
    ) -> Result<Response, AtlanticSdkError> {
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
            .text("cairoVersion", cairo_version.to_string())
            .text("mockFactHash", mock_fact_hash.to_string());
        let client = reqwest::Client::new();
        let response = client
            .post(self.l1.atlantic_query.clone())
            .query(&[("apiKey", &self.api_key)])
            .multipart(form)
            .send()
            .await?;
        Ok(response)
    }

    pub async fn l1_proof_generation_verification(
        &self,
        pie_file: Vec<u8>,
        layout: Layout,
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
            .text("mockFactHash", mock_fact_hash.to_string());

        let client = reqwest::Client::new();
        let response = client
            .post(self.l1.proof_generation_verification.clone())
            .query(&[("apiKey", &self.api_key)]) // Add API key as a query parameter
            .multipart(form)
            .send()
            .await?
            .json::<QueryResponse>()
            .await?;
        Ok(response)
    }
}
