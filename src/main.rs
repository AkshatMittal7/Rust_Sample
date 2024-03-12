use chrono::prelude::*;
use std::fmt;
use std::fs;

struct FileMetadata{
    size: u64,
    file_type: fs::FileType,
    permissions: fs::Permissions,
    modified_time: DateTime<Local>,
    accessed_time: DateTime<Local>,
    created_time: DateTime<Local>,
}

impl fmt::Display for FileMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File Metadata: \n\
            Size: {} bytes \n\
            Type: {:?}\n\
            Permissions: {:?}\n\
            Modified Time: {}\n\
            Accessed Time: {}\n\
            Created Time: {}",
            self.size,
            self.file_type,
            self.permissions,
            self.modified_time.format("%Y-%m-%d %H:%M:%S"),
            self.accessed_time.format("%Y-%m-%d %H:%M:%S"),
            self.created_time.format("%Y-%m-%d %H:%M:%S")
        )
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const FILE: &str = "./floppa_peter_griffin.png";

    let metadata = fs::metadata(FILE)?;
    let file_metadata = FileMetadata {
        size: metadata.len(),
        file_type: metadata.file_type(),
        permissions: metadata.permissions(),
        modified_time: DateTime::from(metadata.modified()?),
        accessed_time: DateTime::from(metadata.accessed()?),
        created_time: DateTime::from(metadata.created()?),
    };

    println!("{}", file_metadata);
    Ok(())
}
