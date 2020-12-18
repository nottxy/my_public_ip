use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PublicIp {
    pub name: String,
    pub ip: String,
    pub updated_at: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Writer {
    pub ip: String,
    pub updated_at: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Reader {
    pub ip: String,
    pub updated_at: i64,
}
