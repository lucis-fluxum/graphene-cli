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
    #[serde(rename = "isCluster")]
    is_cluster: bool,
    #[serde(rename = "restUrl")]
    rest_url: String,
    #[serde(rename = "webAdminURL")]
    web_admin_url: String,
    #[serde(rename = "webAdminAuth")]
    web_admin_auth: String,
    #[serde(rename = "statusChangedAt")]
    // TODO: Not sure about the type of this one, I just know it can be null
    status_changed_at: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: String,
    version: String,
    #[serde(rename = "versionNumber")]
    version_number: String,
    #[serde(rename = "currentSize")]
    current_size: i64,
    plugins: Option<Vec<Plugin>>,
    #[serde(rename = "maxSize")]
    max_size: i64,
    encrypted: bool,
    plan: Plan,
    #[serde(rename = "awsRegion")]
    aws_region: String,
    #[serde(rename = "privateNetworkId")]
    // TODO: Not sure about the type of this one, I just know it can be null
    private_network_id: Option<String>,
    counters: Counters,
    #[serde(rename = "boltURL")]
    bolt_url: String,
    // TODO: I didn't see these fields in the responses I got
    // #[serde(rename = "metricsURL")]
    // metrics_url: Option<String>,
    // cluster: Cluster,
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
