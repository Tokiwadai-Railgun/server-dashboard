use actix_web::{self, get, post, web, HttpRequest, HttpResponse, Responder};
use std::fs::File;
use std::path::Path;
use std::fs;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

const BASE_PATH: &str = "/home/fuyuki/Documents/server-dashboard-api-storage/storage";

#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    user_data: UserdData,
    file_name: String,
    file_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserdData {
    user_id: i16,
    token: String
}

#[get("/file")]
pub async fn get_file(request: HttpRequest) -> impl Responder {
    HttpResponse::InternalServerError().body("Endpoint not implemented yet")
}

#[post("/upload")]
pub async fn upload_file(request_body: web::Json<FileData>) -> impl Responder{
    //TODO: Verify account informations


    // if all is fine then send request
    match save_file(request_body).await {
        Ok(_) => {
            HttpResponse::Ok().body("File created successfully")
        },
        Err(e) => {
            println!("{}", e);
            HttpResponse::InternalServerError().body("Error occured during file creation, please retry")//
        }
    }
}

async fn save_file(file_data: actix_web::web::Json<FileData>) -> std::io::Result<()> {
    // verify if user directory exists, if not then create it 
    if !Path::new(format!("{}/{}", BASE_PATH, file_data.user_data.user_id).as_str()).exists() {
        fs::create_dir(format!("{}/{}", BASE_PATH, file_data.user_data.user_id))?;
        println!("Folder created");
    }
    let path = format!("{}/{}/{}", BASE_PATH, file_data.user_data.user_id, file_data.file_name);
    let mut file = File::create(path)?;
    file.write_all(file_data.file_content.as_bytes())?;
    Ok(())
}
