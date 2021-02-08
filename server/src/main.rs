use actix_web::{middleware, App, HttpServer};
use log::debug;
use structopt::StructOpt;

use my_public_ip_server::{api, Config, Store};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslVerifyMode};

#[derive(Debug, StructOpt)]
#[structopt(name = "my_public_ip_server", about = "My public ip server")]
struct Opt {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    run(&opt).await
}

async fn run(opt: &Opt) -> std::io::Result<()> {
    log4rs::init_file(&opt.log_file, Default::default()).expect("init log4rs error");

    write_pid_file(&opt.pid_file)?;

    let config = Config::load(&opt.config_file)
        .expect("load config error")
        .into();
    let store = Store::open(&opt.db_dir).expect("open db_dir error");
    let api_state = api::ApiState::new(config, store);

    let addr = format!("0.0.0.0:{}", opt.port);

    let ssl_builder = build_ssl_builder(&opt.cert_file, &opt.key_file);

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
