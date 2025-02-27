# commonware-aggregation

Aggregate signatures from multiple contributors over the BN254 curve.

# Usage (3 of 3 Threshold)

_To run this example, you must first install [Rust](https://www.rust-lang.org/tools/install) and [protoc](https://grpc.io/docs/protoc-installation)._


## Holesky (since its broken???)
```bash
anvil -f https://1rpc.io/holesky --block-time 12
```

## For the rest of the cli's start with this
```bash 
cd examples/aggregation
```

## Orchestrator
```bash
cargo run --release -- --key-file keys/orchestrator.yaml --port 3000 --participants keys/orchestrator.yaml,keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml --contributors keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml
```

## Contributor 1
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --key-file keys/contributor1.yaml --port 3001 --participants keys/orchestrator.yaml,keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml --orchestrator keys/orchestrator.yaml --contributors keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml
```

## Contributor 2
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --key-file keys/contributor2.yaml --port 3002 --participants keys/orchestrator.yaml,keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml --orchestrator keys/orchestrator.yaml --contributors keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml
```

## Contributor 3
```bash
cargo run --release -- --bootstrappers 69@127.0.0.1:3000 --key-file keys/contributor3.yaml --port 3003 --participants keys/orchestrator.yaml,keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml --orchestrator keys/orchestrator.yaml --contributors keys/contributor1.yaml,keys/contributor2.yaml,keys/contributor3.yaml
```
