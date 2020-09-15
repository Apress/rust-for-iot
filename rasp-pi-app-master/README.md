
# Raspberry Pi Application Master

## Building
cross build --features "ch09" --target=armv7-unknown-linux-gnueabihf

cross build --features "full" --target=armv7-unknown-linux-gnueabihf

## Deploying
scp target/armv7-unknown-linux-gnueabihf/debug/rasp-app pi@pi3:/home/pi/rasp-app

## Running the Raspberry Pi

sudo ./rasp-app -r /home/ubuntu/RustIOTRootCA.pem -c /home/ubuntu/PiDevice.pem -k /home/ubuntu/PiDevice.key

-s (for theserver)

/var/tokenstorage.json
- make sure this file is empty

scp target/armv7-unknown-linux-gnueabihf/debug/rasp-app pi@pi:/home/pi/rasp-app