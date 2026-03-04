use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Host {
    name: String,
}

pub struct AppConfig {
    hosts: Vec<Host>,
}
