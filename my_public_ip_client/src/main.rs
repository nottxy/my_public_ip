use structopt::StructOpt;
use time::{offset, OffsetDateTime};

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

    match opt.cmd {
        Command::List => list(&opt.api_key, &opt.url).await,
        Command::Update => update(&opt.api_key, &opt.url).await,
    }
}

async fn list(api_key: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let public_ips = reqwest::Client::new()
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
            let updated_at = OffsetDateTime::from_unix_timestamp(public_ip.updated_at)
                .to_offset(offset!(+8))
                .format("%F %T %z");

            println!(
                "{}: ip = {}, updated_at = {}",
                public_ip.name, public_ip.ip, updated_at
            );
        }
    }

    Ok(())
}

async fn update(api_key: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let prev_writer = reqwest::Client::new()
        .put(url)
        .header("APIKEY", api_key)
        .send()
        .await?
        .json::<Option<Writer>>()
        .await?;

    match prev_writer {
        Some(prev_writer) => {
            let updated_at = OffsetDateTime::from_unix_timestamp(prev_writer.updated_at)
                .to_offset(offset!(+8))
                .format("%F %T %z");

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
