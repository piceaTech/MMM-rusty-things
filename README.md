# MMM-rusty-things

MagicMirror module to access your todo-entries from Things and display them in MM.
It was a learning project for me where I learnt about how to interact between Rust and Node.

# Screenshot

The module looks like this:



![Example of MMM-rusty-things publishing todos](img/MMM-rusty-things-screenshot.png)

# Installation

```
git clone https://github.com/piceaTech/MMM-rusty-things.git
cd MMM-rusty-things.git
npm install
mv example.env .env
```
Edit the `.env` and insert your hist_id. 

After that either compile the native module locally on the pi or cross-compile from your desktop.

## No cross-compilation

In the folder `MMM-rusty-things` execute:
```neon build --release```

(This runs for approximately 30 minutes.)

## Cross-compilation (e.g. from x64 to arm)
1. Build the build-Container
  1. `git clone https://github.com/piceaTech/rust-on-raspberry-docker`
  1. `cd rust-on-raspberry-docker`
  1. `git checkout neon`
  1. If you need another node version than the current LTS-Version: Edit the Dockerfile and switch to correct BaseImage.
  1. `docker build --tag "neon-pi-cross:latest" .`
1. Copy the following dependencies as `.deb`s into `native/pi_deps`
  1. ssl: http://ftp.debian.org/debian/pool/main/o/openssl1.0/libssl1.0-dev_1.0.2r-1~deb9u1_armhf.deb
  1. sqlite: http://ftp.debian.org/debian/pool/main/s/sqlite3/libsqlite3-dev_3.16.2-5+deb9u1_armhf.deb
1. Inside your neon-folder run `native/build.sh`. This should create a `native/index.node` which should be compatible with arm.
1. Deploy this artifact to the pi!
1. Done