use actix_multipart::form::{json::Json, tempfile::TempFile, MultipartForm};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size: u64, 
    pub description: String,
    pub file_type: String
}

#[derive(Debug, Serialize)]
pub struct MetadataResponse {
    pub id: i32,
    pub path: String,
    pub description: String,
    pub owner: i16,
    pub file_type: String
}


#[derive(Serialize)]
pub struct UserData {
    pub user_id: i32,
    pub token: String
}

#[derive(Serialize)]
pub struct FileData {
    pub file_name: String,
    pub user_id: i32
}

#[derive(Debug, MultipartForm)]
pub struct FileForm {
    pub file_data: Json<File>,
    pub file: TempFile
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub file_name: String,
    pub file_size: u64,
    pub description: String,
    pub file_type: String,
}

pub struct UserPermissions {
    pub user_id: i32,
    pub roles: String
}
