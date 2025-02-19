use actix_web::get;
use actix_web::{HttpRequest, HttpResponse};

mod file_client;
use file_client::StorageClient;

#[get("/files")]
async fn get_files(req_body: HttpRequest) -> HttpResponse {
    let client = StorageClient::new();
    let value = client.get_file_list();
    
    HttpResponse::Ok().json(value)
}
