# quote_infra

quote_infra is an infraescture that serves as a proxy to the Quote Flaky API

## Installation

Clone the repository:

```
git clone https://github.com/jcalahor/quote_infra.git
cd quote_infra
```

Build the project:
```
cargo build
```

## Build the environment

quote_infra make usage of Redis, Kafka and Elastic Search to operate properly, a docker compose is available for it:

```
cd docker
docker compose up
```

And wait until the enviroment is built

to verify the enviroment is up and running:

```
jcalahor76@ubuntudev:~/development/rust/quote_infra$ docker ps
CONTAINER ID   IMAGE                                                 COMMAND                  CREATED        STATUS                       PORTS                                                                                      NAMES
f1d461d412e7   docker.elastic.co/kibana/kibana:8.7.0                 "/bin/tini -- /usr/l…"   45 hours ago   Up About an hour (healthy)   0.0.0.0:5601->5601/tcp, :::5601->5601/tcp                                                  docker-kibana-1
5b743f912234   docker.elastic.co/elasticsearch/elasticsearch:8.7.0   "/bin/tini -- /usr/l…"   45 hours ago   Up About an hour (healthy)   0.0.0.0:9200->9200/tcp, :::9200->9200/tcp, 9300/tcp                                        docker-elasticsearch-1
5412b8ccfdc8   confluentinc/cp-kafka:latest                          "sh -c 'sleep 10 && …"   3 days ago     Up About an hour             0.0.0.0:9092->9092/tcp, :::9092->9092/tcp, 0.0.0.0:19092->19092/tcp, :::19092->19092/tcp   broker
4e1624fc3401   confluentinc/cp-zookeeper:latest                      "/etc/confluent/dock…"   3 days ago     Up About an hour             2888/tcp, 0.0.0.0:2181->2181/tcp, :::2181->2181/tcp, 3888/tcp                              zookeeper
9af1ca18e402   redis:latest                                          "docker-entrypoint.s…"   4 weeks ago    Up About an hour             0.0.0.0:6379->6379/tcp, :::6379->6379/tcp                                                  docker-redis-1
jcalahor76@ubuntudev:~/development/rust/quote_infra$ 
```

Note: A tweak might be needed in case Elastic Search doesn't start properly (Kibana not opening):
```
curl -X PUT "localhost:9200/_cluster/settings"-H 'Content-Type: application/json'-d '{ "persistent": { "cluster.routing.allocation.enable": "all" } }'
```

## Configuration

Most setting can be edites on a .env file. Most default settings should work except for the ip address of the kafka host, which will need to be setup depending on the ip address
of the host:


```
jcalahor76@ubuntudev:~/development/rust/quote_infra$ cat .env
# Server configuration
SERVER_ADDRESS=0.0.0.0
SERVER_PORT=4000

# Kafka configuration
KAFKA_BOOTSTRAP_SERVERS=192.168.9.183:19092
KAFKA_GROUP_ID=quote_group
KAFKA_TOPIC=quote_feed
KAFKA_CHANNEL_SIZE=100

# Redis configuration
REDIS_URL=redis://127.0.0.1:6379
```

Note: at the moment given the time constraint the url for the Elastic Sarch URL is not configurable and the default is used (local ES)




## Usage


Starting the quote_api_collector

```
cargo run -p quote_api_collector
```

Starting the quote_sinker
```
cargo run -p quote_api_collector
```

Starting the quote_api
```
cargo run -p quote_api
```


## Monitoring

At the moment the only monitoring schema available is via standard logs, which are generateds in /logs folder


```
jcalahor76@ubuntudev:~/development/rust/quote_infra$ cd logs
jcalahor76@ubuntudev:~/development/rust/quote_infra/logs$ ls -l
total 92
-rw-rw-r-- 1 jcalahor76 jcalahor76  3322 feb 11 18:58 2025-02-11_18-56-06@728@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76   142 feb 11 18:56 2025-02-11_18-56-32@841@quote-sinker-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76  3718 feb 11 18:59 2025-02-11_18-58-55@248@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76  2273 feb 11 18:59 2025-02-11_18-58-58@886@quote-sinker-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76   268 feb 11 19:00 2025-02-11_19-00-35@257@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76   268 feb 11 19:01 2025-02-11_19-01-06@774@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76   141 feb 11 19:01 2025-02-11_19-01-25@798@quote-sinker-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76  5375 feb 11 19:02 2025-02-11_19-01-52@330@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76  6745 feb 11 19:02 2025-02-11_19-02-06@211@quote-sinker-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76 13073 feb 11 19:08 2025-02-11_19-04-37@514@quote-api-collector-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76 24303 feb 11 19:08 2025-02-11_19-04-43@803@quote-sinker-output.log
-rw-rw-r-- 1 jcalahor76 jcalahor76    68 feb 11 20:03 2025-02-11_20-03-42@143@quote-api-service-output.log
jcalahor76@ubuntudev:~/development/rust/quote_infra/logs$ 
```

Sample output of the quote_api_collector:

```
2025-02-11 19:04:37.9934523 [INFO] Starting background process...
2025-02-11 19:04:37.994281084 [INFO] Kafka producer successfully created!
2025-02-11 19:04:40.758389225 [INFO] Fetched currencies: ["1inch", "aave", "ada", "aed", "afn", "agix", "akt", "algo", "all", "amd", "amp", "ang", "aoa", "ape", "apt", "ar", "arb", "ars", "atom", "ats", "aud", "avax", "awg", "axs", "azm", "azn", "bake", "bam", "bat", "bbd", "bch", "bdt", "bef", "bgn", "bhd", "bif", "bmd", "bnb", "bnd", "bob", "brl", "bsd", "bsv", "bsw", "btc", "btcb", "btg", "btn", "btt", "busd", "bwp", "byn", "byr", "bzd", "cad", "cake", "cdf", "celo", "cfx", "chf", "chz", "clp", "cnh", "cny", "comp", "cop", "crc", "cro", "crv", "cspr", "cuc", "cup", "cve", "cvx", "cyp", "czk", "dai", "dash", "dcr", "dem", "dfi", "djf", "dkk", "doge", "dop", "dot", "dydx", "dzd", "eek", "egld", "egp", "enj", "eos", "ern", "esp", "etb", "etc", "eth", "eur", "fei", "fil", "fim", "fjd", "fkp", "flow", "flr", "frax", "frf", "ftt", "fxs", "gala", "gbp", "gel", "ggp", "ghc", "ghs", "gip", "gmd", "gmx", "gnf", "gno", "grd", "grt", "gt", "gtq", "gusd", "gyd", "hbar", "hkd", "hnl", "hnt", "hot", "hrk", "ht", "htg", "huf", "icp", "idr", "iep", "ils", "imp", "imx", "inj", "inr", "iqd", "irr", "isk", "itl", "jep", "jmd", "jod", "jpy", "kas", "kava", "kcs", "kda", "kes", "kgs", "khr", "klay", "kmf", "knc", "kpw", "krw", "ksm", "kwd", "kyd", "kzt", "lak", "lbp", "ldo", "leo", "link", "lkr", "lrc", "lrd", "lsl", "ltc", "ltl", "luf", "luna", "lunc", "lvl", "lyd", "mad", "mana", "matic", "mbx", "mdl", "mga", "mgf", "mina", "mkd", "mkr", "mmk", "mnt", "mop", "mro", "mru", "mtl", "mur", "mvr", "mwk", "mxn", "mxv", "myr", "mzm", "mzn", "nad", "near", "neo", "nexo", "nft", "ngn", "nio", "nlg", "nok", "npr", "nzd", "okb", "omr", "one", "op", "ordi", "pab", "paxg", "pen", "pepe", "pgk", "php", "pkr", "pln", "pte", "pyg", "qar", "qnt", "qtum", "rol", "ron", "rpl", "rsd", "rub", "rune", "rvn", "rwf", "sand", "sar", "sbd", "scr", "sdd", "sdg", "sek", "sgd", "shib", "shp", "sit", "skk", "sle", "sll", "snx", "sol", "sos", "spl", "srd", "srg", "std", "stn", "stx", "sui", "svc", "syp", "szl", "thb", "theta", "tjs", "tmm", "tmt", "tnd", "ton", "top", "trl", "trx", "try", "ttd", "tusd", "tvd", "twd", "twt", "tzs", "uah", "ugx", "uni", "usd", "usdc", "usdd", "usdp", "usdt", "uyu", "uzs", "val", "veb", "ved", "vef", "ves", "vet", "vnd", "vuv", "waves", "wemix", "woo", "wst", "xaf", "xag", "xau", "xaut", "xbt", "xcd", "xch", "xdc", "xdr", "xec", "xem", "xlm", "xmr", "xof", "xpd", "xpf", "xpt", "xrp", "xtz", "yer", "zar", "zec", "zil", "zmk", "zmw", "zwd", "zwg", "zwl"]
2025-02-11 19:04:43.468776675 [INFO] Date: 2025-02-11, Exchange Rate from 1INCH to USD: 0.2685759
2025-02-11 19:04:44.012459603 [INFO] Message |{"date":"2025-02-11","rate":0.2685759,"quote":"USD","base":"1INCH","timestamp":1739318683}| sent successfully to topic 'quote_feed'
2025-02-11 19:04:46.658007632 [INFO] Date: 2025-02-11, Exchange Rate from AAVE to USD: 259.33790461
2025-02-11 19:04:46.663824949 [INFO] Message |{"date":"2025-02-11","rate":259.33790461,"quote":"USD","base":"AAVE","timestamp":1739318686}| sent successfully to topic 'quote_feed'
2025-02-11 19:04:47.371667917 [INFO] Date: 2025-02-11, Exchange Rate from ADA to USD: 0.72663771
2025-02-11 19:04:47.375948753 [INFO] Message |{"date":"2025-02-11","rate":0.72663771,"quote":"USD","base":"ADA","timestamp":1739318687}| sent successfully to topic 'quote_feed'
2025-02-11 19:04:50.132164807 [INFO] Date: 2025-02-11, Exchange Rate from AED to USD: 0.27229408
2025-02-11 19:04:50.137043821 [INFO] Message |{"date":"2025-02-11","rate":0.27229408,"quote":"USD","base":"AED","timestamp":1739318690}| sent successfully to topic 'quote_feed'
2025-02-11 19:04:52.776542031 [INFO] Date: 2025-02-11, Exchange Rate from AFN to USD: 0.013484932
2025-02-11 19:04:52.781653967 [INFO] Message |{"date":"2025-02-11","rate":0.013484932,"quote":"USD","base":"AFN","timestamp":1739318692}| sent successfully to topic 'quote_feed'
2025-02-11 19:04:52.782813953 [INFO] Sleeping 5 seconds
2025-02-11 19:05:18.650784878 [INFO] Date: 2025-02-11, Exchange Rate from AGIX to USD: 0.34893102
2025-02-11 19:05:18.659094679 [INFO] Message |{"date":"2025-02-11","rate":0.34893102,"quote":"USD","base":"AGIX","timestamp":1739318718}| sent successfully to topic 'quote_feed'
2025-02-11 19:05:21.346460679 [INFO] Date: 2025-02-11, Exchange Rate from AKT to USD: 1.96969957
2025-02-11 19:05:21.349472732 [INFO] Message |{"date":"2025-02-11","rate":1.96969957,"quote":"USD","base":"AKT","timestamp":1739318721}| sent successfully to topic 'quote_feed'
2025-02-11 19:05:24.094069803 [INFO] Date: 2025-02-11, Exchange Rate from ALGO to USD: 0.29639465
2025-02-11 19:05:24.103588759 [INFO] Message |{"date":"2025-02-11","rate":0.29639465,"quote":"USD","base":
```

Sample output of the quote_sinker:

```
2025-02-11 18:58:58.565688411 [INFO] Kafka consumer successfully created!
2025-02-11 18:58:58.566027029 [INFO] Starting background process...
2025-02-11 18:59:07.878935822 [INFO] Received Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.72663771, quote: "USD", base: "ADA", timestamp: 1739318341 }
2025-02-11 18:59:08.006154845 [INFO] Stored Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.72663771, quote: "USD", base: "ADA", timestamp: 1739318341 }
2025-02-11 18:59:08.615338994 [INFO] Response: {"_index":"quotes","_id":"0acf664b-3315-4ca6-8336-e234403ae40c","_version":1,"result":"created","_shards":{"total":2,"successful":1,"failed":0},"_seq_no":64,"_primary_term":2}
2025-02-11 18:59:08.615363021 [INFO] Quote successfully stored in ESQuoteEnvelope { date: "2025-02-11", rate: 0.72663771, quote: "USD", base: "ADA", timestamp: 1739318341 }
2025-02-11 18:59:08.616229327 [INFO] Received Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.27229408, quote: "USD", base: "AED", timestamp: 1739318344 }
2025-02-11 18:59:08.619283965 [INFO] Stored Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.27229408, quote: "USD", base: "AED", timestamp: 1739318344 }
2025-02-11 18:59:08.635306344 [INFO] Response: {"_index":"quotes","_id":"8caa4643-1c50-4031-b946-5d949f1f8a84","_version":1,"result":"created","_shards":{"total":2,"successful":1,"failed":0},"_seq_no":65,"_primary_term":2}
2025-02-11 18:59:08.63558851 [INFO] Quote successfully stored in ESQuoteEnvelope { date: "2025-02-11", rate: 0.27229408, quote: "USD", base: "AED", timestamp: 1739318344 }
2025-02-11 18:59:26.015074454 [INFO] Received Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.013484932, quote: "USD", base: "AFN", timestamp: 1739318365 }
2025-02-11 18:59:26.017221465 [INFO] Stored Quote: QuoteEnvelope { date: "2025-02-11", rate: 0.013484932, quote: "USD", base: "AFN", timestamp: 1739318365 }
2025-02-11 18:59:26.274592859 [INFO] Response: {"_index":"quotes","_id":"9932406a-e939-4d11-83b9-a114c1cbc221","_version":1,"result":"created","_shards":{"total":2,"successful":1,"failed":0},"_seq_no":66,"_primary_term":2}
2025-02-11 18:59:26.27499171 [INFO] Quote successfully stored in ESQuoteEnvelope { date: "2025-02-11", rate: 0.013484932, quote: "USD", base: "AFN", timestamp: 1739318365 }

```



Inspecting redis:

```
jcalahor76@ubuntudev:~/development/rust/quote_infra/logs$ docker exec -it 9af1ca18e402 redis-cli
127.0.0.1:6379> keys *
 1) "2025-02-11_BGN_USD"
 2) "2025-02-11_AFN_USD"
 3) "2025-02-09_AMP_USD"
 4) "2025-02-08_AED_USD"
 5) "2025-02-11_AED_USD"
 6) "2025-02-08_1INCH_USD"
 7) "2025-02-11_AZN_USD"
 8) "2025-02-11_AMP_USD"
 9) "2025-02-11_BNB_USD"
10) "2025-02-11_ALL_USD"
11) "2025-02-09_AGIX_USD"
12) "2025-02-11_AZM_USD"
13) "2025-02-09_1INCH_USD"
14) "2025-02-11_BHD_USD"
15) "2025-02-11_APT_USD"
16) "2025-02-08_ADA_USD"
17) "2025-02-11_BMD_USD"
18) "2025-02-08_AAVE_USD"
```

Sample quote_api log:

```
2025-02-11 20:03:42.208019689 [INFO] Server started at 0.0.0.0:4000
2025-02-11 20:08:16.573670324 [INFO] Request: QuoteRequest { quote: "AFN", base: "AMD" }
2025-02-11 20:08:16.584617184 [ERROR] [quote_api/src/main.rs:69] One or both quotes not found.
2025-02-11 20:10:06.870662843 [INFO] Request: QuoteRequest { quote: "BGN", base: "AFN" }
2025-02-11 20:10:06.873155539 [INFO] Found quotes: QuoteEnvelope { date: "2025-02-11", rate: 0.013484932, quote: "USD", base: "AFN", timestamp: 1739318692 } - QuoteEnvelope { date: "2025-02-11", rate: 0.52675209, quote: "USD", base: "BGN", timestamp: 1739318873 }
2025-02-11 20:10:06.873172774 [INFO] response with: QuoteResponse { quote: "BGN", base: "AFN", date: "2025-02-11", rate: 39.062272616576784 }
```
Sample usage of the API from Postman:
![image](https://github.com/user-attachments/assets/f5e83a44-e5bc-4f52-987c-260696f3be2d)

Elastic Search output: 
![image](https://github.com/user-attachments/assets/5a0b2ad8-0440-41db-95ce-6a9b9e88b724)



