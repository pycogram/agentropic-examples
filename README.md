# agentropic-examples

Working examples demonstrating the [Agentropic](https://github.com/agentropic/agentropic) multi-agent framework.

## Examples

| Example | Crates Used | What It Shows |
|---------|-------------|---------------|
| `hello_agent` | core | Agent trait, lifecycle, state transitions |
| `messaging` | core, messaging | Router, FIPA performatives, message builder |
| `bdi_reasoning` | cognition | Utility-based strategy evaluation |
| `market_auction` | core, patterns | English and sealed-bid auctions, resource allocation |
| `swarm_consensus` | core, patterns | Flocking, foraging, consensus voting |
| `hierarchy_delegation` | core, patterns | Org levels, task delegation, team roles |
| `supervised_agents` | core, runtime | Health checks, circuit breaker, backoff, metrics |
| `full_system` | **all 5** | End-to-end trading system integrating every crate |

## Running
```bash
# Run a specific example
cargo run --example hello_agent

# Run all examples
for ex in hello_agent messaging bdi_reasoning market_auction swarm_consensus hierarchy_delegation supervised_agents full_system; do
  echo "=== $ex ==="
  cargo run --example $ex
  echo
done
```

## Workspace Setup

Add this crate to your root `Cargo.toml`:
```toml
[workspace]
members = [
    "agentropic-core",
    "agentropic-messaging",
    "agentropic-cognition",
    "agentropic-patterns",
    "agentropic-runtime",
    "agentropic",
    "agentropic-examples",
]
```

## License

MIT OR Apache-2.0
