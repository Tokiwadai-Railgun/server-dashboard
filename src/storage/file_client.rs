use std::str::FromStr;
use std::env;

use super::types;
use sqlx::PgPool;
use types::{FileType, Metadata};

pub struct StorageClient {
    pub _name: String
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

impl StorageClient {
    pub fn new() -> Self {
        Self {
            _name: "test".to_string()
        }
    }

    pub async fn upload_file(&self, metadata: Metadata, content: String, owner_id: i16) -> Result<bool, QueryResponse>  {
        // First save the metadata to the database

        let database_url: String = env::var("DATABASE_URL").unwrap();
        match PgPool::connect(&database_url).await {
            Ok(pool) => {
                match sqlx::query!("INSERT INTO storage (description, type, path, owner) VALUES ($1, $2, $3, $4);", metadata.description, metadata.file_type.to_string(), metadata.path, owner_id).execute(&pool).await {
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

        // Then send the file to the server via sftp
    }

    pub fn get_file_list(&self) -> Vec<Metadata> {
        // write behavior to recieve all files
        // sample files for now
        let result = vec![
            Metadata {
                id: 0,
                name: "test-image".to_string(),
                path: "test/test-image.png".to_string(),
                size: 12000000,
                description: "A sample image".to_string(),
                file_type: FileType::from_str("Image").unwrap()
            },
            Metadata {
                id: 1,
                name: "test-word".to_string(),
                path: "test/test-word.docx".to_string(),
                size: 12000000,
                description: "A sample word file".to_string(),
                file_type: FileType::from_str("Word").unwrap()
            },
            Metadata {
                id: 2,
                name: "test-pdf".to_string(),
                path: "test/test-pdf.pdf".to_string(),
                size: 54000000,
                description: "A sample pdf".to_string(),
                file_type: FileType::from_str("Pdf").unwrap()
            },
        ];

        result
    }
}


