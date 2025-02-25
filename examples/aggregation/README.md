# commonware-aggregation

Aggregate signatures from multiple contributors over the BN254 curve.

# Usage (3 of 3 Threshold)

_To run this example, you must first install [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation)._

## Orchestrator
```bash
cargo run --release -- --me 69@3000 --participants 69,11,22,33 --contributors 11,22,33
```

## Contributor 1
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --me 11@3001 --participants 69,11,22,33  --orchestrator 69 --contributors 11,22,33
```

## Contributor 2
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --me 22@3002 --participants 69,11,22,33  --orchestrator 69 --contributors 11,22,33
```

## Contributor 3
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --me 33@3003 --participants 69,11,22,33  --orchestrator 69 --contributors 11,22,33
```
