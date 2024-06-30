use std::{fs::File, io::{self, Read}, path::PathBuf};

pub fn read_file(path: PathBuf) -> Result<String,io::Error> {
    let mut file_content = String::new();

    File::open(path)?.read_to_string(&mut file_content)?;
    Ok(file_content)
}
