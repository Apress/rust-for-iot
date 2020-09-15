
# Helm Charts

helm install iot --dry-run --debug
- run to make hte difference


## Applications We Use

### Sealed Secrets
https://github.com/bitnami-labs/sealed-secrets
https://github.com/helm/charts/tree/master/stable/sealed-secrets
- helm chart sealed secrets

### PostGres

### EMQTT
https://github.com/emqx/emqx-chart


envFrom:
    - configMapRef:
        name: {{ include "emqx.fullname" . }}-env
* uses a configMap Ref to get more env variables
- which in thie end uses :
emqxConfig
- this is your list of values in the YAML fie

### EventStore
https://github.com/grigorov/eventstore-helm
https://github.com/grigorov/eventstore-helm/releases/tag/0.3.0

helm upgrade --install eventstore eventstore

--set ingress.enabled=true --set ingress.hosts[0]=eventstore.<domainname> --set persistence.enabled=true --set fullnameOverride=eventstore
* we dont need to create ingress since this is going to be done only internally


helm upgrade --install --namespace iot cqrs-es -f eventstore_values.yaml eventstore


## Our Services

### MQTT Service

### Retrieval Service

### Upload Service







#
#  docker run --restart=always -ti --name emqtt-auth --net=iot \
#  -p 8883:8883 -p 18083:18083 -p 8083:8083 -p 8443:8443 -p 8084:8084 -p 8080:8080 \    # <1>
#  -v ~/book_certs:/etc/ssl/certs/ \  # <2>
#  -e EMQ_LISTENER__SSL__EXTERNAL__KEYFILE="\/etc\/ssl\/certs\/EmqttIot.key" \ # <3>
#  -e EMQ_LISTENER__SSL__EXTERNAL__CERTFILE="\/etc\/ssl\/certs\/EmqttIot.pem" \
#  -e EMQ_LISTENER__SSL__EXTERNAL__CACERTFILE="\/etc\/ssl\/certs\/RustIOTRootCA.pem" \
#  -e EMQ_LISTENER__SSL__EXTERNAL__VERIFY=verify_peer \
#  -e EMQ_LISTENER__SSL__EXTERNAL__FAIL_IF_NO_PEER_CERT=true \
#  -e "EMQ_LOG_LEVEL=debug" \
#  -e "EMQ_ADMIN_PASSWORD=your_password" \
#  -d devrealm/emqtt