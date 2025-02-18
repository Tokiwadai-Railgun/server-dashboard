use std::str::FromStr;

pub struct StorageClient {
    name: String
}

impl StorageClient {
    pub fn new() -> Self {
        Self {
            name: "test".to_string()
        }
    }

    pub fn get_file_list() -> Vec<Metadata> {
        // write behavior to recieve all files

        // sample files for now
        let result = vec![
            Metadata {
                id: 0,
                name: "test-image".to_string(),
                path: "test/test-image.png".to_string(),
                size: 12000000,
                description: "A sample file".to_string(),
                file_type: FileType::from_str("Image").unwrap()
            },
            Metadata {
                id: 0,
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

pub struct Metadata {
    id: u64,
    name: String,
    path: String,
    size: u64, 
    description: String,
    file_type: FileType
}

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
