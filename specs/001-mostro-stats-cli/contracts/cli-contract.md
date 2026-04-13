# CLI Contract: mstats

**Date**: 2026-04-13
**Feature**: 001-mostro-stats-cli

## Command Schema

```
mstats [OPTIONS]
```

## Arguments

No positional arguments. All configuration is via flags.

## Flags and Options

### Output Format

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--json` | bool | `false` | Output in JSON format instead of human-readable text |

### Filters

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--from` | ISO 8601 date or Unix timestamp | None (no lower bound) | Start of date range (inclusive). Applies to kind 8383 `created_at` timestamp. |
| `--to` | ISO 8601 date or Unix timestamp | None (no upper bound) | End of date range (inclusive). Applies to kind 8383 `created_at` timestamp. |
| `--node` | hex pubkey string | None (all nodes) | Filter to a specific Mostro node by its pubkey |
| `--currency` | currency code (e.g., USD, EUR) | None (all currencies) | Filter to a specific fiat currency |
| `--side` | `buy` \| `sell` | None (both sides) | Filter by order side |

### Connection

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--relay` | WebSocket URL | `wss://relay.mostro.network` | Relay URL (override default for testing) |

## Exit Codes

| Code | Meaning |
|------|--------|
| `0` | Success (including "no data" state) |
| `1` | General error (relay unreachable, query failed, invalid arguments) |
| `2` | Invalid arguments (unknown flags, malformed values) |

## Output Contract: Human-Readable (default)

Printed to stdout. Structure:

```
=== Mostro Network Statistics ===

Global:
  Orders:              1,234
  Total Dev Fees:      45,678 sats
  Total Volume:        12,345,678 sats
  Avg Order Size:      10,006 sats
  Fiat Volume:
    USD:               50,000
    EUR:               30,000
  By Side:
    Buy:               7,000,000 sats
    Sell:              5,345,678 sats

--- Per-Node Statistics ---

Node: abc123...def0123  (y: ABC123)
  Orders:              456
  Total Dev Fees:      12,345 sats
  Total Volume:        4,567,890 sats
  Avg Order Size:      10,017 sats
  Fiat Volume:
    USD:               25,000
    EUR:               15,000
  By Side:
    Buy:               2,500,000 sats
    Sell:              2,067,890 sats

[... additional nodes ...]

Summary: Processed 1,234 kind 8383 events, joined with 1,180 kind 38383 events.
Unjoined: 54 events excluded from aggregates.
```

## Output Contract: JSON

Printed to stdout as a single valid JSON object. Structure:

```json
{
  "global": {
    "order_count": 1234,
    "total_fees_sats": 45678,
    "total_volume_sats": 12345678,
    "avg_order_size_sats": 10006.0,
    "fiat_volume_by_currency": {
      "USD": 50000.0,
      "EUR": 30000.0
    },
    "volume_by_side": {
      "buy": 7000000,
      "sell": 5345678
    },
    "source_event_ids": ["evt_id_1", "evt_id_2", "..."]
  },
  "nodes": [
    {
      "pubkey": "abc123...def0123",
      "y_tag_value": "ABC123",
      "order_count": 456,
      "total_fees_sats": 12345,
      "total_volume_sats": 4567890,
      "avg_order_size_sats": 10017.0,
      "fiat_volume_by_currency": {
        "USD": 25000.0,
        "EUR": 15000.0
      },
      "volume_by_side": {
        "buy": 2500000,
        "sell": 2067890
      },
      "source_event_ids": ["evt_id_1", "..."]
    }
  ],
  "unjoined_count": 54,
  "unjoined": [
    {
      "event_id": "unmatched_8383_id",
      "order_id": "order_xyz",
      "reason": "OrderNotFound"
    }
  ],
  "errors": [],
  "filter_summary": "No filters applied"
}
```

## Error Output (stderr)

Error messages are printed to stderr in human-readable form regardless of `--json` flag:

```
Error: Could not connect to relay wss://relay.mostro.network: connection timed out after 30s
```

```
Error: Invalid date format for --from: "2026-13-01" is not a valid date
```

## Validation Rules

- `--from` and `--to` accept ISO 8601 date strings (e.g., `2026-01-01`) or raw Unix timestamps. If the value is a date string without time, it is interpreted as midnight UTC of that date (`--from` inclusive start, `--to` inclusive end).
- `--node` must be a valid hex-encoded 32-byte pubkey (64 hex characters). Invalid values produce exit code 2.
- `--currency` accepts any string; the tool does not validate against a currency registry (unknown currencies simply match nothing).
- `--side` must be exactly `buy` or `sell` (case-insensitive). Invalid values produce exit code 2.
- `--relay` must be a valid WebSocket URL (`ws://` or `wss://` scheme). Invalid URLs produce exit code 2.
