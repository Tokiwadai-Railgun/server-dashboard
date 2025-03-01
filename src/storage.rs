use actix_web::{get, post, web};
use actix_web::{HttpRequest, HttpResponse};

mod file_client;
mod types;
use file_client::StorageClient;

use types::{File, FileData, Metadata, UserData};

#[get("/files")]
async fn get_files(request: HttpRequest) -> HttpResponse {
    let user_id: i16 = match request.headers().get("user_id").unwrap().to_str().unwrap().parse::<i16>() {
        Ok(result) => {
            result
        },
        Err(e) => {
            println!("Header is not a number : {}", e);
            return HttpResponse::InternalServerError().body("Please provide a valid user_id");
        }
    };
    let token = request.headers().get("Authorization").unwrap().to_str().unwrap();
    let client = StorageClient::new(user_id, token.to_string());

    match client.get_file_list().await {
        Ok(data) => {
            HttpResponse::Ok().json(data)
        },
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("Error occured getting files")
        }
    }
}

#[post("/upload")]
async fn upload(request: HttpRequest, request_body: web::Json<File>) -> HttpResponse {
    let user_data = UserData {
        user_id: 1,
        token: String::from(request.headers().get("session_token").unwrap().to_str().unwrap())
    };

    let metatada = Metadata {
        id: 0, // will not be used
        name: request_body.file_name.clone(),
        path: request_body.file_name.clone(),
        size: request_body.file_size,
        description: request_body.description.clone(),
        file_type: request_body.file_type.clone()
    };

    let file_data = FileData {
        file_name: request_body.file_name.clone(),
        file_content: request_body.file_content.clone()
    };

    let client = StorageClient::new(user_data.user_id, user_data.token);

    match client.save_metadata(metatada).await {
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
