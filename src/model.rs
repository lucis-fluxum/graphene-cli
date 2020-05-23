use serde::{Deserialize, Serialize};

// TODO: Use more accurate types for these fields, like uuid, url, chrono date times, etc.

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "masterURL")]
    master_url: String,
    nodes: Vec<ServerId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counters {
    nodes: i64,
    relationships: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Db {
    id: String,
    name: String,
    url: String,
    #[serde(rename = "boltURL")]
    bolt_url: String,
    #[serde(rename = "webAdminURL")]
    web_admin_url: String,
    #[serde(rename = "metricsURL")]
    metrics_url: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    version: String,
    #[serde(rename = "currentSize")]
    current_size: i64,
    plugins: Vec<Plugin>,
    #[serde(rename = "maxSize")]
    max_size: i64,
    plan: Plan,
    #[serde(rename = "awsRegion")]
    aws_region: String,
    counters: Counters,
    cluster: Cluster,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
    nodes: i64,
    relationships: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "type")]
    plan_type: String,
    limits: Limits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    id: String,
    name: String,
    enabled: bool,
    #[serde(rename = "isManaged")]
    is_managed: bool,
    kind: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerId {
    #[serde(rename = "serverId")]
    server_id: i64,
}
