use reqwest::Client;
use std::fmt;
use serde::{Deserialize, Serialize};
use reqwest::header;
use std::env;

pub struct ProxmoxClient {
    pub client: Client,
    pub base_url: String,
}

#[derive(Debug)]
pub enum ClientError {
    HeaderValueError,
    ClientGenerationError
}

// ----- Structs shared with proxmox_request module -----
#[derive(Deserialize, Serialize)]
pub struct VMInfo {
    #[serde(rename = "vmid")]
    id: u8,
    name: String,
    status: String,
    maxmem: u64,
    cpus: u8,
    uptime: u64,
    ip: Option<String>,
    #[serde(rename = "disk")]
    storage: u64,
}

#[derive(Deserialize, Serialize)]
pub struct VMInfoResponse {
    data: Vec<VMInfo>
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::HeaderValueError => write!(f,"Error Creating header value"),
            ClientError::ClientGenerationError => write!(f, "Error Generating ")
        }
    }
}

// then implements functions to the client to use if needed
impl ProxmoxClient {
    pub fn new() -> Result<Self, ClientError> {
        let mut headers = header::HeaderMap::new();
        let token = get_token();

        let header_value: header::HeaderValue = match header::HeaderValue::from_str(&format!("PVEAPIToken={}", token)) {
            Ok(header) => header,
            Err(_) => return Err(ClientError::HeaderValueError),
        };

        headers.insert(
            header::HeaderName::from_static("authorization"), 
            header_value
        );

        let client = match Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build() {

            Ok(client) => client,
            Err(_) => return Err(ClientError::ClientGenerationError)
        };

        Ok(Self {
            client,
            base_url: "https://192.168.1.88:8006/api2/json".to_string(),
        })
    }

    pub async fn get_vm_list(self) -> Result<Vec<VMInfo>, reqwest::Error> {
        let response = self.client.get("https://192.168.1.88:8006/api2/json/nodes/owomnipotent/qemu").send().await.unwrap();
        response.json::<Vec<VMInfo>>().await
    }

    pub async fn get_vm_info(self) {
        todo!()
    }
}

#[derive(Debug)]
pub enum ProxmoxAuthError {
    JsonError,
    RequestError
}

fn get_token() -> String {
    env::var("PVEAPIToken").unwrap()
}
