mod crypto_stuff;

// use std::env;
use std::{
    env, fs::{create_dir, read_to_string, remove_dir_all, remove_file, File}, io::{self, Write},
};

fn encrypt_file(file_path: &str) {
    match read_file_names(file_path) {
        Ok(file_names) => {
            println!("{:?}", file_names);
            for file_name in file_names.clone() {
                let _ = crypto_stuff::encrypt_file(&file_name);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn decrypt_file(file_path: &str) {
    match read_file_names(file_path) {
        Ok(file_names) => {
            println!("{:?}", file_names);
            for file_name in file_names.clone() {
                let encrypted_file_path = file_name + ".yars";
                let _ = crypto_stuff::decrypt_file(&encrypted_file_path);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_file_names(file_path: &str) -> Result<Vec<String>, io::Error> {
    let content: String = read_to_string(file_path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();
    Ok(lines)
}


fn create_file(file_path: &str) {
    let file = File::create(file_path);
    match file {
        Ok(mut file) => file
            .write_all("Hello, World!".as_bytes())
            .expect("Write failed"),
        Err(error) => panic!("There was a problem opening the file: {error:?}"),
    }
}

fn prepare_test(file_path: &str) {
    match read_file_names(file_path) {
        Ok(file_names) => {
            println!("{:?}", file_names);
            for file_name in file_names.clone() {
                create_file(file_name.as_str());
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn auto_test() {
    let directory_name: &str = "auto_test_directory";
    match create_dir(directory_name) {
        Ok(()) => {
            for x in 1..100 {
                let file_name: String = directory_name.to_owned() + "/file" + x.to_string().as_str() + ".txt";
                println!("{}", file_name);
                create_file(file_name.as_str());
            }
        }
        Err(e) => eprintln!("Error creating directory: {}", e),
    }
}

fn clean_auto_test() {
    let directory_name: &str = "auto_test_directory";
    let _ = remove_dir_all(directory_name);
}

fn cleanup(file_path: &str) {
    match read_file_names(file_path) {
        Ok(file_names) => {
            println!("{:?}", file_names);
            for file_name in file_names {
                remove_file(file_name).expect("Could not remove file");
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut query: &String = &"".to_string();
    let mut file_path: &String = &"".to_string();

    if args.len() == 2 {
        query = &args[1];
    } else if args.len() == 3 {
        query = &args[1];
        file_path = &args[2];
    } else {
        println!("Error");
    }

    match query.as_str() {
        "prepare" => {
            println!("Preparing test from file {}", file_path);
            prepare_test(file_path)
        },
        "cleanup" => {
            println!("Cleaning the test from file {}", file_path);
            cleanup(file_path)
        },
        "encrypt" => {
            println!("Encrypting...");
            encrypt_file(file_path)
        },
        "decrypt" => {
            println!("Decryptig...");
            decrypt_file(file_path)
        },
        "auto" => {
            println!("Preparing auto test environment...");
            auto_test()
        }
        "cleanup_auto" => {
            println!("Cleaning auto test environment...");
            clean_auto_test()
        }
        _ => println!("Error."),
    }
}
