use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Peer {
    pub name: String,
    pub region: String,
    pub ip: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Peers {
    pub peers: Vec<Peer>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PortConfig {
    pub protocol: String,
    pub port: u16,
    pub cidr: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InstanceConfig {
    pub name: String,
    pub region: String,
    pub instance_type: String,
    pub storage_size: i32,
    pub storage_class: String,
    pub binary: String,
    pub config: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MonitoringConfig {
    pub instance_type: String,
    pub storage_size: i32,
    pub storage_class: String,
    pub dashboard: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub instances: Vec<InstanceConfig>,
    pub monitoring: MonitoringConfig,
    pub ports: Vec<PortConfig>,
}
