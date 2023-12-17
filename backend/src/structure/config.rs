use gotham_derive::{StateData, StaticResponseExtender};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, StateData, StaticResponseExtender)]
pub struct RequestBody {
    pub address: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Device {
    name: String,
    mac: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    devices: Vec<Device>
}

impl Config {
    pub fn new() -> Config {
        Config {devices: Vec::new()}
    }
}