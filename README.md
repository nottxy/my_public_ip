
# Run server
```shell
RUST_LOG=info MY_PUBLIC_IP_CONFIG=my_public_ip_server/my_public_ip.toml MY_PUBLIC_IP_DB=db MY_PUBLIC_IP_PORT=8080 cargo run -p my_public_ip_server
```

# Run client
```shell
cargo run -p my_public_ip_client -- http://192.168.0.17:8080 --api-key=abcdef list

cargo run -p my_public_ip_client -- http://192.168.0.17:8080 --api-key=012345 update
```