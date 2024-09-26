use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
#[derive(Serialize, Deserialize, Debug)]
pub struct SharpQueryResponse {
    #[serde(rename = "sharpQueryId")]
    pub sharp_query_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "proofPath")]
    pub proof_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: String,
    pub status: String,
    pub context: Option<Context>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response {
    pub jobs: Vec<Job>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ProverResult {
    pub proof: String,
    pub serialized_proof: Vec<Felt>,
    pub program_hash: Felt,
    pub program_output: Vec<Felt>,
    pub program_output_hash: Felt,
}
