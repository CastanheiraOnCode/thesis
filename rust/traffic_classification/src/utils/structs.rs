use std::net::IpAddr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Flow {
    #[serde(rename = "@timestamp" )]
    pub timestamp: String,
    pub source: Source,
    pub destination: Destination
    /*pub dns_info: Option<DNS>,
    pub network_info: Option<Network>, // Include network info
    pub tls_info: Option<TLS>, */
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub address: Option<String>,
    pub as_number: Option<i64>,
    pub as_organization_name: Option<String>,
    pub bytes: Option<i64>,
    pub packets: Option<i64>,
    pub port: Option<i64>,
    pub ip: Option<IpAddr>,  // Make ip optional
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Destination {
    pub address: Option<String>,
    pub as_number: Option<i64>,
    pub as_organization_name: Option<String>,
    pub domain: Option<String>,
    pub bytes: Option<i64>,
    pub packets: Option<i64>,
    pub port: Option<i64>,
    pub ip: Option<IpAddr>,  // Make ip optional
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DNS {
    pub answers: Vec<serde_json::Value>,
    pub op_code: Option<String>,
    pub response_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub bytes: Option<i64>,
    pub packets: Option<i64>,
    pub protocol: Option<String>,
    pub transport: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TLS {
    pub sni: Option<String>,
}
