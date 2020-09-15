# Rasp Pi MQ

## Install
Need to make sure capnpn is installed first.

https://capnproto.org/install.html
- all the installation instructions are there

docker build -t  

cargo build --target x86_64-unknown-linux-musl --release
cross build --target=armv7-unknown-linux-gnueabihf

## Copy to the Pi
scp target/armv7-unknown-linux-gnueabihf/debug/rasp-pi-mq pi@pi:/home/pi/rasp-pi-mq
- copies to the raspberry pi 

## Runs

/home/pi/rasp-pi-mq -s 192.168.4.39