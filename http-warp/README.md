
```shell
curl http://127.0.0.1:3030/health
```

```shell
curl --location --request POST 'http://127.0.0.1:3030/set' \
--header 'Content-Type: application/json' \
--data-raw '{
    "key": "name",
    "value": "coolbeevip"
}'
```

```shell
curl http://127.0.0.1:3030/get/name
```