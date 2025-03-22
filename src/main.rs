use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

// login module
pub mod login;

// proxmox module
pub mod proxmox ;
use proxmox::proxmox_request;

// Storage
mod storage;

// Structs for the user and the session

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Loading environment variables
    dotenv::dotenv().expect("Failed to load .env file");
    println!("Starting web server on port 8070 ...");

    // setting up cors 

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(login::login)
            .service(login::logout)
            .service(login::authorize)
            .service(proxmox_request::request_vm_list)
            .service(
                web::scope("/storage")
                    .service(storage::upload)
                    .service(storage::get_files)
                    .service(storage::download)
            )
    })
    .bind(("127.0.0.1", 8070))?
    .run()
    .await
}
