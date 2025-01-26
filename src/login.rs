use actix_web::{cookie::Cookie, get, post, web, HttpRequest, HttpResponse, Responder};
use base32::Alphabet;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, PgPool};
use time::Duration;
use core::fmt;
use std::env;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordVerifier
    },
    Argon2
};



// This part is made for error handling, by creating this struct we can handle errors from multiple
// source, here for example is it made to handle errors from env, argon2 and sqlx.

#[derive(Debug)]
pub enum AuthError {
    DatabaseConnectionError,
    InvalidCredentials,
    EnvVarError,
    PasswordHashError,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::DatabaseConnectionError => write!(f,"Error connecting to the database"),
            AuthError::InvalidCredentials => write!(f, "Invalid Credentials given"),
            AuthError::EnvVarError => write!(f, "Not able to recover environment variable"),
            AuthError::PasswordHashError => write!(f, "Unable to hash the password")
        }
        
    }
}

impl std::error::Error for AuthError {}



#[derive(Deserialize)]
struct User {
    id: i16,
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Session {
    user_id: i32,
    token: String,
    expires_at: OffsetDateTime
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String
}

struct VerifyResponse {
    status: bool,
    user: User
}

fn generate_token() -> String{
    // generate 20 random bytes
    let mut bytes = vec![0u8; 20];
    OsRng.fill_bytes(&mut bytes);
    
    // then get it to a base32 encoded version
    base32::encode(Alphabet::Rfc4648 { padding: false}, &bytes)
}

async fn store_session(session: Session) -> Result<(), ()> {
    let database_url: String = env::var("DATABASE_URL").unwrap();
    match PgPool::connect(&database_url).await {
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
async fn verify_credentials(username: &str, password: &str) -> Result<VerifyResponse, AuthError> {
    let database_url: String = env::var("DATABASE_URL").unwrap();
    match PgPool::connect(&database_url).await {
        Ok(pool) => {
            match sqlx::query!("SELECT id, password, salt from users WHERE username = $1;", &username).fetch_one(&pool).await {
                Ok(result) => {
                    // hash the input password to match the one in the database

                    // let argon2 = Argon2::default();
                    // For account creation, make script to generate passwords
                    // let salt = SaltString::generate(&mut OsRng);
                    // let hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|_| AuthError::PasswordHashError)?.to_string();
                    // println!("Salt {}", salt.to_string());
                    // println!("Hash {}", hash);

                    let password_hash = PasswordHash::new(&result.password).map_err(|_| AuthError::PasswordHashError)?;
                    match  Argon2::default().verify_password(password.as_bytes(), &password_hash) {
                        Ok(_) => {
                            return Ok(VerifyResponse {status: true, user: User { username: username.to_string(), password: result.password, id: result.id}});
                        },
                        Err(_) => {
                            return Err(AuthError::InvalidCredentials);
                        }
                    }
                }
                Err(e) => {
                    println!("Query Failed: {}", e);
                }
            
            }
        }
        Err(e) => {
            println!("Connection test failed: {}", e);
            return Err(AuthError::DatabaseConnectionError);
        }
    }

    Ok(VerifyResponse { status: false, user: User { username: String::from(username), password: String::from(password), id: 0} })
}




#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/authorize")]
async fn authorize(req_body: HttpRequest) -> HttpResponse {
    let database_url = env::var("DATABASE_URL").unwrap();
    // Verify if the session exists
    //
    match req_body.headers().get("session_token") {
        Some(token_header) => {
            println!("{}", token_header.to_str().unwrap());
            match PgPool::connect(&database_url).await {
                Ok(pool) => {
                    match sqlx::query!("SELECT COUNT(*) as number from user_session WHERE token = $1;", token_header.to_str().unwrap()).fetch_one(&pool).await {
                        Ok(result) => {
                            // to be added : check for expiration date
                            // get the number of rows in the result
                            if result.number.unwrap_or(0) == 1 {return HttpResponse::Ok().body("Authorization successfull")}
                        },
                        Err(e) => {
                            println!("Error Querrying the session : {}", e)
                        }
                    }
                },
                Err(e) => {
                    println!("Error occured connecting to the database : {}", e)
                }
            };
        }
        None => {
            println!("No cookie");
            return HttpResponse::Unauthorized().body("Not logged in") 
        }
    }

    

    HttpResponse::Unauthorized().body("Unvalid Session")
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
    let status = verify_credentials(&req_body.username, &req_body.password).await;
    match status { 
        Ok(verify_response) if !verify_response.status => {
            HttpResponse::Unauthorized().body("Not logged in")
        },
        Ok(verify_response) => { 
            // if verify_response.status == false return {HttpResponse::Unauthorized("Not logged in")};
            // Generate session token
            let token = generate_token();
            // Save session token in database
            let session: Session = Session {
                user_id: verify_response.user.id.into(),
                token: token.clone(),
                expires_at: OffsetDateTime::now_utc() + Duration::new(3 * 60 * 60, 0)
            };
            store_session(session).await.expect("Failed storing the session");
            let cookie = Cookie::build("session_token", token).http_only(true).path("/").finish(); // to
            // be added to production : .secure(true)
            // Return session token
            HttpResponse::Ok().cookie(cookie).finish()
         },
        Err(err) => {
            match err {
                AuthError::DatabaseConnectionError => {
                    HttpResponse::InternalServerError().body("Error connecting to the database")
                },
                AuthError::InvalidCredentials => {
                    HttpResponse::Unauthorized().body("Invalid Credentials")
                },
                AuthError::EnvVarError => {
                    HttpResponse::InternalServerError().body("An Error occured on the server")
                },
                AuthError::PasswordHashError => {
                    println!("Password Hash Error occured");
                    HttpResponse::InternalServerError().body("An Error occured on the server")
                }
            }
        }
    }
}
