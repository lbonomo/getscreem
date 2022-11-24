


## Run in local (Linux Debian 11)

apt-get install -y mingw-w64 libxcb1 libxrandr2 libdbus-1-3 libdbus-1-dev pkg-config libgtk-3-dev

`cargo run`

### Building Windows app in Linux
https://bevy-cheatbook.github.io/setup/cross/linux-windows.html

```
apt-get install -y mingw-w64 libxcb1 libxrandr2 libdbus-1-3 libdbus-1-dev pkg-config libgtk-3-dev
rustup target add x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu --release
```

### Building Windows app in Linux (Microsoft Visual C++)

Esta aparenta ser la mejor opsion, el binario es mucho más chico.

```
rustup target add x86_64-pc-windows-msvc

cargo install xwin
sudo mkdir /opt/xwin
sudo chown lbonomo:lbonomo /opt/xwin
xwin --accept-license splat --output /opt/xwin

cat /home/lbonomo/.cargo/config.toml 
[target.x86_64-pc-windows-msvc]
linker = "lld"
rustflags = [
  "-Lnative=/opt/xwin/crt/lib/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64"
]

sudo apt install lld

cargo build --target=x86_64-pc-windows-msvc --release
```

## Documentation
