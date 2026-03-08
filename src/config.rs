use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize, Debug)]
pub struct Host {
    pub name: String,
    pub ip: Ipv4Addr,
    #[serde(skip_deserializing)]
    pub is_online: bool,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub hosts: Vec<Host>,
}
