use serde::{Deserialize, Serialize};
use starknet_types_core::felt::Felt;
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
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
pub struct JobResponse {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct SharpQueryDetails {
    id: String,
    #[serde(rename = "submittedByClient")]
    submitted_by_client: String,
    status: String,
    step: Option<String>,
    #[serde(rename = "programHash")]
    program_hash: Option<String>,
    layout: Option<String>,
    #[serde(rename = "programFactHash")]
    program_fact_hash: Option<String>,
    price: String,
    #[serde(rename = "creditsUsed")]
    credits_used: usize,
    #[serde(rename = "isFactMocked")]
    is_fact_mocked: bool,
    prover: Option<String>,
    chain: String,
    steps: Vec<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "completedAt")]
    completed_at: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SharpQueryResponse {
    #[serde(rename = "sharpQuery")]
    sharp_query: SharpQueryDetails,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SharpQueriesResponse {
    #[serde(rename = "sharpQueries")]
    sharp_queries: Vec<SharpQueryDetails>, // List of SharpQueryDetails
    total: usize, // Total count of queries
}
pub enum ProverVersion {
    Starkware,
    Herodotus,
}
impl ProverVersion {
    pub fn to_string(&self) -> String {
        match self {
            ProverVersion::Starkware => "starkware_sharp".to_string(),
            ProverVersion::Herodotus => "herodotus_stone".to_string(),
        }
    }
}
#[derive(Debug, Clone)]

pub struct SharpSdk {
    pub api_key: String,
    pub l1: L1Endpoints,
    pub l2: L2Endpoints,
    pub sharp_queries: SharpQueriesEndpoints,
    pub health_check: HealthCheckEndpoint,
    pub program_registry: ProgramRegistryEndpoint,
}
#[derive(Debug, Clone)]
pub struct L1Endpoints {
    pub atlantic_query: Url,
    pub trace_generation: Url,
    pub proof_generation_verification: Url,
}
#[derive(Debug, Clone)]
pub struct L2Endpoints {
    pub atlantic_query: Url,
    pub trace_generation: Url,
    pub from_trace_to_proof_generation: Url,
    pub proof_generation: Url,
    pub from_proof_generation_to_proof_verification: Url,
    pub proof_verification: Url,
}
#[derive(Debug, Clone)]
pub struct SharpQueriesEndpoints {
    pub get_queries: Url,
    pub get_query: Url,
    pub get_query_jobs: Url,
}
#[derive(Debug, Clone)]
pub struct HealthCheckEndpoint {
    pub is_alive: Url,
}
#[derive(Debug, Clone)]
pub struct ProgramRegistryEndpoint {
    pub submit_program: Url,
}

impl SharpSdk {
    pub fn new(api_key: String, base_url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            api_key,
            l1: L1Endpoints {
                atlantic_query: Url::parse(&format!("{}/l1/atlantic-query", base_url))?,
                trace_generation: Url::parse(&format!(
                    "{}/l1/atlantic-query/trace_generation",
                    base_url
                ))?,
                proof_generation_verification: Url::parse(&format!(
                    "{}/l1/atlantic-query/proof_generation_verification",
                    base_url
                ))?,
            },
            l2: L2Endpoints {
                atlantic_query: Url::parse(&format!("{}/l2/atlantic-query", base_url))?,
                trace_generation: Url::parse(&format!(
                    "{}/l2/atlantic-query/trace-generation",
                    base_url
                ))?,
                from_trace_to_proof_generation: Url::parse(&format!(
                    "{}/l2/atlantic-query/from-trace-generation-to-proof-generation",
                    base_url
                ))?,
                proof_generation: Url::parse(&format!("{}/proof-generation", base_url))?,
                from_proof_generation_to_proof_verification: Url::parse(&format!(
                    "{}/l2/atlantic-query/from-proof-generation-to-proof-verification",
                    base_url
                ))?,
                proof_verification: Url::parse(&format!(
                    "{}/l2/atlantic-query/proof-verification",
                    base_url
                ))?,
            },
            sharp_queries: SharpQueriesEndpoints {
                get_queries: Url::parse(&format!("{}/atlantic-queries", base_url))?,
                get_query: Url::parse(&format!("{}/atlantic-query/", base_url))?,
                get_query_jobs: Url::parse(&format!("{}/atlantic-query-jobs/", base_url))?,
            },
            health_check: HealthCheckEndpoint {
                is_alive: Url::parse(&format!("{}/is-alive", base_url))?,
            },
            program_registry: ProgramRegistryEndpoint {
                submit_program: Url::parse(&format!("{}/submit-program", base_url))?,
            },
        })
    }
}
