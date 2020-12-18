use my_public_ip_server::Store;

pub fn new_test_store() -> Store {
    let db = sled::Config::new()
        .temporary(true)
        .open()
        .expect("can not create test db");
    Store::new(db).expect("can not create test store")
}
