use serde::Serialize;
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

#[derive(Debug, Serialize)]
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
