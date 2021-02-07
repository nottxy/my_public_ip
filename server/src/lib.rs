pub mod api;
pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod store;

pub use {
    config::{Config, ConfigKeys},
    error::{Error, Result},
    store::Store,
};

use my_public_ip_lib::{PublicIp, Reader, Writer};

pub fn update_ip(
    config: &ConfigKeys,
    store: &Store,
    key: &str,
    writer: &Writer,
) -> Result<Option<Writer>> {
    let writer_name = config.get_writer_name(key).ok_or(Error::InvalidWriterKey)?;
    store.set_writer(writer_name, writer)
}

pub fn list_ips(
    config: &ConfigKeys,
    store: &Store,
    key: &str,
    reader: &Reader,
) -> Result<Vec<PublicIp>> {
    let reader_name = config.get_reader_name(key).ok_or(Error::InvalidReaderKey)?;
    let public_ips = store.list_writer()?;
    store.set_reader(reader_name, reader)?;
    Ok(public_ips)
}
