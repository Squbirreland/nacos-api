use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NacosHost {
    #[serde(rename = "instanceId")]
    pub instance_id: String,
    pub ip: String,
    pub port: i32,
    pub weight: f64,
    pub healthy: bool,
    pub enabled: Option<bool>,
    pub ephemeral: Option<bool>,
    #[serde(rename = "clusterName")]
    pub cluster_name: String,
    pub service: String,
    pub metadata: HashMap<String, String>,
    #[serde(rename = "instanceHeartBeatInterval")]
    pub instance_heart_beat_interval: Option<i32>,
    #[serde(rename = "instanceHeartBeatTimeOut")]
    pub instance_heart_beat_time_out: Option<i32>,
    pub ip_delete_timeout: Option<i32>,
    #[serde(rename = "instanceIdGenerator")]
    pub instance_id_generator: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosServerView {
    pub name: Option<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    pub clusters: Option<String>,
    #[serde(rename = "cacheMillis")]
    pub cache_millis: Option<i32>,
    pub hosts: Option<Vec<NacosHost>>,
    #[serde(rename = "lastRefTime")]
    pub last_ref_time: Option<i64>,
    pub checksum: Option<String>,
    #[serde(rename = "allIps")]
    pub all_ips: Option<bool>,
    #[serde(rename = "reachProtectionThreshold")]
    pub reach_protection_threshold: Option<bool>,
    pub valid: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosBeat {
    #[serde(rename = "clientBeatInterval")]
    pub client_beat_interval: u64,
    pub code: i32,
    #[serde(rename = "lightBeatEnabled")]
    pub light_beat_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosServiceInfo {
    #[serde(rename = "namespaceId")]
    pub namespace_id: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    pub name: String,
    #[serde(rename = "protectThreshold")]
    pub protect_threshold: f64,
    pub metadata: HashMap<String, String>,
    pub selector: HashMap<String, String>,
    pub clusters: Vec<NacosClusterInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosClusterInfo {
    #[serde(rename = "healthChecker")]
    pub health_checker: HashMap<String, String>,
    pub metadata: HashMap<String, String>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosServiceList {
    pub doms: Vec<String>,
    pub count: isize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosMetrics {
    #[serde(rename = "serviceCount")]
    pub service_count: Option<isize>,
    pub load: Option<f64>,
    pub mem: Option<f64>,
    #[serde(rename = "responsibleServiceCount")]
    pub responsible_service_count: Option<isize>,
    #[serde(rename = "instanceCount")]
    pub instance_count: Option<isize>,
    pub cpu: Option<f64>,
    pub status: Option<String>,
    #[serde(rename = "responsibleInstanceCount")]
    pub responsible_instance_count: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosServerSimpleView {
    pub servers: Vec<NacosServerSimple>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosServerSimple {
    pub ip: String,
    pub port: isize,
    pub state: String,
    #[serde(rename = "extendInfo")]
    pub extend_info: NacosExtendInfo,
    pub address: String,
    #[serde(rename = "failAccessCnt")]
    pub fail_access_cnt: isize,
    pub abilities: HashMap<String, HashMap<String, bool>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosExtendInfo {
    #[serde(rename = "lastRefreshTime")]
    pub last_refresh_time: isize,
    #[serde(rename = "raftMetaData")]
    pub raft_meta_data: HashMap<String, HashMap<String, NacosMetaData>>,
    #[serde(rename = "raftPort")]
    pub raft_port: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NacosMetaData {
    pub leader: String,
    #[serde(rename = "raftGroupMember")]
    pub raft_group_member: Vec<String>,
    pub term: isize,
}