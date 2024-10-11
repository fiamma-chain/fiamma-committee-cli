use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{circuit::CircuitType, error::CircuitError};

const CHUNK_SIZE: usize = 4096;

pub fn write_vk_to_file(vk_hash: &str, vk: &[u8]) -> anyhow::Result<()> {
    let file_path = get_vk_path(vk_hash, CircuitType::Groth16);

    let path = Path::new(&file_path);

    if path.exists() {
        fs::remove_file(path)?;
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);

    for chunk in vk.chunks(CHUNK_SIZE) {
        writer.write_all(chunk)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn read_vk_from_file(vk_hash: &str) -> anyhow::Result<Vec<u8>> {
    let file_path = get_vk_path(vk_hash, CircuitType::Groth16);

    read_vk_from_path(file_path.to_str().unwrap())
}

pub fn read_vk_from_path(path: &str) -> anyhow::Result<Vec<u8>> {
    let file_path = PathBuf::from_str(path)?;

    if !Path::new(&file_path).exists() {
        anyhow::bail!(CircuitError::CircuitNotExists(path.to_string()))
    }

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut vk_bytes = vec![];
    let mut buffer = vec![0u8; CHUNK_SIZE];

    loop {
        let read_bytes = reader.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }
        vk_bytes.extend_from_slice(&buffer[..read_bytes]);
    }

    Ok(vk_bytes)
}

pub fn get_vk_path(vk_hash: &str, circuit_type: CircuitType) -> PathBuf {
    let mut base_path = std::env::current_dir().expect("Failed to determine the current directory");
    base_path.push("circuits-metadata");
    base_path.push(circuit_type.to_string());
    base_path.push(vk_hash);
    base_path.push("verify-key");
    base_path
}

pub fn get_groth16_context_script_path(vk_hash: &str, circuit_type: CircuitType) -> PathBuf {
    let mut base_path = std::env::current_dir().expect("Failed to determine the current directory");
    base_path.push("circuits-metadata");
    base_path.push(circuit_type.to_string());
    base_path.push(vk_hash);
    base_path.push("context-scripts");
    base_path
}

pub fn get_groth16_leaf_script_path(
    vk_hash: &str,
    validator_key: &str,
    circuit_type: CircuitType,
) -> PathBuf {
    let mut base_path = std::env::current_dir().expect("Failed to determine the current directory");
    base_path.push("circuits-metadata");
    base_path.push(circuit_type.to_string());
    base_path.push(vk_hash);
    base_path.push(format!("leaf-scripts-{}", validator_key));
    base_path
}
