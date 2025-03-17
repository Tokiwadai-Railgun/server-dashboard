use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{self, get, post, HttpRequest, HttpResponse, Responder};
use std::path::Path;
use std::fs;

const BASE_PATH: &str = "/home/fuyuki/Documents/server-dashboard-api-storage/storage";

#[derive(Debug, MultipartForm)]
pub struct FileData {
    file: TempFile,
}


#[get("/file")]
pub async fn get_file() -> impl Responder {
    HttpResponse::InternalServerError().body("Endpoint not implemented yet")
}

#[post("/upload")]
pub async fn upload_file( request: HttpRequest, MultipartForm(form): MultipartForm<FileData>) -> impl Responder{
    let user_id = request.headers().get("user_id").ok_or_else(|| HttpResponse::BadRequest().body("Missing user_id header")); 

    if let Err(response) = user_id {
        return response;
    }

    let user_id = user_id.unwrap().to_str().unwrap();

    // ----- Préparation du dossier de sauvegarde -----
    if !Path::new(format!("{}/{}", BASE_PATH, user_id).as_str()).exists() {
        if let Err(e) = fs::create_dir(format!("{}/{}", BASE_PATH, user_id)) {
            println!("Error creating file: {}", e);
            return HttpResponse::InternalServerError().body("Error creating user directory");
        };
        println!("Folder created");
    }

    // ----- Copie du fichier temporaire à la bonne destination -----
    let path = format!("{}/{}/{}", BASE_PATH, user_id, form.file.file_name.unwrap());
    let temp_path = form.file.file.path();

    match fs::copy(temp_path, path) {
        Ok(_) => {
            println!("File created ! ");
            HttpResponse::Ok().body("File created")
        },
        Err(e) => {
            println!("Error creating file : {}", e);
            HttpResponse::InternalServerError().body("An error occured when saving the file")
        }
    }
}

#[get("/download/{filename}")]
pub async fn download_file( request: HttpRequest ) -> actix_web::Result<NamedFile> {
    let file_name: String = request.match_info().query("filename").parse().unwrap();
    let user_id = request.headers().get("user_id").ok_or_else(|| actix_web::error::ErrorBadRequest("Missing user_id header"))?.to_str().unwrap();

    let path = format!("{}/{}/{}", BASE_PATH, user_id, file_name);

    Ok(NamedFile::open(path)?)
}
