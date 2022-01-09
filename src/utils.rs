use anyhow::{Error, Result};
use ethers::providers::{Http, Provider};
use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

#[allow(non_snake_case)]
pub fn ARCHIVE_NODE() -> String {
    env::var("ARCHIVE_NODE").unwrap()
}

pub fn get_provider() -> Result<Provider<Http>> {
    match Provider::<Http>::try_from(ARCHIVE_NODE().as_str()) {
        Ok(p) => Ok(p),
        Err(_e) => Err(Error::msg(
            "Failed to connect to infura http provider".to_string(),
        )),
    }
}

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

/// Returns all files in provided directory
///
/// If a single file is provided it returns a vector including that file
pub fn files_in_directory(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let files_src = {
        if path.is_dir() {
            let mut entries = std::fs::read_dir(path)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()?;
            entries.sort();
            let mut temp_src: Vec<PathBuf> = Vec::new();
            for el in entries.iter() {
                let entry: &str = match el.to_str() {
                    Some(e) => e,
                    None => break,
                };
                temp_src.push(PathBuf::from(entry));
            }
            temp_src
        } else {
            return Err(Error::msg("Expecting directory".to_string()));
        }
    };
    Ok(files_src)
}
