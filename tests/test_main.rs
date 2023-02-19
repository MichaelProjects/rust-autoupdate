use std::{fs::{self, DirEntry, File}, path::PathBuf};

use auto_update::{check_for_update, download_install_update};
use tokio;


#[tokio::test]
async fn test_check_for_update(){
    let url: String = "https://api.github.com/repos/cli/cli/releases".to_string();
    let needs_update = check_for_update(url).await.unwrap();
    assert!(needs_update);
}

#[tokio::test]
async fn test_install_download_update(){}


#[tokio::test]
async fn test_normal_usage() {
    let os = "linux".to_string();
    let architecture = "arm64".to_string();
    let application_name = "my_test_application".to_string();
    let url: String = "https://api.github.com/repos/cli/cli/releases".to_string();
    let install_path = "/Users/michael/Development/tests/auto_update/".to_string();
    download_install_update(url, os, architecture, install_path.clone(), application_name.clone()).await;

    //check if the directory contains the downloaded file
    let files = fs::read_dir(PathBuf::from(install_path.clone())).unwrap();
    let mut found = false;
    for file in files {
        let x = file.unwrap();
        if x.file_name().to_str().unwrap().to_string() == application_name {
            found = true;
            break
        }
    }
    assert!(found);
    fs::remove_file(PathBuf::from(install_path.clone())).unwrap();
}

