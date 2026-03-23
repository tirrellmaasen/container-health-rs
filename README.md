# container-health-rs

A small CLI tool to check the health of running containers. Written in Rust for minimal resource usage.

## Why

I needed something lighter than a full monitoring stack for dev environments. This polls container health endpoints and exits non-zero on failure — useful in CI pipelines and Argo workflows.

## Build

```bash
cargo build --release
```

## Usage

```bash
# Check a single endpoint
container-health-rs --url http://localhost:8080/healthz --timeout 5

# Check multiple from a config file
container-health-rs --config checks.yaml
```

## Status

Work in progress. Basic HTTP checks work. Adding TCP and gRPC support next.

## License

MIT
