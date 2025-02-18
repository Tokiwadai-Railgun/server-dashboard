use actix_web::{get, post};
use actix_web::{HttpRequest, HttpResponse};

mod file_client;
use file_client::StorageClient;

#[get("/")]
pub fn get_files(request: HttpRequest) -> HttpResponse {
    let client = StorageClient::new();
    let value = client.get_file_list();
    
    HttpResponse::Ok().json(value)

}
