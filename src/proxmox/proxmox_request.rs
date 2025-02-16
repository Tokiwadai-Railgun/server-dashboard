use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::{HttpRequest, post, get};

use crate::proxmox::proxmox_client::{self, CompletedVmInfo};

use super::proxmox_client::ProxmoxClient;

// #[derive(Debug, Deserialize)]
// struct VMDetails {
//     #[serde(rename="vmid")]
//     id: String,
//     status: String,
//     maxmem: u64,
//     cpus: u32,
//     uptime: u64,
//     #[serde(default)]
//     ip: Option<String>,
//     #[serde(rename = "disk")]
//     storage: u64,
//     #[serde(rename = "ostype")]
//     os: String
// }


// Proxmox client struct with base informations
// ----- List of vms -----
#[get("/proxmox/vms")] 
pub async fn request_vm_list(_req: HttpRequest) -> HttpResponse {
    // let client = Client::builder()
    //     .danger_accept_invalid_certs(true)
    //     .build().unwrap();
    let proxmox = ProxmoxClient::new().unwrap();

    let response = proxmox.get_vm_list().await;

    match response {
        Ok(data) => {
            let json = serde_json::to_value(data).unwrap();
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json);


        },
        Err(e) => {println!("Error occured : {}", e)}
    }

    HttpResponse::BadRequest().body("Bad request")
}
