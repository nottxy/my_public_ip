use my_public_ip_lib::{Reader, Writer};

mod common;

#[test]
fn test_reader() {
    let store = common::new_test_store();

    let reader = store.get_reader("guard").expect("can not get reader");
    assert!(reader.is_none());

    let reader = Reader {
        ip: "127.0.0.1".to_string(),
        updated_at: 1,
    };
    store
        .set_reader("guard", &reader)
        .expect("can not set reader");

    let reader2 = store.get_reader("guard").expect("can not get reader");
    let reader2 = reader2.expect("reader is None");
    assert_eq!("127.0.0.1", reader2.ip);
    assert_eq!(1, reader2.updated_at);
}

#[test]
fn test_writer() {
    let store = common::new_test_store();

    let writer = store.get_writer("test").expect("can not get writer");
    assert!(writer.is_none());

    let writer = Writer {
        ip: "192.168.0.1".to_string(),
        updated_at: 2,
    };
    store
        .set_writer("test", &writer)
        .expect("can not set writer");

    let writer2 = store.get_writer("test").expect("can not get writer");
    let writer2 = writer2.expect("writer is None");
    assert_eq!("192.168.0.1", writer2.ip);
    assert_eq!(2, writer2.updated_at);
}
