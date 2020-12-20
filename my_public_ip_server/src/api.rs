use actix_web::{error::BlockingError, get, put, web, HttpRequest, HttpResponse};

use my_public_ip_lib::{Reader, Writer};

use crate::{ConfigKeys, Error, Result, Store};

#[derive(Clone)]
pub struct ApiState {
    config: ConfigKeys,
    store: Store,
}

impl ApiState {
    pub fn new(config: ConfigKeys, store: Store) -> ApiState {
        ApiState { config, store }
    }
}

#[get("/")]
pub async fn list_ips(api_state: web::Data<ApiState>, req: HttpRequest) -> Result<HttpResponse> {
    let (api_key, ip) = get_api_key_and_ip(&req)?;

    let api_state = api_state.into_inner();
    let reader = Reader {
        ip,
        updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
    };

    wrap_block_res(
        web::block(move || crate::list_ips(&api_state.config, &api_state.store, &api_key, &reader))
            .await,
    )
}

#[put("/")]
pub async fn update_ip(api_state: web::Data<ApiState>, req: HttpRequest) -> Result<HttpResponse> {
    let (api_key, ip) = get_api_key_and_ip(&req)?;

    let api_state = api_state.into_inner();
    let writer = Writer {
        ip,
        updated_at: time::OffsetDateTime::now_utc().unix_timestamp(),
    };

    wrap_block_res(
        web::block(move || {
            crate::update_ip(&api_state.config, &api_state.store, &api_key, &writer)
        })
        .await,
    )
}

fn wrap_block_res<T: serde::Serialize>(
    res: std::result::Result<T, BlockingError<Error>>,
) -> Result<HttpResponse> {
    res.map(|res| HttpResponse::Ok().json(res))
        .map_err(Into::into)
}

fn get_api_key_and_ip(req: &HttpRequest) -> Result<(String, String)> {
    let api_key = req
        .headers()
        .get("APIKEY")
        .ok_or(Error::InvalidReaderKey)?
        .to_str()?
        .to_string();

    let ip = req
        .peer_addr()
        .ok_or(Error::ReadIpAddrError)?
        .ip()
        .to_string();

    Ok((api_key, ip))
}
