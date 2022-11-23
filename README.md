

## Docker

```
docker run -v $(pwd):/app -w /app rust:bullseye cargo build --release
```


```
docker run -it -v $(pwd):/app -w /app rust:bullseye bash

apt-get update -y 
apt-get install -y libxcb1 libxrandr2 libdbus-1-3 libdbus-1-dev pkg-config
cargo build --target=x86_64-pc-windows-gnu --release

```