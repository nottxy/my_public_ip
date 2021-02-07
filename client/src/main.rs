use log::{error, info};
use reqwest::Client;
use structopt::StructOpt;
use time::{OffsetDateTime, UtcOffset};

use my_public_ip_lib::{PublicIp, Writer};

const VERSION: &str = "0.5.0";

#[derive(Debug, StructOpt)]
enum Command {
    Version,
    List {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        url: String,
    },
    Update {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        url: String,
    },
    UpdateForever {
        #[structopt(short, long)]
        api_key: String,
        #[structopt(short, long)]
        url: String,
        #[structopt(short, long)]
        interval: Option<i32>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "my_public_ip_client", about = "My public ip client")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let opt = Opt::from_args();

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("build client error");

    opt.cmd.call(&client).await
}

impl Command {
    async fn call(self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Version => Self::version().await,
            Command::List { api_key, url } => Command::list(client, &api_key, &url).await,
            Command::Update { api_key, url } => Command::update(client, &api_key, &url).await,
            Command::UpdateForever {
                api_key,
                url,
                interval,
            } => Command::update_loop(client, &api_key, &url, interval.unwrap_or(60)).await,
        }
    }

    async fn version() -> Result<(), Box<dyn std::error::Error>> {
        println!("VERSION: {}", VERSION);
        Ok(())
    }

    async fn list(
        client: &Client,
        api_key: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let public_ips = client
            .get(url)
            .header("APIKEY", api_key)
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

    async fn update_loop(
        client: &Client,
        api_key: &str,
        url: &str,
        interval: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(interval as u64)).await;
            if let Err(err) = Command::update(&client, api_key, url).await {
                error!("ERROR: {:?}", err);
            }
        }
    }

    async fn update(
        client: &Client,
        api_key: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let prev_writer = client
            .put(url)
            .header("APIKEY", api_key)
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
}

fn format_date_time(date_time: i64) -> Result<String, Box<dyn std::error::Error>> {
    let date_time = OffsetDateTime::from_unix_timestamp(date_time);
    let east8 = UtcOffset::east_hours(8);

    Ok(date_time.to_offset(east8).format("%F %T %z"))
}
