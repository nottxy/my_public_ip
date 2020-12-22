
# Run server
```shell
cargo run -p my_public_ip_server -- \
  --config-file my_public_ip_server/config.toml \
  --db-dir db \
  --log-file my_public_ip_server/log4rs.yaml \
  --pid-file my_public_ip_server/my_public_ip_server.pid \
  --cert-file my_public_ip_server/cert.pem \
  --key-file my_public_ip_server/key.pem \
  --port 8998
```

# Run client
```shell
cargo run -p my_public_ip_client -- \
  https://127.0.0.1:8998 \
  --api-key=abcdef list

cargo run -p my_public_ip_client -- \
  https://127.0.0.1:8998 \
  --api-key=012345 update
```