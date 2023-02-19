use std::{error::Error, fs::File, io::Write};

use hyper::{Body, Client, Request, Response, StatusCode};
use hyper_tls::HttpsConnector;
use serde_json::Value;

async fn fetch_assets(uri: String) -> Result<Vec<Value>, Box<dyn Error>> {
    // load all releases from github releases
    let url = uri.parse::<hyper::Uri>().unwrap();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rust_autoupdate")
        .uri(url)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(request).await?;
    if resp.status() != StatusCode::OK {
        return Ok(vec![]);
    }
    let content = response_to_body(resp).await.unwrap();

    let parsed: Value = serde_json::from_str(content.as_str())?;
    let array = parsed.as_array().unwrap();
    return Ok(array.to_owned());
}

pub async fn check_for_update(uri: String) -> Result<bool, Box<dyn Error>> {
    let array = fetch_assets(uri).await.unwrap();
    if array.len() > 0 {
        let version = array[0]["tag_name"].as_str().unwrap();
        let release_version = numerate_version(version);
        let binary_new = numerate_version(env!("CARGO_PKG_VERSION"));
        // does this latest release a version change? if so it checks the assets
        if release_version > binary_new {
            return Ok(true);
        }
    }
    return Ok(false);
}

fn numerate_version(original: &str) -> String {
    let mut version_vec = vec![];

    for x in original.chars() {
        if x.is_numeric() {
            version_vec.push(x.to_string());
        }
    }
    return version_vec.concat();
}

async fn response_to_body(resp: Response<Body>) -> Result<String, Box<dyn Error>> {
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let string = String::from_utf8(body_bytes.to_vec())?;
    Ok(string)
}

pub async fn download_install_update(
    uri: String,
    used_os: String,
    architecture: String,
    install_path: String,
    application_name: String,

) {
    let url = uri.parse::<hyper::Uri>().unwrap();
    let array = fetch_assets(uri).await.unwrap();
    if array.len() > 0 {
        let version = array[0]["tag_name"].as_str().unwrap();
        let release_version = numerate_version(version);
        let binary_new = numerate_version(env!("CARGO_PKG_VERSION"));
        // does this latest release a version change? if so it checks the assets
        if release_version > binary_new {
            let assets = array[0]["assets"].as_array().unwrap();
            // check if the release contains assets
            if assets.len() > 0 {
                for asset in assets {
                    let asset_name = asset["name"].as_str().unwrap().to_lowercase();
                    // check if the asset name contains the correct os and architecture
                    if asset_name.contains(used_os.to_lowercase().as_str())
                        && asset_name.contains(architecture.to_lowercase().as_str())
                    {
                        let https = HttpsConnector::new();
                        let client = Client::builder().build::<_, hyper::Body>(https);
                        let request = Request::builder()
                            .uri(asset["browser_download_url"].as_str().unwrap())
                            .header("Accept", "application/vnd.github+json")
                            .header("User-Agent", "rust_autoupdate")
                            .body(Body::empty())
                            .unwrap();
                        let res = client.request(request).await.unwrap();
                        if res.status() == StatusCode::FOUND {
                            let headers = res.headers();

                            let location = headers.get("location").unwrap().to_str().unwrap();
                            let request = Request::builder()
                                .uri(location.parse::<hyper::Uri>().unwrap())
                                .header("User-Agent", "rust_autoupdate")
                                .body(Body::empty())
                                .unwrap();
                            let res = client.request(request).await.unwrap();
                            respone_to_file(res, install_path, application_name).await;
                            return;
                        }
                    }
                }
            }
        }
    }
}

async fn respone_to_file(resp: Response<Body>, install_path: String, filename: String) {
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    let filename = format!("{}/{}", install_path, filename);
    println!("Download {}", filename);
    let mut f = File::create(filename).unwrap();
    f.write(&body_bytes).unwrap();
}
