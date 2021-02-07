
# Run server
```shell
cargo run -p my_public_ip_server -- \
  version
  
  
cargo run -p my_public_ip_server -- \
  run \
  --config-file server/config.toml \
  --db-dir db \
  --log-file server/log4rs.yaml \
  --pid-file server/my_public_ip_server.pid \
  --cert-file server/cert.pem \
  --key-file server/key.pem \
  --port 8998
```

# Run client
```shell
RUST_LOG=info cargo run -p my_public_ip_client -- \
  version
  
  
RUST_LOG=info cargo run -p my_public_ip_client -- \
  list \
  --url=https://127.0.0.1:8998 \
  --api-key=abcdef


RUST_LOG=info cargo run -p my_public_ip_client -- \
  update \
  --url=https://127.0.0.1:8998 \
  --api-key=012345


RUST_LOG=info cargo run -p my_public_ip_client -- \
  update-forever \
  --url=https://127.0.0.1:8998 \
  --api-key=012345 \
  --interval=3
```