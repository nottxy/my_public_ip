
# Run server
```shell
cargo run -p my_public_ip_server -- \
  --config-file my_public_ip_server/config.toml \
  --db-dir db \
  --log-file my_public_ip_server/log4rs.yaml \
  --pid-file my_public_ip_server/my_public_ip_server.pid \
  --port 8998
```

# Run client
```shell
cargo run -p my_public_ip_client -- \
  http://192.168.0.17:8080 \ 
  --api-key=abcdef list

cargo run -p my_public_ip_client -- \
  http://192.168.0.17:8080 \
  --api-key=012345 update
```