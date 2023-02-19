## Rust Auto-Update mechanism
Keep your applications up to date with the Auto-Update library for rust.

## Supported Services
- Github Releases => (eg. release asset name schema: your_application_name-linux-arm64 )

## Usage
This could be a possible main entry point or better said a way to call the function, including the needed arguments.
```rust
#[tokio::main]
async fn main() {
    let os = "linux".to_string();
    let architecture = "arm64".to_string();
    let application_name = "my_test_application"
    let url: String = "https://api.github.com/repos/cli/cli/releases".to_string();
    let install_path = "/Users/michael/Development/tests/auto_update".to_string();
    check_for_update(url, os, architecture, install_path, application_name).await;
}
```
## Issues
If you are in the happy place to find a bug or a miss behavior of this library, I would be happy if you could [open an issue](https://github.com/MichaelProjects/rust-autoupdate/issues/new) with a quick guide to reproduce it, and the issue itself.

