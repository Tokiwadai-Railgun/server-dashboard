use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use base32::Alphabet;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, PgPool};
use time::Duration;


const DATABASE_URL: &str = "postgres://fuyuki:Walendithas@localhost:5432/server_dashboard";
// Structs for the user and the session
#[derive(Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct Session {
    id: i32,
    user_id: i32,
    token: String,
    expires_at: OffsetDateTime
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String
}

fn generate_token() -> String{
    // generate 20 random bytes
    let mut bytes = vec![0u8; 20];
    OsRng.fill_bytes(&mut bytes);
    
    // then get it to a base32 encoded version
    base32::encode(Alphabet::Rfc4648 { padding: false}, &bytes)
}

async fn store_session(session: Session) -> Result<(), ()> {
    match PgPool::connect(DATABASE_URL).await {
        Ok(pool) => {
            match sqlx::query!("INSERT INTO user_session (user_id, token, expires_at) values ($1, $2, $3)", session.user_id, session.token, session.expires_at).execute(&pool).await {
                Ok(_) => {Ok(())},
                Err(e) => {
                    println!("Query failed : {}", e);
                    Err(())
                }
            }
        }
        Err(e) => {
            println!("Connection to the database failed : {}",e);
            Err(())
        }
    }
}
// Functions related to db
async fn verify_credentials(email: &str, password: &str) -> Result<bool, sqlx::Error> {
    match PgPool::connect(DATABASE_URL).await {
        Ok(pool) => {
            match sqlx::query!("SELECT password from users WHERE email = $1;", email).fetch_one(&pool).await {
                Ok(result) => {
                    if result.password == password {
                        return Ok(true)
                    }
                }
                Err(e) => {
                    println!("Query Failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Connection test failed: {}", e)
        }
    }

    Ok(false)
}




#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/authorize")]
async fn authorize(req_body: web::Json<Token>) -> HttpResponse {
    todo!()
}

#[post("/revoke")]
async fn revoke(req_body: web::Json<Token>) -> HttpResponse {
    // revoke all sessions for this user
    todo!()
}

#[post("/logout")]
async fn logout(req_body: web::Json<Token>) -> HttpResponse {
    // revoke the session with this token
    todo!()
}


#[post("/login")]
async fn login(req_body: web::Json<User>) -> HttpResponse {
    // Then query database to search for a user with this email

    // If it exist verify the password 
    let status = verify_credentials(&req_body.email, &req_body.password).await;
    match status { 
        Ok(false) => {
            HttpResponse::Unauthorized().body("Wrong username or password")
        },
        Ok(true) => { 
            // Generate session token
            let token = generate_token();
            
            // Save session token in database
            let session: Session = Session {
                id: 0,
                user_id: req_body.id,
                token: token.clone(),
                expires_at: OffsetDateTime::now_utc() + Duration::new(3 * 60 * 60, 0)
            };
            store_session(session).await.expect("Failed storing the session");
            
            // Return session token
            HttpResponse::Ok().json(Token {
                token
            })
         },
        Err(_) => {
            HttpResponse::InternalServerError().body("Error querrying database")
        }
    }
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server on port 8080 ...");
    HttpServer::new(|| {
        App::new()
            .service(login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
