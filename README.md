#### API

```bash
docker run -d \      
  --name=flaresolverr \
  -p 8191:8191 \
  -e LOG_LEVEL=info \
  --restart unless-stopped \
  ghcr.io/flaresolverr/flaresolverr:latest
cargo run -p server
curl -v -X GET http://localhost:3030/api/v1/search\?q\=Selle%20italia%20slr%20boost%20endurance
```

#### Client

```bash
cd client
trunk serve
```

#### SSR

```bash
cd ssr
trunk -v build index.html
cargo run --features=ssr --bin ssr_server -- --dir dist
```