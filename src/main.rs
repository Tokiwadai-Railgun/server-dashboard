use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use actix_cors::Cors;

// login module
pub mod login;

// proxmox module
pub mod proxmox ;
use proxmox::proxmox_request;

// Storage
mod storage;

// Structs for the user and the session

async fn manual_hello() -> impl Responder {
    println!("{}", env::var("DATABASE_URL").unwrap());
    HttpResponse::Ok().body("Hey there!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Loading environment variables
    dotenv::dotenv().expect("Failed to load .env file");
    println!("Starting web server on port 8080 ...");

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
            .service(login::authorize)
            .service(proxmox_request::request_vm_list)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
