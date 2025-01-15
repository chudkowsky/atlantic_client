use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::AtlanticSdkError;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
    #[serde(rename = "atlanticQueryId")]
    pub atlantic_query_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FactHashResponse {
    #[serde(rename = "factHash")]
    pub fact_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "proofPath")]
    pub proof_path: Option<String>,
    #[serde(rename = "piePath")]
    pub pie_path: Option<String>,
    #[serde(rename = "layout")]
    pub layout: Option<String>,
    #[serde(rename = "cairoVersion")]
    pub cairo_version: Option<usize>,
    #[serde(rename = "inputPath")]
    pub input_path: Option<String>,
    #[serde(rename = "programPath")]
    pub program_path: Option<String>,
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
    pub id: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "submittedByClient")]
    pub submitted_by_client: String,
    pub status: String,
    pub step: Option<String>,
    #[serde(rename = "programHash")]
    pub program_hash: Option<String>,
    pub layout: Option<String>,
    #[serde(rename = "programFactHash")]
    pub program_fact_hash: Option<String>,
    pub price: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: usize,
    #[serde(rename = "creditsUsed")]
    pub credits_used: usize,
    #[serde(rename = "traceCreditsUsed")]
    pub trace_credits_used: usize,
    #[serde(rename = "isFactMocked")]
    pub is_fact_mocked: Option<bool>,
    pub prover: Option<String>,
    pub chain: Option<String>,
    pub steps: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SharpQueryResponse {
    #[serde(rename = "atlanticQuery")]
    pub sharp_query: SharpQueryDetails,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SharpQueriesResponse {
    #[serde(rename = "sharpQueries")]
    pub sharp_queries: Vec<SharpQueryDetails>, // List of SharpQueryDetails
    pub total: usize, // Total count of queries
}
pub enum ProverVersion {
    Starkware,
}
impl Display for ProverVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProverVersion::Starkware => write!(f, "starkware_sharp"),
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
    Auto,
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
            Layout::Auto => write!(f, "auto"),
            Layout::Small => write!(f, "small"),
            Layout::Dynamic => write!(f, "dynamic"),
        }
    }
}
impl FromStr for Layout {
    type Err = AtlanticSdkError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "starknet_with_keccak" => Ok(Layout::StarknetWithKeccak),
            "recursive" => Ok(Layout::Recursive),
            "starknet" => Ok(Layout::Starknet),
            "recursive_with_poseidon" => Ok(Layout::RecursiveWithPoseidon),
            "dex" => Ok(Layout::Dex),
            "auto" => Ok(Layout::Auto),
            "small" => Ok(Layout::Small),
            "dynamic" => Ok(Layout::Dynamic),
            _ => Err(AtlanticSdkError::InvalidLayout),
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
    pub fact_hash_calculation: Url,
}
#[derive(Debug, Clone)]
pub struct L2Endpoints {
    pub atlantic_query: Url,
    pub from_proof_generation_to_proof_verification: Url,
    pub proof_verification: Url,
    pub fact_hash_calculation: Url,
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
                atlantic_query: base_url.join("/v1/l1/atlantic-query")?,
                proof_generation_verification: base_url
                    .join("/v1/l1/atlantic-query/proof-generation-verification")?,
                fact_hash_calculation: base_url.join("/v1/l1/fact-hash-calculation")?,
            },
            l2: L2Endpoints {
                atlantic_query: base_url.join("/v1/l2/atlantic-query")?,
                from_proof_generation_to_proof_verification: base_url
                    .join("/v1/l2/atlantic-query/proof-generation-verification")?,
                proof_verification: base_url.join("/v1/l2/atlantic-query/proof-verification")?,
                fact_hash_calculation: base_url.join("/v1/l2/fact-hash-calculation")?,
            },
            proof_generation_trace_generation: ProofGenTraceGenEndpoints {
                trace_generation: base_url.join("/v1/trace-generation")?,
                proof_generation: base_url.join("/v1/proof-generation")?,
                trace_gen_to_proof_gen: base_url.join("/v1/trace-generation-proof-generation")?,
            },
            atlantic_queries: AtlanticQueriesEndpoints {
                get_queries: base_url.join("/v1/atlantic-queries")?,
                get_query: base_url.join("/v1/atlantic-query/")?,
                get_query_jobs: base_url.join("/v1/atlantic-query-jobs/")?,
            },
            health_check: HealthCheckEndpoint {
                is_alive: base_url.join("/v1/is-alive")?,
            },
            program_registry: ProgramRegistryEndpoint {
                submit_program: base_url.join("/v1/submit-program")?,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_new() {
        let api_key = "api_key".to_string();
        let base_url = Url::parse("http://test_url:8080").unwrap();
        let sdk = AtlanticSdk::new(api_key.clone(), base_url.clone()).unwrap();
        assert_eq!(sdk.api_key, api_key);
        assert_eq!(
            sdk.l1.atlantic_query,
            Url::parse("http://test_url:8080/v1/l1/atlantic-query").unwrap()
        );
    }
    #[test]
    fn test_new_with_slash() {
        let api_key = "api_key".to_string();
        let base_url = Url::parse("http://test_url:8080/").unwrap();
        let sdk = AtlanticSdk::new(api_key.clone(), base_url.clone()).unwrap();
        assert_eq!(sdk.api_key, api_key);
        assert_eq!(
            sdk.l1.atlantic_query,
            Url::parse("http://test_url:8080/v1/l1/atlantic-query").unwrap()
        );
    }
    #[test]
    fn test_deserialize_jobs() {
        let response = r#"{
            "jobs": [
                {
                "id": "f01b3b3c-a35b-4ffb-a7ee-1c14334ab95c",
                "sharpQueryId": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                "status": "COMPLETED",
                "jobName": "TRACE_GENERATION",
                "createdAt": "2024-11-26T07:55:21.151Z",
                "completedAt": "2024-11-26T08:04:18.275Z",
                "context": {
                    "cairoVersion": 0,
                    "piePath": "sharp_queries/query_01JDKQF9VY2NDBFZAFNFXZC17Z/pie.zip",
                    "inputPath": "sharp_queries/query_01JDKQF9VY2NDBFZAFNFXZC17Z/input.json",
                    "programPath": "sharp_queries/query_01JDKQF9VY2NDBFZAFNFXZC17Z/program.json",
                    "layout": "recursive_with_poseidon"
                }
                },
                {
                "id": "92584ec3-a0c2-44e3-a044-a864485080ab",
                "sharpQueryId": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                "status": "COMPLETED",
                "jobName": "PROOF_GENERATION",
                "createdAt": "2024-11-26T08:04:18.598Z",
                "completedAt": "2024-11-26T08:32:26.411Z",
                "context": {
                    "proofPath": "sharp_queries/query_01JDKQF9VY2NDBFZAFNFXZC17Z/proof.json"
                }
                },
                {
                "id": "1ebba79a-2286-4cd4-85fa-5628bddd04b9",
                "sharpQueryId": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                "status": "COMPLETED",
                "jobName": "FACT_HASH_GENERATION",
                "createdAt": "2024-11-26T08:32:26.471Z",
                "completedAt": "2024-11-26T08:36:04.738Z",
                "context": {
                    "child_program_hash": "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                    "child_output": [
                    "0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07",
                    "0x74f69aa694b52f25bac8892c49500deffd5efa54b9c067ce067f0ff81de21fc"
                    ],
                    "bootloader_output": [
                    "0x1",
                    "0x4",
                    "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                    "0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07",
                    "0x74f69aa694b52f25bac8892c49500deffd5efa54b9c067ce067f0ff81de21fc"
                    ],
                    "bootloader_output_hash": "0x6bcea6d3c698d3ed3836bef8952bfd4e4c077a2ea3e21fa286c2651da341583",
                    "bootloader_program_hash": "0x5ab580b04e3532b6b18f81cfa654a05e29dd8e2352d88df1e765a84072db07",
                    "fact_hash": "0x46997b155c917870ee90724b9d0a42d2fac9bb60f6ebbe2c700aa7495d028bf"
                }
                },
                {
                "id": "97e54a9c-ee03-49e1-8840-7323d3e02ab2",
                "sharpQueryId": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                "status": "COMPLETED",
                "jobName": "PROOF_VERIFICATION",
                "createdAt": "2024-11-26T08:36:04.771Z",
                "completedAt": "2024-11-26T08:46:05.365Z",
                "context": {
                    "numberOfSteps": 16777216,
                    "hasher": "keccak_160_lsb",
                    "initial": {
                    "transactionHash": "0x3fde27ac92a8a46f0d77817c8ef8358b06581ff36b077de7a7501a6b309e818",
                    "price": 0.47,
                    "gasAmount": 8543
                    },
                    "step1": {
                    "transactionHash": "0x2b29294a8152cb4e4a05e8b8a06fa36c3927f4bc0bd62416b70f1d0f94930d7",
                    "price": 0.09,
                    "gasAmount": 1568
                    },
                    "step2": {
                    "transactionHash": "0x49b8c5e1778a8a016c80da6039592fef4784a623a525d43462f1f205926bce8",
                    "price": 0.07,
                    "gasAmount": 1223
                    },
                    "step3": {
                    "transactionHash": "0x743e13775587c32f647f5523fb65cf67796fea4e84e68dbb474490ebc26e086",
                    "price": 0.06,
                    "gasAmount": 920
                    },
                    "step4": {
                    "transactionHash": "0xacddfae337e941a8264c94864dee15518bdd8e90edc4aea6fe79b932e98e32",
                    "price": 0.04,
                    "gasAmount": 558
                    },
                    "step5": {
                    "transactionHash": "0xc2e89ecbf65a7bbfa0e69da034f2eaa283b0486e8fef9bbfde734303f78197",
                    "price": 0.03,
                    "gasAmount": 494
                    },
                    "step6": {
                    "transactionHash": "0x68755fa1b60ef08dee4058d71a1049238a337093eeabf937f5b00ebb2b72cd4",
                    "price": 0.03,
                    "gasAmount": 459
                    },
                    "step7": {
                    "transactionHash": "0x2ba659d0a0fcb53226f3564e6d71d8921e5f8460a796f2b6b5d0407497acd3f",
                    "price": 0.02,
                    "gasAmount": 335
                    },
                    "step8": {
                    "transactionHash": "0x24e0378f2c2a9e8dc84a742e0edeec5e8576f6304db243230aa065433166663",
                    "price": 0.02,
                    "gasAmount": 293
                    },
                    "final": {
                    "transactionHash": "0x2a4a05ccecc281ef1f3ba56a442c7f152f893123da37eaad254ba9b15c528f5",
                    "price": 0.02,
                    "gasAmount": 212
                    }
                }
                }
            ],
            "steps": [
                "TRACE_GENERATION",
                "PROOF_GENERATION",
                "FACT_HASH_GENERATION",
                "PROOF_VERIFICATION"
            ]
        }"#;
        let job_response: JobResponse = serde_json::from_str(response).unwrap();
        assert_eq!(job_response.jobs.len(), 4);
        assert_eq!(job_response.steps.len(), 4);
    }
    #[test]
    fn test_deserialize_query() {
        let query = r#"
            {
                "atlanticQuery": {
                    "id": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                    "submittedByClient": "01J8M351V7NV7QRQYCS54RKFYD",
                    "status": "DONE",
                    "step": "PROOF_VERIFICATION",
                    "programHash": "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                    "layout": "recursive_with_poseidon",
                    "programFactHash": "0x46997b155c917870ee90724b9d0a42d2fac9bb60f6ebbe2c700aa7495d028bf",
                    "price": "0.85",
                    "gasUsed": 14605,
                    "creditsUsed": 0,
                    "traceCreditsUsed": 0,
                    "isFactMocked": false,
                    "chain": "L2",
                    "prover": "STARKWARE_SHARP",
                    "steps": [
                    "TRACE_GENERATION",
                    "PROOF_GENERATION",
                    "FACT_HASH_GENERATION",
                    "PROOF_VERIFICATION"
                    ],
                    "createdAt": "2024-11-26T07:55:21.124Z",
                    "completedAt": "2024-11-26T08:46:05.375Z"
                }
            }
        "#;
        let query_response: SharpQueryResponse = serde_json::from_str(query).unwrap();
        assert_eq!(query_response.sharp_query.id, "01JDKQF9VY2NDBFZAFNFXZC17Z");
        println!("{:#?}", query_response);
    }
    #[test]
    fn test_deserialize_atlantic_queries() {
        let queries = r#"
        {
            "sharpQueries": [
                {
                "id": "01JDKQF9VY2NDBFZAFNFXZC17Z",
                "submittedByClient": "01J8M351V7NV7QRQYCS54RKFYD",
                "status": "DONE",
                "step": "PROOF_VERIFICATION",
                "programHash": "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                "layout": "recursive_with_poseidon",
                "programFactHash": "0x46997b155c917870ee90724b9d0a42d2fac9bb60f6ebbe2c700aa7495d028bf",
                "price": "0.85",
                "gasUsed": 14605,
                "creditsUsed": 0,
                "traceCreditsUsed": 0,
                "isFactMocked": false,
                "chain": "L2",
                "prover": "STARKWARE_SHARP",
                "steps": [
                    "TRACE_GENERATION",
                    "PROOF_GENERATION",
                    "FACT_HASH_GENERATION",
                    "PROOF_VERIFICATION"
                ],
                "createdAt": "2024-11-26T07:55:21.124Z",
                "completedAt": "2024-11-26T08:46:05.375Z"
                },
                {
                "id": "01JDKQF21NWMENSXK96E3N7CE8",
                "submittedByClient": "01J8M351V7NV7QRQYCS54RKFYD",
                "status": "DONE",
                "step": "PROOF_VERIFICATION",
                "programHash": "0x193641eb151b0f41674641089952e60bc3aded26e3cf42793655c562b8c3aa0",
                "layout": "recursive_with_poseidon",
                "programFactHash": "0x373061fa6164c9ba2c437ce1e9e148045a8b1d9373bd161a078973f5c8c568a",
                "price": "0.91",
                "gasUsed": 14839,
                "creditsUsed": 0,
                "traceCreditsUsed": 0,
                "isFactMocked": false,
                "chain": "L2",
                "prover": "STARKWARE_SHARP",
                "steps": [
                    "TRACE_GENERATION",
                    "PROOF_GENERATION",
                    "FACT_HASH_GENERATION",
                    "PROOF_VERIFICATION"
                ],
                "createdAt": "2024-11-26T07:55:13.273Z",
                "completedAt": "2024-11-26T08:56:00.861Z"
                }
            ],
            "total": 148
        }"#;
        let queries_response: SharpQueriesResponse = serde_json::from_str(queries).unwrap();
        println!("{:#?}", queries_response);
    }
}
