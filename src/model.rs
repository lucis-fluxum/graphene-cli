use serde::{Deserialize, Serialize};

// TODO: Use more accurate types for these fields, like uuid, url, chrono date times, etc.

#[derive(Debug, Serialize, Deserialize)]
pub struct Backup {
    duration: i64,
    timestamp: String,
    size: i64,
    state: String,
    id: String,
    origin: String,
    #[serde(rename = "packageUrl")]
    package_url: String,
    #[serde(rename = "isDownloadable")]
    is_downloadable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "masterURL")]
    pub master_url: String,
    pub nodes: Vec<ServerId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counters {
    pub nodes: i64,
    pub relationships: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub id: String,
    pub name: String,
    #[serde(rename = "isCluster")]
    pub is_cluster: bool,
    #[serde(rename = "restUrl")]
    pub rest_url: String,
    #[serde(rename = "webAdminURL")]
    pub web_admin_url: String,
    #[serde(rename = "webAdminAuth")]
    pub web_admin_auth: String,
    #[serde(rename = "statusChangedAt")]
    // TODO: Not sure about the type of this one, I just know it can be null
    pub status_changed_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    version: String,
    #[serde(rename = "versionNumber")]
    pub version_number: String,
    #[serde(rename = "currentSize")]
    pub current_size: i64,
    pub plugins: Option<Vec<Plugin>>,
    #[serde(rename = "maxSize")]
    pub max_size: i64,
    pub encrypted: bool,
    pub plan: Plan,
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "privateNetworkId")]
    // TODO: Not sure about the type of this one, I just know it can be null
    pub private_network_id: Option<String>,
    pub counters: Counters,
    #[serde(rename = "boltURL")]
    pub bolt_url: String,
    // TODO: I didn't see these fields in the responses I got
    // #[serde(rename = "metricsURL")]
    // metrics_url: Option<String>,
    // cluster: Cluster,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
    pub nodes: i64,
    pub relationships: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "type")]
    pub plan_type: String,
    pub limits: Limits,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    #[serde(rename = "isManaged")]
    pub is_managed: bool,
    pub kind: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerId {
    #[serde(rename = "serverId")]
    pub server_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub description: String,
}
