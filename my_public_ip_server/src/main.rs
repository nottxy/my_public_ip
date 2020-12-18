use actix_web::{App, HttpServer};
use std::env;

use my_public_ip_server::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let api_state = api::ApiState::default();

    let port =
        env::var("MY_PUBLIC_IP_PORT").expect("the MY_PUBLIC_IP_CONFIG var in env is missing");
    let port: u16 = port.parse().expect("invalid MY_PUBLIC_IP_PORT");
    let addr = format!("0.0.0.0:{}", port);

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
