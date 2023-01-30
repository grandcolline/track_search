# Track Search

とりあえず、動くものを・・・🌝

## Documents

https://storage.googleapis.com/track-search-docs/main/index.html

## How to running application in local

1. Install rust

| name  | version |
| :---: | :------ |
| Rust  | 1.63.0  |
| Cargo | 1.63.0  |

2. Exec command

```bash
cargo run -- -e .env
```

3. Check it 🤟

- Search Page: http://localhost:8080/
- Track Psge: http://localhost:8080/track/aaaaaaa

<details>
<summary>If you use gRPC mode...</summary>

1. Rewrite `.env`

```diff
- APP_ADAPTER=html
+ APP_ADAPTER=grpc
```

2. Exec command

```bash
cargo run -- -e .env
```

3. Check it 🤟

```bash
# search tarck
$ grpcurl -plaintext \
    -proto adapter/application/grpc/proto/track.proto \
    -d '{"keyword":"aaaa"}' \
    localhost:8080 tracksearch.Track.SearchTrack

# get tarck
$ grpcurl -plaintext \
    -proto adapter/application/grpc/proto/track.proto \
    -d '{"id":"aaaaaaa"}' \
    localhost:8080 tracksearch.Track.GetTrack
```

</details>

## Command for develop

### Build

```bash
docker build -t track-search .
```

### Fomat

```bash
# rust file
cargo fmt

# proto file
find adapter/application/grpc/proto/ -name "*.proto" | xargs clang-format -i
```

## Refs

- [[Rust] モジュールのベストプラクティス](https://zenn.dev/msakuta/articles/83f9991b2aba62)
- [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649)
- [clean-architecture-buckpal-rust](https://github.com/jayy-lmao/clean-architecture-buckpal-rust)
- [11Takanori/actix-web-clean-architecture-sample](https://github.com/11Takanori/actix-web-clean-architecture-sample)
- [AkifumiSato/at-api](https://github.com/AkifumiSato/at-api)
- [jdomenechb/rust-ddd-example](https://github.com/jdomenechb/rust-ddd-example)
