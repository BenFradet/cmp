#### Server

```bash
docker run -d \
  --name=flaresolverr \
  -p 8191:8191 \
  -e LOG_LEVEL=info \
  -m 4G \
  ghcr.io/flaresolverr/flaresolverr:latest
cd server
cargo watch -x run
curl -v -X GET http://localhost:3030/api/v1/search\?q\=Selle%20italia%20slr%20boost%20endurance
```

clear flare solver sessions:

```bash
curl -L -X POST 'http://localhost:8191/v1' \
  -H 'Content-Type: application/json' \
  --data-raw '{
    "cmd": "sessions.list"
  }' |
  jq -r '.sessions[]' |
  while IFS= read -r session; do
    echo "\n"
    echo "destroying session $session"
    json='{
      "cmd": "sessions.destroy",
      "session": "'$session'"
    }'
    curl -L -X POST 'http://localhost:8191/v1' \
      -H 'Content-Type: application/json' \
      --data-raw $json
    echo "\n"
  done
```

#### Client

```bash
cd client
trunk serve --proxy-backend=http://localhost:3030/api
```

#### SSR

```bash
cd ssr
trunk -v build index.html
cargo run --features=ssr --bin ssr_server -- --dir dist
```