RUSTFLAGS="-L $PWD/pi_deps/usr/lib/gcc/arm-linux-gnueabihf/12/" \
# CC="armv7-unknown-linux-gnueabihf-cc" \ # apparrently no longer needed
PKG_CONFIG_ALLOW_CROSS=true \
OPENSSL_INCLUDE_DIR="$PWD/pi_deps/usr/include" \
OPENSSL_LIB_DIR="$PWD/pi_deps/usr/lib/arm-linux-gnueabihf" \
RUST_BACKTRACE=1 \
CARGO_BUILD_TARGET="arm-unknown-linux-gnueabihf" \
CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true \
npm_config_arch=arm \
npm run build-release