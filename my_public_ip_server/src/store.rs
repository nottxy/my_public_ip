use std::env;

use my_public_ip_lib::{PublicIp, Reader, Writer};

use crate::error::Result;

#[derive(Clone)]
pub struct Store {
    db: sled::Db,
    writer_tree: sled::Tree,
    reader_tree: sled::Tree,
}

impl Store {
    pub fn new(db: sled::Db) -> Result<Store> {
        let writer_tree = db.open_tree("writer")?;
        let reader_tree = db.open_tree("reader")?;
        Ok(Store {
            db,
            writer_tree,
            reader_tree,
        })
    }

    pub fn get_writer(&self, name: &str) -> Result<Option<Writer>> {
        let val = self.writer_tree.get(name)?;
        val.map(|val| serde_json::from_slice(val.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    pub fn set_writer(&self, name: &str, writer: &Writer) -> Result<Option<Writer>> {
        let writer_str = serde_json::to_string(writer)?;

        let last_writer = self.writer_tree.insert(name, writer_str.as_bytes())?;

        last_writer
            .map(|val| serde_json::from_slice(val.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    pub fn get_reader(&self, name: &str) -> Result<Option<Reader>> {
        let val = self.reader_tree.get(name)?;
        val.map(|val| serde_json::from_slice(val.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    pub fn set_reader(&self, name: &str, reader: &Reader) -> Result<Option<Reader>> {
        let reader_str = serde_json::to_string(reader)?;

        let last_reader = self.reader_tree.insert(name, reader_str.as_bytes())?;

        last_reader
            .map(|val| serde_json::from_slice(val.as_ref()))
            .transpose()
            .map_err(Into::into)
    }

    pub fn list_writer(&self) -> Result<Vec<PublicIp>> {
        let iter = self.writer_tree.iter();

        let (lower, upper) = iter.size_hint();
        let mut public_ips = Vec::with_capacity(upper.unwrap_or(lower));

        for row in iter {
            let (k, v) = row?;
            let writer: Writer = serde_json::from_slice(v.as_ref())?;
            public_ips.push(PublicIp {
                name: String::from_utf8_lossy(k.as_ref()).into_owned(),
                ip: writer.ip,
                updated_at: writer.updated_at,
            });
        }

        Ok(public_ips)
    }

    pub fn flush(&self) -> Result<()> {
        self.db.flush().map(|_| ()).map_err(Into::into)
    }
}

impl Default for Store {
    fn default() -> Store {
        let file_path =
            env::var("MY_PUBLIC_IP_DB").expect("the MY_PUBLIC_IP_DB var in env is missing");
        let db = sled::open(&file_path).expect("could not open MY_PUBLIC_IP_DB");

        Store::new(db).expect("could not create store")
    }
}
