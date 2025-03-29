use std::env;
use std::fs::File;
use std::io::Read;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, Responder};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;

mod file_client;
mod types;
use file_client::StorageClient;

use reqwest::header::{HeaderMap, HeaderValue};
use sqlx::PgPool;
use types::{FileForm, Metadata, UserData, UserPermissions};


const AUTHORIZED_ROLES: [&str; 2] = ["Admin", "cloud"];

#[get("/files")]
async fn get_files(request: HttpRequest) -> HttpResponse {
    let token = request.headers().get("Authorization").unwrap().to_str().unwrap();

    // Get userId from database
    let user = get_user_permissions(token).await.unwrap();

    let client = StorageClient::new(user.user_id.try_into().unwrap(), token.to_string());


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
async fn upload(request: HttpRequest, MultipartForm(form): MultipartForm<FileForm>) -> HttpResponse {

    // ----- Check user Permissions -----
    let token = request.headers().get("Authorization");

    let user = match token {
        Some(token) => {
            get_user_permissions(token.to_str().unwrap()).await.unwrap()
        }
        None => {
            return HttpResponse::Unauthorized().body("Unautorized");
        }
    };

    if verify_user_permissions(&user.roles) {
        println!("{}", user.roles);
        return HttpResponse::Unauthorized().body("Missing permissions");
    };

    // ----- Get file informaiton -----
    let user_data = UserData {
        user_id: user.user_id,
        token: String::from(token.unwrap().to_str().unwrap())
    };

    let metatada = Metadata {
        id: 0, // will not be used
        name: form.file_data.file_name.clone(),
        path: form.file_data.file_name.clone(),
        size: form.file_data.file_size,
        description: form.file_data.description.clone(),
        file_type: form.file_data.file_type.clone()
    };

    // ----- Get file content -----
    let file_path = form.file.file.path();
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error occured when opening temporary file : {}", e);
            return HttpResponse::InternalServerError().body("Error readig uploaded file");
        }
    };

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        println!("Error reading file data : {}", e);
        return HttpResponse::InternalServerError().body("Unable to read file data");
    }


    // ----- Save file information -----
    let client = StorageClient::new(user_data.user_id.try_into().unwrap(), user_data.token);

    match client.save_metadata(&metatada).await {
        Ok(_) => {},
        Err(e) => {
            println!("Error occured when saving metadata : {}", e);
            return HttpResponse::InternalServerError().body("Error occured when saving metadata")
        }
    };

    // ----- Save file ------
    match client.save_file(&metatada, buffer).await {
        Ok(status) => {
            if !status {
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

async fn get_user_permissions(token: &str) -> Result<UserPermissions, ()> { // return the user_id
    let database_url = env::var("DATABASE_URL").unwrap();

    match PgPool::connect(&database_url).await {
        Ok(pool) => {
            match sqlx::query!("SELECT * FROM session_permissions WHERE token = $1", token).fetch_one(&pool).await {
                Ok(result) => {
                    let result = UserPermissions {
                        user_id: result.user_id.unwrap(),
                        roles: result.roles.unwrap()
                    };

                    Ok(result)

                },
                Err(e) => {
                    println!("Error retrieving user permissions : {}", e);
                    Err(())
                }
            }
        },
        Err(e) => {
            println!("Error connecting to database : {}", e);
            Err(())
        }
    }
}

fn verify_user_permissions(roles: &str) -> bool {
    for authorized in AUTHORIZED_ROLES {
        if roles.contains(authorized) { return true };
    }


    false
}


#[get("/download/{filename}")]
async fn download(request: HttpRequest, filename: web::Path<String>) -> impl Responder {
    // Extract the filename from the path
    let filename = filename.into_inner();
    
    // Get the user_id from Authentication token
    // ----- Check user Permissions -----
    let token = request.headers().get("Authorization");

    let user = match token {
        Some(token) => {
            get_user_permissions(token.to_str().unwrap()).await.unwrap()
        }
        None => {
            return HttpResponse::Unauthorized().body("Unautorized");
        }
    };

    if verify_user_permissions(&user.roles) {
        println!("{}", user.roles);
        return HttpResponse::Unauthorized().body("Missing permissions");
    };
    

    // ----- Generating request to storage API -----
    let client = reqwest::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert("user_id", HeaderValue::from_str(&user.user_id.to_string()).unwrap());
    
    let url = format!("http://localhost:8090/download/{}", filename);
    
    let response = match client.get(&url)
        .headers(headers)
        .send()
        .await {
            Ok(resp) => resp,
            Err(_) => return HttpResponse::InternalServerError().body("Failed to connect to upstream server"),
        };
    
    if !response.status().is_success() {
        let status_code = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        return HttpResponse::build(status_code)
            .body(format!("Upstream server returned: {}", response.status()));
    }
    
    let content_type = response.headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();
    
    let content_disposition = match response.headers().get("content-disposition") {
        Some(h) => h.to_str().unwrap_or(&format!("attachment; filename=\"{}\"", filename)).to_string(), // Content disposition to make destination download file
        None => format!("attachment; filename=\"{}\"", filename),
    };
    
    let bytes = match response.bytes().await {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read response body"),
    };
    

    // ----- Return file -----
    HttpResponse::Ok()
        .content_type(content_type)
        .append_header(("Content-Disposition", content_disposition))
        .body(bytes)
}
