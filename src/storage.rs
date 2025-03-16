use std::env;
use std::fs::File;
use std::io::Read;

use actix_multipart::form::MultipartForm;
use actix_web::{get, post};
use actix_web::{HttpRequest, HttpResponse};

mod file_client;
mod types;
use file_client::StorageClient;

use sqlx::PgPool;
use types::{FileForm, Metadata, UserData, UserPermissions};

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

    if !user.roles.contains("admin") && !user.roles.contains("cloud") {
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
