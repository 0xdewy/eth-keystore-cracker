use serde_json;
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub fn read_lines(path: &PathBuf) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    let pass_file = File::open(path).unwrap();
    let lines = io::BufReader::new(pass_file).lines();
    for line in lines {
        if let Ok(l) = line {
            content.push(l);
        }
    }
    content
}

pub fn find_keystore_files(directory: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut keystore_files = Vec::new();

    for entry in std::fs::read_dir(directory)? {
        let path = entry?.path();

        if !path.is_file() {
            keystore_files.extend(find_keystore_files(&path)?);
            continue;
        }

        let file_content = match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(_) => continue,
        };

        if is_keystore(&file_content) {
            match serde_json::from_str::<serde_json::Value>(&file_content) {
                Ok(json) => match json.get("address") {
                    Some(address) => address.as_str().unwrap().to_string(),
                    None => continue,
                },
                Err(_) => continue,
            };

            keystore_files.push(path);
        }
    }

    Ok(keystore_files)
}

// Check if there is an 'address' or 'crypto' or 'Crypto' field in the json
fn is_keystore(file_content: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(file_content)
        .map(|json| json.get("address").is_some() || json.get("crypto").is_some() || json.get("Crypto").is_some())
        .unwrap_or(false)
}

