use std::time::Duration;

use log::{error, info};
use reqwest::Client;
use structopt::StructOpt;
use time::{OffsetDateTime, UtcOffset};

use my_public_ip_lib::{PublicIp, Writer};

#[derive(Debug, StructOpt)]
#[structopt(name = "my_public_ip_client", about = "My public ip client")]
struct Opt {
    #[structopt(flatten)]
    args: Args,
    #[structopt(long, default_value = "10")]
    timeout: u64,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(long)]
    api_key: String,
    #[structopt(long)]
    url: String,
}

#[derive(Debug, StructOpt)]
enum Command {
    List,
    Update,
    UpdateForever {
        #[structopt(long, default_value = "60")]
        interval: u64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opt = Opt::from_args();

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(opt.timeout))
        .build()
        .expect("build client error");

    opt.cmd.call(&client, &opt.args).await
}

impl Command {
    async fn call(self, client: &Client, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::List => Self::list(client, args).await,
            Self::Update => Self::update(client, args).await,
            Self::UpdateForever { interval } => {
                Self::update_forever(client, args, Duration::from_secs(interval)).await
            }
        }
    }

    async fn list(client: &Client, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        let public_ips = client
            .get(&args.url)
            .header("APIKEY", &args.api_key)
            .send()
            .await?
            .json::<Vec<PublicIp>>()
            .await?;

        if public_ips.is_empty() {
            info!("list: There is not any public ip");
        } else {
            for public_ip in public_ips {
                let updated_at = format_date_time(public_ip.updated_at)?;

                info!(
                    "list: name={}, ip={}, updated_at={}",
                    public_ip.name, public_ip.ip, updated_at
                );
            }
        }

        Ok(())
    }

    async fn update(client: &Client, args: &Args) -> Result<(), Box<dyn std::error::Error>> {
        let prev_writer = client
            .put(&args.url)
            .header("APIKEY", &args.api_key)
            .send()
            .await?
            .json::<Option<Writer>>()
            .await?;

        match prev_writer {
            Some(prev_writer) => {
                let updated_at = format_date_time(prev_writer.updated_at)?;

                info!("update: ip={}, updated_at={}", prev_writer.ip, updated_at);
            }
            None => {
                info!("update: no_prev_writer");
            }
        }

        Ok(())
    }

    async fn update_forever(
        client: &Client,
        args: &Args,
        interval: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            tokio::time::sleep(interval).await;
            if let Err(err) = Self::update(&client, args).await {
                error!("ERROR: {:?}", err);
            }
        }
    }
}

fn format_date_time(date_time: i64) -> Result<String, Box<dyn std::error::Error>> {
    let date_time = OffsetDateTime::from_unix_timestamp(date_time);
    let east8 = UtcOffset::east_hours(8);

    Ok(date_time.to_offset(east8).format("%F %T %z"))
}
