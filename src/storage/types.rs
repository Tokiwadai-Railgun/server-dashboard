use serde::{Serialize, Deserialize};
use std::string::ToString;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size: u64, 
    pub description: String,
    pub file_type: FileType
}

#[derive(Debug, Serialize)]
pub struct MetadataResponse {
    pub id: i32,
    pub path: String,
    pub description: String,
    pub owner: i16,
    pub file_type: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileType {
    Image,
    Word,
    Excel,
    Pdf,
    Video
}


impl FromStr for FileType {
    type Err = ();

    fn from_str(input: &str) -> Result<FileType, Self::Err> {
        match input {
            "Image" => Ok(Self::Image),
            "Word" => Ok(Self::Word),
            "Excel" => Ok(Self::Excel),
            "Pdf" => Ok(Self::Pdf),
            "Video" => Ok(Self::Video),
            _ => Err(())
        }
    }
}
impl ToString for FileType {
    fn to_string(&self) -> String {
        match &self {
            FileType::Video => String::from("Video"),
            FileType::Image => String::from("Image"),
            FileType::Excel => String::from("Excel"),
            FileType::Word => String::from("Word"),
            FileType::Pdf => String::from("Pdf")
        }
    }
}

#[derive(Serialize)]
pub struct UserData {
    pub user_id: i16,
    pub token: String
}

#[derive(Serialize)]
pub struct FileData {
    pub file_name: String,
    pub file_content: String
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub file_name: String,
    pub file_size: u64,
    pub description: String,
    pub file_type: FileType,
    pub file_content: String
}
