extern crate auto_update;

#[tokio::main]
async fn main() {
    let os = "linux".to_string();
    let architecture = "arm64".to_string();
    let application_name = "my_test_application".to_string();
    let url: String = "https://api.github.com/repos/cli/cli/releases".to_string();
    //let install_path = "/usr/local/bin/".to_string();

    let install_path = "/Users/michael/Development/tests/auto_update/".to_string();
    let res = auto_update::check_for_update(url.clone()).await;
    println!("{:?}", res);

    if res.unwrap() {
        let x = auto_update::download_install_update(url, os, architecture, install_path, application_name).await;
        println!("Install res: {:?}", x)
    }
}

