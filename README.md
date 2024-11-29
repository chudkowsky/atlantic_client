# Atlantic Client

This is a client library for interacting with the Atlantic service. The project aims to provide an easy-to-use interface for connecting and communicating with Atlantic's API, enabling you to integrate it seamlessly into your applications.

## Features

- Connect to the Atlantic service
- Send and receive data from the service
- Handle responses and errors efficiently

## Installation

To include this client in your project, add the following to your `Cargo.toml`:


```toml
[dependencies]
atlantic_client = "0.1.0"
```

## Usage

Here's a basic example of how to use the Atlantic client in your project:

```rust

use std::{path::PathBuf, str::FromStr};
use clap::Parser;
use url::Url;
use atlantic_client::{error::AtlanticSdkError, models::{AtlanticSdk, Layout}};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, env)]
    pub prover_url: Url,
    #[arg(long, short, env)]
    pub layout: String,
    #[arg(long, env)]
    pub pie_path: PathBuf,
    #[arg(long, env)]
    pub prover_api_key: String,
}
#[tokio::main]
pub async fn main() -> Result<(), AtlanticSdkError> {
    let args = Args::parse();
    let sharp_sdk = AtlanticSdk::new(args.prover_api_key, args.prover_url)?;
    let pie_file = std::fs::read(args.pie_path)?;
    let layout = Layout::from_str(args.layout.as_str())?;
    let atlantic_query = sharp_sdk.proof_generation(pie_file, layout, atlantic_client::models::ProverVersion::Starkware).await?;
    println!("{:?}", atlantic_query);
    Ok(())
}
```

## Contributing

If you find any issues, bugs, or have suggestions for improvements, please feel free to open an issue or submit a pull request. I'm open to collaboration and would appreciate any feedback to improve the project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

```