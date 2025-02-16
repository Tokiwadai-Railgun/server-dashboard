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

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClientError::HeaderValueError => write!(f,"Error Creating header value"),
            ClientError::ClientGenerationError => write!(f, "Error Generating ")
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProxmoxResonse<T> {
    data: T
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMAgentResonse<T> {
    result: Option<Vec<T>>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceData{
    #[serde(rename = "hardware-address")]
    hardware_address: String,
    name: String,
    #[serde(rename = "ip-addresses")]
    ip_addresses: Vec<InterfaceIP>

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceIP {
    #[serde(rename = "ip-address")]
    ip_address: String,
    prefix: u8,
    #[serde(rename = "ip-address-type")]
    ip_address_type: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StorageResponse {
    name: String,
    #[serde(rename = "used-bytes")]
    used_bytes: u64,
    #[serde(rename = "total-bytes")]
    total_bytes: u64
    
}

// ----- Structs shared with proxmox_request module -----
#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct VMInfo {
    vmid: u8,
    name: String,
    status: String,
    maxmem: u64,
    mem: u64,
    cpus: u8,
    cpu: f32,
    uptime: u64,
    // ip: Vec<String>,
    maxdisk: u64,
    disk: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VMInfoResponse {
    data: Vec<VMInfo>
}


#[derive(Deserialize, Serialize, Debug)]
pub struct CompletedVmInfo {
    vmid: u8,
    name: String,
    status: String,
    uptime: u64,
    mem: MemInfo,
    cpu: CpuInfo,
    ip: Vec<NetworkInfo>,
    storage: Vec<StorageInfo>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MemInfo {
    current_mem: u64,
    max_mem: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CpuInfo {
    current_cpu: f32,
    max_cpu: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StorageInfo {
    name: String,
    current_disk: u64,
    max_disk: u64,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct NetworkInfo {
    interface_name: String,
    ip_address: String
}

#[derive(Debug)]
pub enum ResponseError {
    ReqwestError,
    JsonError
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseError::ReqwestError => write!(f, "Error Requesting data"),
            ResponseError::JsonError => write!(f, "Error  transofmring data to json")
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

    pub async fn get_vm_list(&self) -> Result<Vec<CompletedVmInfo>, ResponseError> {
        let mut return_value = Vec::new();
        let response = self.client.get(format!("{}/nodes/owomnipotent/qemu", self.base_url)).send().await.unwrap();
        match response.json::<VMInfoResponse>().await {
            Ok(json) => {
                for vm in &json.data {
                    // get the ip and the disk usage
                    let ip = self.get_vm_id(vm.vmid).await.unwrap();
                    let storage = self.get_vm_disks(vm.vmid).await.unwrap();
                    let complete_vm_info = CompletedVmInfo {
                        vmid: vm.vmid,
                        name: vm.name.clone(),
                        status: vm.status.clone(),
                        uptime: vm.uptime,
                        cpu: CpuInfo {
                            current_cpu: vm.cpu,
                            max_cpu: vm.cpus
                        },
                        mem: MemInfo {
                            current_mem: vm.mem,
                            max_mem: vm.maxmem
                        },
                        ip,
                        storage                    };
                    return_value.push(complete_vm_info);
                }
                Ok(return_value)
            }
            Err(_) => {
                Err(ResponseError::JsonError)
            }

        }
    }

    pub async fn get_vm_disks(&self, vmid: u8) -> Result<Vec<StorageInfo>, ResponseError> {
        let url = format!("{}/nodes/owomnipotent/qemu/{}/agent/get-fsinfo", self.base_url, vmid);
        match self.client.get(url).send().await {
            Ok(data) => {
                match data.json::<ProxmoxResonse<VMAgentResonse<StorageResponse>>>().await {
                    Ok(json) => {
                        let mut disk_result = Vec::new();
                        for disk in json.data.result.unwrap().into_iter() {
                            disk_result.push(StorageInfo {
                                name: disk.name,
                                current_disk: disk.used_bytes,
                                max_disk: disk.total_bytes
                            })
                        };

                        Ok(disk_result)
                    }
                    Err(_) => {
                        Ok(Vec::new())
                    }

                }
            }
            Err(_) => {
                Err(ResponseError::ReqwestError)
            }
        }

    }

    pub async fn get_vm_id(&self, vmid: u8) -> Result<Vec<NetworkInfo>, ResponseError> {
        let url = format!("{}/nodes/owomnipotent/qemu/{}/agent/network-get-interfaces", self.base_url, vmid);
        let ip_response = self.client.get(url).send().await;

        match ip_response{
            Ok(data) => {
                match data.json::<ProxmoxResonse<VMAgentResonse<InterfaceData>>>().await {
                    Ok(json) => {
                        let mut response_value = Vec::new();
                        json.data.result.into_iter().for_each(|interface| {
                            let correct_interface = interface.into_iter().find(|interface_details| interface_details.name != "lo").unwrap();
                            let interface_name = correct_interface.name.clone();
                            let ip_address = correct_interface.ip_addresses.clone().into_iter().find(|address| address.ip_address_type == "ipv4").unwrap().ip_address; 

                            response_value.push( NetworkInfo {
                                interface_name,
                                ip_address
                            });

                        });
                        Ok(response_value)
                    },
                    Err(_) => { Ok(Vec::new()) }
                }
            }
            Err(e) => {
                println!("An error occured : {}", e);
                Err(ResponseError::ReqwestError)
            }
        }
    }
}

fn get_token() -> String {
    env::var("PVEAPIToken").unwrap()
}
