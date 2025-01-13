#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = vereinsflieger::Client::new().await?;
    let access_token = client.access_token();
    println!("{access_token}");
    Ok(())
}
