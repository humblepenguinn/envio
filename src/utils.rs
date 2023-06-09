use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

/*
* Get the home directory of the user

@return PathBuf
*/
pub fn get_homedir() -> PathBuf {
    match dirs::home_dir() {
        Some(home) => home,
        None => {
            println!("{}: Home directory not found", "Error".red());
            std::process::exit(1);
        }
    }
}

/*
* Get the config directory which is located in the home directory
* The directory is called .envio

@return PathBuf
*/
pub fn get_configdir() -> PathBuf {
    let homedir = get_homedir();

    homedir.join(".envio")
}

/*
* Get the current working directory

@return PathBuf
*/
pub fn get_cwd() -> PathBuf {
    if let Err(e) = std::env::current_dir() {
        println!(
            "{}: Current directory not found\n {}: {}",
            "Error".red(),
            "Error info".yellow(),
            e
        );
        std::process::exit(1);
    } else {
        std::env::current_dir().unwrap()
    }
}

/*
* Download a file from a url with a progress bar

@param url &str
@param file_name &str
*/
pub async fn download_file(url: &str, file_name: &str) {
    let client = Client::new();
    let mut resp = if let Err(e) = client.get(url).send().await {
        println!("{}: {}", "Error".red(), e);
        std::process::exit(1);
    } else {
        client.get(url).send().await.unwrap()
    };

    let mut file = if let Err(e) = File::create(file_name) {
        println!("{}: {}", "Error".red(), e);
        std::process::exit(1);
    } else {
        File::create(file_name).unwrap()
    };

    let mut content_length = if resp.content_length().is_none() {
        println!("{}: Can not get content length of ", "Error".red());
        std::process::exit(1);
    } else {
        resp.content_length().unwrap()
    };

    let pb = ProgressBar::new(content_length);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    while let Some(chunk) = resp.chunk().await.unwrap() {
        let chunk_size = chunk.len();
        if let Err(e) = file.write_all(&chunk) {
            println!("{}: {}", "Error".red(), e);
            std::process::exit(1);
        }

        pb.inc(chunk_size as u64);
        content_length -= chunk_size as u64;
    }

    pb.finish();
}
