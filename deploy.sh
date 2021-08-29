#!/usr/bin/env bash
cargo fmt
cargo build
cargo clippy --all

ssh pi@192.168.0.100 <<'ENDSSH'
  mkdir -p hcl/home_source
  cd hcl/home_source
  ls | grep -v target | xargs rm -rfv
ENDSSH

scp -r ./src pi@192.168.0.100:/home/pi/hcl/home_source/
scp ./Cargo.lock pi@192.168.0.100:/home/pi/hcl/home_source/
scp ./Cargo.toml pi@192.168.0.100:/home/pi/hcl/home_source/

ssh pi@192.168.0.100 <<'ENDSSH'
  cd hcl/home_source
  cargo build --release
  sudo systemctl stop home
  rm /home/pi/hcl/home
  cp target/release/home /home/pi/hcl/home
  sudo systemctl start home
ENDSSH