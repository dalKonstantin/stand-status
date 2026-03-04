use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize, Debug)]
pub struct Host {
    name: String,
    ip: Ipv4Addr,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub hosts: Vec<Host>,
}
