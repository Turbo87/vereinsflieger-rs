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

    let credentials = vereinsflieger::Credentials {
        club_id: Some(args.club_id),
        username: args.username,
        password: args.password,
        app_key: args.app_key,
        auth_secret: None,
    };

    let client = vereinsflieger::Client::new(credentials);

    for user in client.list_users().await? {
        println!(
            "- {} {} ({})",
            user.first_name, user.last_name, user.user_id
        );
    }

    Ok(())
}
