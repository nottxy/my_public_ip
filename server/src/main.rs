use actix_web::{middleware, App, HttpServer};
use log::{debug, info};
use structopt::StructOpt;

use my_public_ip_server::{api, Config, Store};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};

const VERSION: &str = "0.5.0";

#[derive(Debug, StructOpt)]
enum Command {
    Version,
    Run(RunArgs),
}

#[derive(Debug, StructOpt)]
struct RunArgs {
    #[structopt(long, default_value = "/var/run/my_public_ip_server.pid")]
    pid_file: String,
    #[structopt(long)]
    log_file: String,
    #[structopt(long)]
    config_file: String,
    #[structopt(long)]
    db_dir: String,
    #[structopt(long, default_value = "8998")]
    port: u16,
    #[structopt(long)]
    cert_file: String,
    #[structopt(long)]
    key_file: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "my_public_ip_server", about = "My public ip server")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

impl Command {
    async fn call(self) -> std::io::Result<()> {
        match self {
            Command::Version => Self::version().await,
            Command::Run(run_args) => Self::run(&run_args).await,
        }
    }

    async fn version() -> std::io::Result<()> {
        println!("VERSION: {}", VERSION);
        Ok(())
    }

    async fn run(run_args: &RunArgs) -> std::io::Result<()> {
        log4rs::init_file(&run_args.log_file, Default::default()).expect("init log4rs error");

        info!("VERSION: {}", VERSION);
        write_pid_file(&run_args.pid_file)?;

        let config = Config::load(&run_args.config_file)
            .expect("load config error")
            .into();
        let store = Store::open(&run_args.db_dir).expect("open db_dir error");
        let api_state = api::ApiState::new(config, store);

        let addr = format!("0.0.0.0:{}", run_args.port);

        let ssl_builder = build_ssl_builder(&run_args.cert_file, &run_args.key_file);

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .data(api_state.clone())
                .service(api::list_ips)
                .service(api::update_ip)
        })
        .bind_openssl(addr, ssl_builder)?
        .run()
        .await
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    opt.cmd.call().await
}

fn write_pid_file(pid_file: &str) -> std::io::Result<()> {
    use std::io::Write;

    debug!("write pid to file: {}", pid_file);

    let pid = std::process::id();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(pid_file)?;

    file.write_all(pid.to_string().as_ref())
}

fn build_ssl_builder(cert_file: &str, key_file: &str) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("mozilla_intermediate create ssl builder error");
    builder
        .set_private_key_file(key_file, SslFiletype::PEM)
        .expect("set_private_key_file error");
    builder
        .set_certificate_chain_file(cert_file)
        .expect("set_certificate_chain_file error");
    builder.set_verify(SslVerifyMode::NONE);
    builder
}
