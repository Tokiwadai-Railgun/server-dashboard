use actix_web::{self, get, post, App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| {
        App::new()
            .service(store_file)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



#[derive(Debug, Serialize)]
struct File {
    user_id: i16,
    file_name: String,
    content: String
}

#[post("/store")]
async fn store_file(req_body: web::Json<File>) -> HttpResponse {
    HttpResponse::Ok().body("Test")
}
