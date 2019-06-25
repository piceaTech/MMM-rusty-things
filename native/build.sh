npm install

docker run --volume $PWD:/home/node/project \
  --volume $PWD/native/pi_deps:/home/node/deb-deps \
  --volume ~/.cargo/registry:/home/node/.cargo/registry \
  -e RUST_BACKTRACE=1 \
  -e CARGO_BUILD_TARGET="arm-unknown-linux-gnueabihf" \
  -e npm_config_arch=arm \
  neon-pi-cross:latest