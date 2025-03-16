use std::env;
use crate::storage::types::MetadataResponse;

use super::types;
use reqwest::Client;
use reqwest::multipart as reqwest_multipart;
use sqlx::PgPool;
use types::Metadata;



// CONSTS And Initialisation 
const STORAGE_API_URL: &str = "http://localhost:8090";


pub struct StorageClient {
    pub user_id: i16,
    pub token: String
}

pub enum QueryResponse {
    DatabaseConnectionError,
    DatabaseQueryError,
}

impl std::fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryResponse::DatabaseConnectionError => write!(f,"Error connecting to the database"),
            QueryResponse::DatabaseQueryError => write!(f, "Unable to Query Database")
        }

    }
}

pub enum RequestError {
    RequestError,
    TransformationError
}

impl StorageClient {
    pub fn new(user_id: i16, token: String) -> Self {
        Self {
            user_id,
            token
        }
    }

    pub async fn save_metadata(&self, metadata: &Metadata) -> Result<bool, QueryResponse>  {
        // First save the metadata to the database
        let database_url: String = env::var("DATABASE_URL").unwrap();
        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                match sqlx::query!("INSERT INTO storage (description, type, path, owner) VALUES ($1, $2, $3, $4);", metadata.description, metadata.file_type.to_string(), metadata.path, self.user_id).execute(&pool).await {
                    Ok(_) => {
                        Ok(true)
                    }
                    Err(_) => {
                        Err(QueryResponse::DatabaseQueryError)
                    }
                }
            },
            Err(_) => {
                Err(QueryResponse::DatabaseConnectionError)
            }
        }

    }

    pub async fn save_file(&self, metadata: &Metadata, buffer: Vec<u8>) -> Result<bool, reqwest::Error> {
        let client = Client::new();

        let form_part = reqwest_multipart::Part::bytes(buffer)
            .file_name(metadata.name.clone())
            .mime_str(&metadata.file_type)
            .unwrap_or_else(|_| reqwest_multipart::Part::bytes(Vec::new()));

        let multipart_form = reqwest_multipart::Form::new()
            .part("file", form_part)
            .text("user_id", self.user_id.to_string());


        println!("Saving file");
        let response = client
            .post(format!("{}/upload", STORAGE_API_URL))
            .multipart(multipart_form)
            .header("Authorization", self.token.clone())
            .header("user_id", self.user_id)
            .send()
        .await;


        match response {
            Ok(response) => {
                if response.status() != 200 {
                    println!("{:?}", response);
                    println!("{}", response.text().await.unwrap());
                    println!("Error occured");
                    return Ok(false)
                }
                println!("File saved");
                Ok(true)
            },
            Err(e) => {
                println!("Error saving file : {}", e);
                Err(e)
            }
        }
    }

    pub async fn get_file_list(&self) -> Result<Vec<MetadataResponse>, sqlx::Error> {
        let database_url = env::var("DATABASE_URL").unwrap();
        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                match sqlx::query_as!(MetadataResponse, "SELECT id, owner, path, description, type as file_type FROM storage WHERE owner = $1", self.user_id).fetch_all(&pool).await {
                    Ok(data) => {
                        Ok(data)
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}
