#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = vereinsflieger::Client::new();
    let access_token = client.get_access_token().await?;
    println!("{access_token}");
    Ok(())
}
