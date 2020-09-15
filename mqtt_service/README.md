# MQTT Service

- Monitors heartbeat
- Sends on Recording to the MQTT

## Ports
HTTP: 3005
MQTT: 1883 (nonssl) 8883 (ssl)

## Running 

cargo run --features "full" -- --http_server 0.0.0.0

# Topics Subscribed To

# URLs
- config
-- Config change sent from our application
- recording/start
-- start active recording
- recording/s
-- stop active recoridng

# Topics
- health/$id
-- The health data of the box sent to our database
- motion/$id
-- this will be used to detect the motion and if its their send it ot hte server
- config
-- config change pushed from the device

## Topics Sent from the App
- recording/$id/$action
-- recordigns to start or stop

## Create the EMQTT
docker pull emqx/emqx:latest
docker run --rm -ti --name emqx -p 18083:18083 -p 1883:1883 emqx/emqx:latest
** not this one

-- Running without Auth

docker pull devrealm/emqtt
docker run --restart=always -ti --name emqtt-no-auth -p 18083:18083 -p 1883:1883 -p 8083:8083 -p 8443:8443 -p 8084:8084 -p 8080:8080 -e "EMQ_LOG_LEVEL=debug" -e "EMQ_ADMIN_PASSWORD=your_password" -e "EMQ_NAME=emqtt_broker" -d devrealm/emqtt


https://github.com/devrealm/emq-docker
- running with authentication

docker run --restart=always -ti --name emqtt-auth -p 18083:18083 -p 1883:1883 -p 8083:8083 -p 8443:8443 -p 8084:8084 -p 8080:8080 -v /etc/letsencrypt/live/yourdomain.com/privkey.pem:/opt/emqttd/etc/certs/key.pem -v /etc/letsencrypt/live/yourdomain.com/fullchain.pem:/opt/emqttd/etc/certs/cert.pem -e "EMQ_LOG_LEVEL=debug" -e "EMQ_ADMIN_PASSWORD=your_password"  -d devrealm/emqtt


# Publish
## No Auth
mosquitto_sub -d -h localhost -p 1883 -t "$SYS/broker/clients/connected" --tls-version tlsv1.2

mosquitto_sub -d -h localhost -p 1883 -t "health/+" --tls-version tlsv1.2
mosquitto_sub -d -h localhost -p 1883 -t "recording/+" --tls-version tlsv1.2

mosquitto_pub -d -h localhost -p 1883 -t "health/24f89ac6-e53d-418b-be6d-2a7571aa483f" --tls-version tlsv1.2 -f health.json

## HTTP
http post http://iot-backend:3005/api/recording/7a386bec-54d4-4479-b6ab-84c8f615a64e/start
http POST http://iot-backend:3005/api/recording/7a386bec-54d4-4479-b6ab-84c8f615a64e/stop

http get http://iot-backend:3005/api/healthz


# Data Size
Json: 82 bytes
Cap: 184 bytes


{
 "uuid": "9cf81814-1df0-49ca-9bac-0b32283eb29b",
 "status": "Green",
 "msg": "Here we define what could be going on with our application and its items.",
 "timestamp": 1562453750553,
 "peripherals":[{ "name": "Camera" }, { "name": "Temp" }]
}
