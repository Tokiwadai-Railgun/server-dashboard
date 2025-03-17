use actix_web::{self, get, App, HttpResponse, HttpServer, Responder};

mod file_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(file_handler::upload_file)
            .service(file_handler::download_file)
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
