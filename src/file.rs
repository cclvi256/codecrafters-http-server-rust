use std::{fs::File, io::Write};

pub fn read_file(env_path: &str, path: &str) -> Result<Vec<u8>, std::io::Error> {
    let full_path = format!("{}/{}", env_path, path);
    let content = std::fs::read(full_path)?;
    Ok(content)
}

pub fn write_file(env_path: &str, path: &str, content: &Vec<u8>) -> Result<(), std::io::Error> {
    let full_path = format!("{}/{}", env_path, path);
    let mut file = File::open(full_path)?;
    file.write_all(content.as_slice())?;
    Ok(())  
}
