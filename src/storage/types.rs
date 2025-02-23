use serde::{Serialize, Deserialize};
use std::string::ToString;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub size: u64, 
    pub description: String,
    pub file_type: FileType
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

pub struct UserData {
    pub user_id: i16,
    pub token: String
}

pub struct FileData {
    pub user_data: UserData,
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
