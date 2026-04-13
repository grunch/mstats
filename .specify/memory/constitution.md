<!--
  SYNC IMPACT REPORT
  ====================
  Version change: 1.0.0 → 1.1.0
  Bump rationale: MINOR — new principle added (Canonical Nostr Implementation).

  Modified principles: (renumbered — existing principles unchanged)
    - III. Small and Maintainable CLI-First Design (III → III, no rename)
    - Renumbering: previous III→III, IV→IV, V→V, VI→VII, new principle added as VI

  Added sections:
    - VI. Canonical Nostr Implementation

  Removed sections: (none)

  Templates requiring updates:
    - .specify/templates/plan-template.md       ✅ No changes needed
    - .specify/templates/spec-template.md        ✅ No changes needed
    - .specify/templates/tasks-template.md       ✅ No changes needed
    - .qwen/commands/speckit.*.md                ✅ No changes needed
    - README.md                                  ⚠ Pending — principles not referenced yet

  Follow-up TODOs: (none)
-->

# mstats Constitution

## Core Principles

### I. Correctness Over Cleverness
All reported statistics MUST be directly derived from Nostr events and reproducible from source data. The tool MUST NOT emit inferred, estimated, or speculative metrics. Every number in the output must be traceable to specific event IDs.

**Rationale**: Statistics that cannot be audited or reproduced erode trust. Correctness is the primary non-negotiable quality attribute.

### II. Single Canonical Data Source (v1)
The first version MUST use only one relay as the canonical source of events: `wss://relay.mostro.network`. Multi-relay support, relay discovery, and cross-relay reconciliation are explicitly out of scope for v1 unless introduced by a later specification.

**Rationale**: Limiting to a single relay in v1 keeps the implementation simple, testable, and free from the complexity of relay disagreement, event deduplication, and trust scoring.

### III. Small and Maintainable CLI-First Design
The first version MUST remain small, dependency-light, and easy to audit. The project MUST follow a clear Rust CLI architecture. Dashboards, databases, and unnecessary abstraction layers are out of scope for v1.

**Rationale**: A small, readable codebase is auditable and maintainable. Adding UI frameworks or persistence layers prematurely increases complexity and slows delivery.

### IV. Transparent Handling of Incomplete Data
Missing tags, missing linked events, malformed events, and relay failures MUST be handled explicitly. The tool MUST NEVER silently fabricate statistics from incomplete joins. When data is missing or events cannot be joined, the output MUST clearly indicate the gap.

**Rationale**: Silent data fabrication is worse than no data. Users must know when statistics are incomplete so they can reason about confidence.

### V. Human and Machine Readable Output
The tool MUST provide a clear human-readable report by default and MUST support machine-readable JSON output for scripting and further analysis.

**Rationale**: Default readability serves interactive users; JSON output enables composition with other tools, CI pipelines, and dashboards built downstream.

### VI. Canonical Nostr Implementation
The Rust implementation MUST use `nostr-sdk` version `0.44.1` as the canonical Nostr dependency in v1. No alternative Nostr libraries or custom protocol implementations are permitted for v1.

**Rationale**: A single, pinned Nostr SDK eliminates protocol ambiguity, ensures consistent event handling, and makes the implementation auditable against a known reference. Pinning the exact version prevents drift and simplifies reproducible builds.

### VII. Explicit Scope for v1
The first version is strictly limited to:
- Reading kind 8383 events
- Extracting `order-id`
- Fetching related kind 38383 events
- Joining both event types
- Aggregating per-node statistics

The first version MUST NOT include:
- Web UI
- Charts
- Persistent database
- Advanced scoring
- Multi-relay support
- Real-time monitoring

**Rationale**: A tight v1 scope ensures delivery of a correct, auditable tool before expanding features. Scope creep is the primary risk to correctness and timeliness.

## Scope Constraints

v1 boundaries are enforced by principle VII. Any feature outside the listed capabilities requires a new specification and a constitution amendment documenting the scope change. The v1 constraint exists to deliver a correct, minimal tool before investing in complexity.

## Development Workflow

All features MUST follow the specify workflow:
1. Feature specification written first (spec.md)
2. Implementation plan with research and data model (plan.md)
3. Tasks generated from spec and plan (tasks.md)
4. Implementation follows tasks, committing incrementally

Code MUST be written in Rust, formatted with `rustfmt`, and pass `cargo clippy` without warnings before merge. Tests are encouraged but not mandatory for v1 — correctness of output is verified through manual audit against relay data.

## Governance

This constitution supersedes all other development practices. All PRs and code reviews MUST verify compliance with these principles before merge. Amendments require:
1. A written proposal describing the change and rationale
2. User or stakeholder agreement
3. An update to this file with a version bump
4. A migration plan if the change affects existing behavior

**Versioning policy**: Semantic versioning (MAJOR.MINOR.PATCH). MAJOR for backward-incompatible principle changes, MINOR for additions, PATCH for clarifications.

**Compliance review**: Every feature specification MUST include a Constitution Check section validating that the design respects the principles listed here.

**Version**: 1.1.0 | **Ratified**: 2026-04-13 | **Last Amended**: 2026-04-13
