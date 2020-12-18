use my_public_ip_lib::Writer;
use my_public_ip_server::{update_ip, Config};

mod common;

#[test]
fn test_list_writer() {
    let store = common::new_test_store();

    let public_ips = store.list_writer().expect("can not read public ips");
    assert!(public_ips.is_empty());

    let config = Config::default().into();

    let updated_at = time::OffsetDateTime::now_utc().unix_timestamp();

    let writer = Writer {
        ip: "192.168.0.1".to_string(),
        updated_at,
    };

    update_ip(&config, &store, "012345", &writer).expect("can not update ip");

    let public_ips = store.list_writer().expect("can not read public ips");
    assert_eq!(1, public_ips.len());

    let public_ip = public_ips.first().unwrap();
    assert_eq!("192.168.0.1", public_ip.ip);
    assert_eq!(updated_at, public_ip.updated_at);
    assert_eq!("test", public_ip.name);
}
