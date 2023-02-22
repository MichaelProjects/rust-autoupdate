## Rust Auto-Update mechanism
Keep your applications up to date with the Auto-Update library for rust.

## Supported Services
- Github Releases => (eg. release asset name schema: your_application_name-linux-arm64) 

## Package the release correctly

TLDR: Ensure your binary or application is the only file in this archive aswell as no further directory structures.

If you use github releases you are most likely to ship a standalone binary for cli applications or a simple app with *.app for mac. In this case you need to directy compress the binaries without any further folder structure.

## Usage
This could be a possible main entry point or better said a way to call the function, including the needed arguments.
```rust
#[tokio::main]
async fn main() {
    let os = "linux".to_string();
    let architecture = "arm64".to_string();
    let application_name = "my_test_application".to_string();
    let url: String = "https://api.github.com/repos/cli/cli/releases".to_string();
    let install_path = "/Users/michael/Development/tests/auto_update".to_string();
    check_for_update(url, os, architecture, install_path, application_name).await;
}
```

## Issues
If you are in the happy place to find a bug or a miss behavior of this library, I would be happy if you could [open an issue](https://github.com/MichaelProjects/rust-autoupdate/issues/new) with a quick guide to reproduce it, and the issue itself.

