pub mod error;
pub mod l1_sharp;
pub mod l2_sharp;
pub mod models;
pub mod proof_gen_trace_gen;

use error::AtlanticSdkError;
use models::{AtlanticSdk, JobResponse, SharpQueriesResponse, SharpQueryResponse};
use tracing::info;

impl AtlanticSdk {
    pub async fn get_is_alive(&self) -> Result<bool, AtlanticSdkError> {
        info!("Checking if SHARP API is alive");
        let res = reqwest::get(self.health_check.is_alive.clone()).await?;
        Ok(res.status().is_success())
    }

    pub async fn get_sharp_query_jobs(
        &self,
        sharp_query_id: &str,
    ) -> Result<JobResponse, AtlanticSdkError> {
        info!("Checking job status for sharpQueryId: {}", sharp_query_id);
        let url = format!("{}{}", self.atlantic_queries.get_query_jobs, sharp_query_id);
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
    ) -> Result<SharpQueryResponse, AtlanticSdkError> {
        let url = format!("{}{}", self.atlantic_queries.get_query, sharp_query_id);
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
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SharpQueriesResponse, AtlanticSdkError> {
        let mut query_params = vec![("apiKey", self.api_key.as_str())];
        let mut optional_params = std::collections::HashMap::new();

        if let Some(limit) = limit {
            optional_params.insert("limit", limit.to_string());
        }
        if let Some(offset) = offset {
            optional_params.insert("offset", offset.to_string());
        }
        // Populate query_params with references
        for (key, value) in &optional_params {
            query_params.push((key, value.as_str()));
        }

        let client = reqwest::Client::new();
        let response = client
            .get(self.atlantic_queries.get_queries.clone())
            .query(&query_params)
            .send()
            .await?
            .json::<SharpQueriesResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_proof(&self, query_id: String) -> Result<String, AtlanticSdkError> {
        let url = format!(
            "https://atlantic-queries.s3.nl-ams.scw.cloud/sharp_queries/query_{}/proof.json",
            query_id
        );
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }
}
