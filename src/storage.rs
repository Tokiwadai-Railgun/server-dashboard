use actix_web::{get, post, web};
use actix_web::{HttpRequest, HttpResponse};

mod file_client;
mod types;
use file_client::StorageClient;

use types::{File, FileData, Metadata, UserData};

#[get("/files")]
async fn get_files(_req_body: HttpRequest) -> HttpResponse {
    let client = StorageClient::new();
    let value = client.get_file_list();
    
    HttpResponse::Ok().json(value)
}

#[post("/upload")]
async fn upload(request: HttpRequest, request_body: web::Json<File>) -> HttpResponse {
    let user_data = UserData {
        user_id: 1,
        token: String::from(request.headers().get("session_token").unwrap().to_str().unwrap())
    };

    let user_id = 1;


    let metatada = Metadata {
        id: 0, // will not be used
        name: request_body.file_name.clone(),
        path: request_body.file_name.clone(),
        size: request_body.file_size,
        description: request_body.description.clone(),
        file_type: request_body.file_type.clone()
    };

    let file_data = FileData {
        user_data,
        file_name: request_body.file_name.clone(),
        file_content: request_body.file_content.clone()
    };

    let client = StorageClient::new();

    match client.save_metadata(metatada, user_id).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error occured when saving metadata : {}", e);
            return HttpResponse::InternalServerError().body("Error occured when saving metadata")
        }
    };

    match client.save_file(file_data).await {
        Ok(status) => {
            if status != true {
                return HttpResponse::InternalServerError().body("Errr occured when saving file")
            }
        },
        Err(e) => {
            println!("Error occured when saving file : {}", e);
            return HttpResponse::InternalServerError().body("Error occured when saving file")
        }
    }

    HttpResponse::Ok().body("File saved")
}
