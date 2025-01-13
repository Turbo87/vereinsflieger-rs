#[derive(Debug, clap::Parser)]
struct Args {
    /// The `CID` of the club.
    #[clap(long)]
    club_id: u32,

    /// The `Appkey` of the club.
    #[clap(long)]
    app_key: String,

    /// The username or email address of the user.
    #[clap(long)]
    username: String,

    /// The password of the user.
    #[clap(long)]
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let args = Args::parse();

    let client = vereinsflieger::Client::new().await?;
    client
        .sign_in(args.club_id, &args.username, &args.password, &args.app_key)
        .await?;

    println!("Sign in successful");

    Ok(())
}
