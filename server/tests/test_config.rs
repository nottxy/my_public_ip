use my_public_ip_server::{Config, ConfigKeys};

#[test]
fn test_default() {
    let config: ConfigKeys = Config::default().into();

    assert_eq!(Some("guard"), config.get_reader_name("abcdef"));
    assert_eq!(None, config.get_reader_name("abcdefg"));

    assert_eq!(Some("test"), config.get_writer_name("012345"));
    assert_eq!(None, config.get_writer_name("0123456"));
}
