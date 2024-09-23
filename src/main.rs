use reqwest::multipart;
use tokio::{fs::File, io::AsyncReadExt};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long,env)]
    api_key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let is_alive = get_is_alive().await;
    println!("{:?}", is_alive);
    let is_proof_verification = proof_verification(args.api_key).await;
    println!("{:?}", is_proof_verification);
}

async fn get_is_alive() -> Result<bool, reqwest::Error> {
    let res = reqwest::get("https://sharp.api.herodotus.cloud/is-alive").await?;
    Ok(res.status().is_success())
}
async fn proof_verification(api_key: String) -> Result<bool, reqwest::Error> {
    let url = format!("https://sharp.api.herodotus.cloud/submit-sharp-query/proof_verification?apiKey={}", api_key);
    let client = reqwest::Client::new();

    // Read the file content
    let mut file = File::open("/home/mateuszpc/dev/dojo/proof_186801.json").await.unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await.unwrap();

    // Create multipart form
    let form = multipart::Form::new()
        .part("proofFile", multipart::Part::bytes(buffer).file_name("proof_186801.json").mime_str("application/json")?);

    let response = client
        .post(url)
        .header("accept", "application/json")
        .multipart(form)
        .send()
        .await?;

    let response_text = response.text().await.unwrap();
    println!("{:#?}", response_text);

    Ok(true)
}
