use serde::{Deserialize, Serialize};

// TODO: Use more accurate types for these fields, like uuid, url, chrono date times, etc.
// TODO: Use serde(rename = "...") using field names in API documentation

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
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
    bolt_url: String,
    web_admin_url: String,
    metrics_url: Option<String>,
    created_at: String,
    version: String,
    current_size: i64,
    plugins: Vec<Plugin>,
    max_size: i64,
    plan: Plan,
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
    plan_type: String,
    limits: Limits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    id: String,
    name: String,
    enabled: bool,
    is_managed: bool,
    kind: Option<String>,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerId {
    server_id: i64,
}
