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

    let params = vereinsflieger::Credentials {
        club_id: Some(args.club_id),
        username: &args.username,
        password: &args.password,
        app_key: &args.app_key,
        auth_secret: None,
    };

    let client = vereinsflieger::Client::new(&params).await?;

    let sale = vereinsflieger::NewSale {
        booking_date: "2025-01-14",
        article_id: "Lebensmittel",
        amount: 3.0,
        member_id: Some(11011),
        callsign: Some("ClubFridge neo (5x Weizen, 1x Spaten und 2x Chips)"),
        sales_tax: None,
        total_price: Some(42.15),
        counter: None,
        comment: None,
        cost_type: Some("Theke"),
        caid2: None,
        spid: None,
    };

    client.add_sale(&sale).await?;

    Ok(())
}
