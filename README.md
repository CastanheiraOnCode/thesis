## THESIS WORK

Repo that will hold all my advances in order to develop my thesis.

# Notes

# First Steps
    Capture packets from my computer
    Display them with Elastic
    Process them with Python
    
	
https://medium.com/@0xgradius/containerizing-my-nsm-stack-docker-suricata-and-elk-5be84f17c684

# Suricata
    suricata.readthedocs.io


# Configuration Steps

1. # Ensure that the traffic from the monitored ports is directed to the monitoring system.
    - Configure the network devices to forward the traffic to the monitoring system.
        - Set up port mirroring or network taps if necessary.

The interface that we will be using to this is **wlo1**

Let's set it up for traffic sniffing:
        `ip link set wlo1 multicast off`
        `ip link set wlo1 promisc on`
        `ip link set wlo1 up`


2.  ## Install Docker 
    - Remove old Docker configuration
    - Install using the apt repository

3. ## Docker containers for ElasticSearch and Kibana
    - Let' s use the containers for ElasticSearch and Kibana    
        `docker pull ubuntu`
        `docker pull docker.elastic.co/elasticsearch/elasticsearch:6.1.1`
        `docker pull docker.elastic.co/kibana/kibana:6.1.1
        docker pull docker.elastic.co/logstash/logstash:6.1.1`
    - ## Launch Elastic container
        `docker run -d -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" --hostname=elastic --name=elastic --network=host -t --mount source=elastic,destination=/usr/share/elasticsearch/data docker.elastic.co/elasticsearch/elasticsearch:7.17.20`
    - ## Launch Kibana container
    `docker run -d -e ELASTICSEARCH_URL="http://localhost:9200" --hostname=kibana --name=kibana --network=host -p 5601:5601 -t docker.elastic.co/kibana/kibana:7.17.20`
    - ## Launch Logstash container
        First of all use elastic_beats_configuration in order to parse and format the Suricata EVE Json logs sent via Filebeats before shipping them to the Elastic Search
    `docker run -d --hostname=logstash --name=logstash --network="host" -e "xpack.monitoring.elasticsearch.url=http://localhost:9200" -t logstash`
    - ## Launch Suricata container

        `docker build -t suricata .`
        `docker run -d --network=host --hostname=suricata --name=suricata -it suricata`



Filebeat is installed in
/var/lib/suricata/filebeat-8.13.2-linux-x86_64


Changed password for user apm_system
PASSWORD apm_system = kz3ve7ANx8jdvoS0zfQE
dock
Changed password for user kibana_system
PASSWORD kibana_system = 9CehyCXuZFc6f1sfQJbw

Changed password for user kibana
PASSWORD kibana = 9CehyCXuZFc6f1sfQJbw

Changed password for user logstash_system
PASSWORD logstash_system = o8WUAeY8ibG75WYuO3et

Changed password for user beats_system
PASSWORD beats_system = WHjPdStb6aTtIieQxqF5

Changed password for user remote_monitoring_user
PASSWORD remote_monitoring_user = v4QdNcvIfyyFiawUTtde

Changed password for user elastic
PASSWORD elastic = kyRUwAYXtIe5tEZRMY1f


kibana config   
cat <<EOL > kibana.yml
#
# ** THIS IS AN AUTO-GENERATED FILE **
#

# Default Kibana configuration for docker target
server.host: "0.0.0.0"
server.shutdownTimeout: "5s"
elasticsearch.hosts: ["http://elastic:kyRUwAYXtIe5tEZRMY1f@elasticsearch:9200"]
elasticsearch.username: "elastic"
elasticsearch.password: "kyRUwAYXtIe5tEZRMY1f"
monitoring.ui.container.elasticsearch.enabled: true
EOL

### Sep 23 iteration

https://www.elastic.co/guide/en/elasticsearch/reference/current/run-elasticsearch-locally.html

Single node deployment do elastic 

``` docker run -p 127.0.0.1:9200:9200 -d --name elasticsearch --network elastic-net \
  -e ELASTIC_PASSWORD=$ELASTIC_PASSWORD \
  -e "discovery.type=single-node" \
  -e "xpack.security.http.ssl.enabled=false" \
  -e "xpack.license.self_generated.type=basic" \
  docker.elastic.co/elasticsearch/elasticsearch:8.15.1
```

### Environment vars

export ELASTIC_PASSWORD="elastic"
export KIBANA_PASSWORD="kibana"


### Create elastic network

docker network create elastic-net

# Kibana conf

## set kibana system 

### configure the Kibana password in the ES container
curl -u elastic:$ELASTIC_PASSWORD \
  -X POST \
  http://localhost:9200/_security/user/kibana_system/_password \
  -d '{"password":"'"$KIBANA_PASSWORD"'"}' \
  -H 'Content-Type: application/json'

### Start kibana container

docker run -p 127.0.0.1:5601:5601 -d --name kibana --network elastic-net \
  -e ELASTICSEARCH_URL=http://elasticsearch:9200 \
  -e ELASTICSEARCH_HOSTS=http://elasticsearch:9200 \
  -e ELASTICSEARCH_USERNAME=kibana_system \
  -e ELASTICSEARCH_PASSWORD=$KIBANA_PASSWORD \
  -e "xpack.security.enabled=false" \
  -e "xpack.license.self_generated.type=trial" \
  docker.elastic.co/kibana/kibana:8.15.1

### Connect with python

https://www.elastic.co/guide/en/elasticsearch/reference/current/run-elasticsearch-locally.html#local-dev-connecting-clients


### Setup to install the latest stable Suricata:

sudo apt-get install software-properties-common
sudo add-apt-repository ppa:oisf/suricata-stable
sudo apt-get update

#### Basic Setup

Change the af-packet section to my interface

ip addr output:

```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: eno1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN group default qlen 1000
    link/ether a8:b1:3b:79:ca:43 brd ff:ff:ff:ff:ff:ff
    altname enp2s0
4: wlo1: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    link/ether 4c:d5:77:f7:3a:e1 brd ff:ff:ff:ff:ff:ff
    altname wlp3s0
    inet 192.168.1.68/24 brd 192.168.1.255 scope global dynamic noprefixroute wlo1
       valid_lft 2793sec preferred_lft 2793sec
    inet6 2001:8a0:ee80:1a00:f301:7e0a:a93b:15f6/64 scope global temporary dynamic 
       valid_lft 89960sec preferred_lft 79683sec
    inet6 2001:8a0:ee80:1a00:11a4:fc6:da2b:54f6/64 scope global dynamic mngtmpaddr noprefixroute 
       valid_lft 89960sec preferred_lft 89960sec
    inet6 fe80::b3a0:c230:6a4e:f824/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
6: br-c68339c083a2: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default 
    link/ether 02:42:73:8d:18:a5 brd ff:ff:ff:ff:ff:ff
    inet 172.80.6.1/24 brd 172.80.6.255 scope global br-c68339c083a2
       valid_lft forever preferred_lft forever
    inet6 fe80::42:73ff:fe8d:18a5/64 scope link 
       valid_lft forever preferred_lft forever
10: br-7553cced7c86: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether 02:42:c0:1b:09:92 brd ff:ff:ff:ff:ff:ff
    inet 172.80.0.1/24 brd 172.80.0.255 scope global br-7553cced7c86
       valid_lft forever preferred_lft forever
    inet6 fe80::42:c0ff:fe1b:992/64 scope link 
       valid_lft forever preferred_lft forever
15: docker0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN group default 
    link/ether 02:42:08:4b:6d:f8 brd ff:ff:ff:ff:ff:ff
    inet 10.139.0.1/24 brd 10.139.0.255 scope global docker0
       valid_lft forever preferred_lft forever
17: veth7dcf232@if16: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-c68339c083a2 state UP group default 
    link/ether ca:6e:62:a0:c2:93 brd ff:ff:ff:ff:ff:ff link-netnsid 0
    inet6 fe80::c86e:62ff:fea0:c293/64 scope link 
       valid_lft forever preferred_lft forever
```
 So, we will change the suricata.yaml with

```
sudo nano /etc/suricata/suricata.yaml
```

and change the af-packet section for enp2s0

We also set __outputs__ to process ethernet headers

### We will now use Elastic Agent
Which is a single, unified way to add monitoring for logs, metrics and othert types of data to a host.

In a diagram, it would position itself between the monitored service (Suricata, more precisely the __eve.json__ folder) and the Elastic Search

We will be using the Elastic Agent to make use of the Elastic Integrations, which are a way to connect Elastic to external services and systems and quickly get insights. This was chosen because there is already an Integration for Suricata

We will be using the Elastic Integration Tutorial provided from here 
https://www.elastic.co/guide/en/observability/8.15/logs-metrics-get-started.html

in order to complete, we need to install an Elastic Agent
and a Elastic Fleet - which is on localhost:8220


## Install Fleet Server to a centralized host

```

curl -L -O https://artifacts.elastic.co/downloads/beats/elastic-agent/elastic-agent-8.15.1-linux-x86_64.tar.gz
tar xzvf elastic-agent-8.15.1-linux-x86_64.tar.gz
cd elastic-agent-8.15.1-linux-x86_64
sudo ./elastic-agent install \
  --fleet-server-es=http://localhost:9200 \
  --fleet-server-service-token=AAEAAWVsYXN0aWMvZmxlZXQtc2VydmVyL3Rva2VuLTE3MjcxMDE0ODA3ODY6ZVhQdTlYeDhUSS1DOHI4c3E1X0JxZw \
  --fleet-server-policy=fleet-server-policy \
  --fleet-server-port=8220

  ```

## Let's now add the Elastic Agent

### 2 Install Elastic Agent on your host

```
curl -L -O https://artifacts.elastic.co/downloads/beats/elastic-agent/elastic-agent-8.15.1-linux-x86_64.tar.gz
tar xzvf elastic-agent-8.15.1-linux-x86_64.tar.gz
cd elastic-agent-8.15.1-linux-x86_64
sudo ./elastic-agent install --url=https://localhost:8220 --enrollment-token=ajlFckg1SUJhXzAySjgxczhQcmo6V19ZYnMzdUFSb2FhVXlhSjhtRE9kdw==
```

### Creating the docker-compose for an easy setup

Started by containerizing the ELK stack elements
Now, we will alter the kibana image, add the suricata integration and use that image as our kibana image in the docker compose 


## Ansible 

Vamos usar SSH para configurar 

```
$ python -m pip install --user ansible
```
Code to install ansible on the control Node

## Generate an API key