#### API

`cargo run -p server`
`curl -v -X GET http://localhost:3030/api/v1/search\?q\=Selle%20italia%20slr%20boost%20endurance`

#### CSR

`cd ssr`
`trunk -v build index.html`

#### SSR

`cd ssr`
`cargo run --features=ssr --bin ssr_server -- --dir dist`