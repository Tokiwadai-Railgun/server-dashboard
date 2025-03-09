use serde::{Serialize, Deserialize};

const VALID_TYPES: [&str; 2] = ["image/png", "image/jpeg"];

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
    pub user_id: i16,
    pub token: String
}

#[derive(Serialize)]
pub struct FileData {
    pub file_name: String,
    pub file_content: Vec<u8>
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub file_name: String,
    pub file_size: u64,
    pub description: String,
    pub file_type: String,
    pub file_content: Vec<u8>
}

impl File {
    pub fn verif_type(&self) -> bool {
        if !VALID_TYPES.contains(&self.file_type.as_str()) { return false };

        true
    }
}
