## update and install some things we should probably have
apt-get update
apt-get upgrade -y
apt-get install -y \
  apt-utils \
  curl \
  git \
  gnupg2 \
  jq \
  build-essential \
  openssl \
  libssl-dev \
  pkg-config \
  cmake \
  wget \
  file \
  ca-certificates \
  zstd \
  clang

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y
rustup install default
rustup component add rustfmt
rustup component add clippy
source $HOME/.cargo/env
cargo install cargo-expand
cargo install cargo-edit

## Install Buck2 and Reindeer
wget https://github.com/facebook/buck2/releases/download/2025-02-01/buck2-x86_64-unknown-linux-musl.zst
zstd -d /home/buck2-x86_64-unknown-linux-musl.zst
mv /home/buck2-x86_64-unknown-linux-musl /home/buck2
chmod +x /home/buck2
mv /home/buck2 /usr/local/bin/buck2
cargo install --locked --git https://github.com/facebookincubator/reindeer reindeer
