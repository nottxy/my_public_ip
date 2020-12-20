use actix_web::{App, HttpServer};
use log::debug;
use structopt::StructOpt;

use my_public_ip_server::{api, Config, Store};

#[derive(Debug, StructOpt)]
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    log4rs::init_file(&opt.log_file, Default::default()).expect("init log4rs error");
    debug!("opt: {:?}", opt);

    write_pid_file(&opt.pid_file)?;

    let config = Config::load(&opt.config_file)
        .expect("load config error")
        .into();
    let store = Store::open(&opt.db_dir).expect("open db_dir error");
    let api_state = api::ApiState::new(config, store);

    let addr = format!("0.0.0.0:{}", opt.port);

    HttpServer::new(move || {
        App::new()
            .data(api_state.clone())
            .service(api::list_ips)
            .service(api::update_ip)
    })
    .bind(addr)?
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
