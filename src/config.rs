use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestCase {
    pub id: String,
    pub url: String,
    pub method: Option<String>,
    pub post_data: Option<String>,
    pub expected_status: Option<u16>,
    pub verify_positive: Option<String>,
    pub verify_negative: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebScenario {
    pub global_timeout_ms: Option<u64>,
    pub steps: Vec<TestCase>,
}
