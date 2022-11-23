

## Docker

```
docker run -v $(pwd):/app -w /app rust:bullseye cargo build --release
```

```
docker run -it -v $(pwd):/app -w /app rust:bullseye bash
```


### Building Windows app in Linux
```
apt-get install -y mingw-w64 libxcb1 libxrandr2 libdbus-1-3 libdbus-1-dev pkg-config
rustup target add x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu --release
```