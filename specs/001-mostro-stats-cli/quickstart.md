# Quickstart: mstats

**Date**: 2026-04-13
**Feature**: 001-mostro-stats-cli

## Prerequisites

- Rust toolchain (latest stable) with `cargo` and `rustfmt`
- `cargo-clippy` installed (via `rustup component add clippy`)
- Network access to `wss://relay.mostro.network`

## Build

```bash
cargo build --release
```

The binary is produced at `target/release/mstats`.

## Run

### Basic usage (human-readable report)

```bash
./target/release/mstats
```

Connects to `wss://relay.mostro.network`, fetches all kind 8383 events, joins with kind 38383 events, and prints global + per-node statistics.

### JSON output

```bash
./target/release/mstats --json
```

### Filtering

```bash
# Date range
./target/release/mstats --from 2026-01-01 --to 2026-03-31

# Specific node
./target/release/mstats --node <hex-pubkey>

# Specific currency
./target/release/mstats --currency USD

# Buy orders only
./target/release/mstats --side buy

# Combined filters with JSON output
./target/release/mstats --json --from 2026-01-01 --currency USD --side sell
```

### Custom relay (for testing)

```bash
./target/release/mstats --relay wss://test-relay.example.com
```

## Run Tests

```bash
# Unit tests (offline, always run)
cargo test

# Integration tests (requires relay connection)
cargo test --features integration-tests
```

## Code Quality

```bash
# Format check
cargo fmt --check

# Lint
cargo clippy
```

## Expected Project Structure After Implementation

```
.
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── relay.rs
│   ├── event_parser.rs
│   ├── joiner.rs
│   ├── aggregator.rs
│   ├── models.rs
│   ├── filters.rs
│   └── output.rs
└── tests/
    ├── unit/
    └── integration/
```

## Development Loop

```bash
# After making changes
cargo fmt && cargo clippy && cargo test

# Quick run during development
cargo run -- --json --from 2026-03-01 --to 2026-03-31
```
