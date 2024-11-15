use std::fmt::Display;

use serde::{Deserialize, Serialize};
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
    #[serde(rename = "sharpQueryId")]
    pub sharp_query_id: String,
    pub status: String,
    #[serde(rename = "jobName")]
    pub job_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
    pub context: Option<Context>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JobResponse {
    pub jobs: Vec<Job>,
    pub steps: Vec<String>,
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
impl Display for ProverVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProverVersion::Starkware => write!(f, "starkware_sharp"),
            ProverVersion::Herodotus => write!(f, "herodotus_stone"),
        }
    }
}
pub enum Layout {
    StarknetWithKeccak,
    Recursive,
    Starknet,
    RecursiveWithPoseidon,
    Dex,
    Small,
    Dynamic,
}
impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layout::StarknetWithKeccak => write!(f, "starknet_with_keccak"),
            Layout::Recursive => write!(f, "recursive"),
            Layout::Starknet => write!(f, "starknet"),
            Layout::RecursiveWithPoseidon => write!(f, "recursive_with_poseidon"),
            Layout::Dex => write!(f, "dex"),
            Layout::Small => write!(f, "small"),
            Layout::Dynamic => write!(f, "dynamic"),
        }
    }
}
pub enum CairoVersion {
    Zero,
}
impl Display for CairoVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CairoVersion::Zero => write!(f, "0"),
        }
    }
}
#[derive(Debug, Clone)]

pub struct AtlanticSdk {
    pub api_key: String,
    pub l1: L1Endpoints,
    pub l2: L2Endpoints,
    pub proof_generation_trace_generation: ProofGenTraceGenEndpoints,
    pub atlantic_queries: AtlanticQueriesEndpoints,
    pub health_check: HealthCheckEndpoint,
    pub program_registry: ProgramRegistryEndpoint,
}
#[derive(Debug, Clone)]
pub struct L1Endpoints {
    pub atlantic_query: Url,
    pub proof_generation_verification: Url,
}
#[derive(Debug, Clone)]
pub struct L2Endpoints {
    pub atlantic_query: Url,
    pub from_proof_generation_to_proof_verification: Url,
    pub proof_verification: Url,
}
#[derive(Debug, Clone)]
pub struct ProofGenTraceGenEndpoints {
    pub trace_generation: Url,
    pub proof_generation: Url,
    pub trace_gen_to_proof_gen: Url,
}
#[derive(Debug, Clone)]
pub struct AtlanticQueriesEndpoints {
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

impl AtlanticSdk {
    pub fn new(api_key: String, base_url: Url) -> Result<Self, url::ParseError> {
        Ok(Self {
            api_key,
            l1: L1Endpoints {
                atlantic_query: base_url.join("/l1/atlantic-query")?,
                proof_generation_verification: base_url
                    .join("/l1/atlantic-query/proof-generation-verification")?,
            },
            l2: L2Endpoints {
                atlantic_query: base_url.join("/l2/atlantic-query")?,
                from_proof_generation_to_proof_verification: base_url
                    .join("/l2/atlantic-query/proof-generation-verification")?,
                proof_verification: base_url.join("/l2/atlantic-query/proof-verification")?,
            },
            proof_generation_trace_generation: ProofGenTraceGenEndpoints {
                trace_generation: base_url.join("/trace-generation")?,
                proof_generation: base_url.join("/proof-generation")?,
                trace_gen_to_proof_gen: base_url.join("/trace-generation-proof-generation")?,
            },
            atlantic_queries: AtlanticQueriesEndpoints {
                get_queries: base_url.join("/atlantic-queries")?,
                get_query: base_url.join("/atlantic-query/")?,
                get_query_jobs: base_url.join("/atlantic-query-jobs/")?,
            },
            health_check: HealthCheckEndpoint {
                is_alive: base_url.join("/is-alive")?,
            },
            program_registry: ProgramRegistryEndpoint {
                submit_program: base_url.join("/submit-program")?,
            },
        })
    }
}
