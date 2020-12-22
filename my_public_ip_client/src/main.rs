use reqwest::Client;
use structopt::StructOpt;
use time::{OffsetDateTime, UtcOffset};

use my_public_ip_lib::{PublicIp, Writer};

#[derive(Debug, StructOpt)]
enum Command {
    List,
    Update,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "my_public_ip_client", about = "My public ip client")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, long)]
    api_key: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let clinet = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("build client error");

    opt.cmd.call(&clinet, &opt.api_key, &opt.url).await
}

impl Command {
    async fn call(
        self,
        clinet: &Client,
        api_key: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::List => Command::list(&clinet, api_key, url).await,
            Command::Update => Command::update(&clinet, api_key, url).await,
        }
    }

    async fn list(
        clinet: &Client,
        api_key: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let public_ips = clinet
            .get(url)
            .header("APIKEY", api_key)
            .send()
            .await?
            .json::<Vec<PublicIp>>()
            .await?;

        if public_ips.is_empty() {
            println!("There is not any public ip");
        } else {
            for public_ip in public_ips {
                let updated_at = format_date_time(public_ip.updated_at)?;

                println!(
                    "{}: ip = {}, updated_at = {}",
                    public_ip.name, public_ip.ip, updated_at
                );
            }
        }

        Ok(())
    }

    async fn update(
        clinet: &Client,
        api_key: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let prev_writer = clinet
            .put(url)
            .header("APIKEY", api_key)
            .send()
            .await?
            .json::<Option<Writer>>()
            .await?;

        match prev_writer {
            Some(prev_writer) => {
                let updated_at = format_date_time(prev_writer.updated_at)?;

                println!(
                    "Prev writer: ip = {}, updated_at = {}",
                    prev_writer.ip, updated_at
                );
            }
            None => {
                println!("There is not a prev writer");
            }
        }

        Ok(())
    }
}

fn format_date_time(date_time: i64) -> Result<String, Box<dyn std::error::Error>> {
    let date_time = OffsetDateTime::from_unix_timestamp(date_time);
    let local_offset = UtcOffset::try_local_offset_at(date_time)?;

    Ok(date_time.to_offset(local_offset).format("%F %T %z"))
}
