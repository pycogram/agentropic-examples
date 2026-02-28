# Agentropic Examples

Working examples demonstrating the [Agentropic](https://github.com/agentropic) multi-agent framework.

## Examples

| Example | What it shows |
|---------|--------------|
| `hello_agent` | Spawn an agent with Runtime, watch it tick |
| `messaging` | Two agents send messages through the Router |
| `agents_talking` | Three agents: asker, responder, observer |
| `supervised_agents` | Agent crashes 3 times, Supervisor restarts it |
| `cognitive_agent` | Reasons from BeliefBase, falls back to LLM |
| `full_system` | All features together in one demo |

### Patterns

| Pattern | What it shows |
|---------|--------------|
| `team_pattern` | Leader assigns tasks, Executors work, Coordinator tracks |
| `hierarchy_pattern` | Commander -> Captain -> Soldiers, orders down, reports up |
| `swarm_pattern` | Decentralized consensus voting, no leader |
| `coalition_pattern` | Temporary alliance for a mission, then disband |
| `market_pattern` | Sealed-bid auction, highest bidder wins |
| `federation_pattern` | Weighted voting on proposals |
| `holarchy_pattern` | Nested autonomous units delegate down |
| `blackboard_pattern` | Shared knowledge space for collaborative problem solving |

## Run
```bash
cargo run --example hello_agent
cargo run --example messaging
cargo run --example agents_talking
cargo run --example supervised_agents
cargo run --example cognitive_agent
cargo run --example full_system

# Patterns
cargo run --example team_pattern
cargo run --example hierarchy_pattern
cargo run --example swarm_pattern
cargo run --example coalition_pattern
cargo run --example market_pattern
cargo run --example federation_pattern
cargo run --example holarchy_pattern
cargo run --example blackboard_pattern
```

## Cognitive Agent Setup

The `cognitive_agent` and `full_system` examples need data files:

- `data/beliefs.json` — Knowledge base (included)
- `data/config.json` — LLM provider config (included, defaults to Ollama)

To use Ollama locally:
```bash
curl -fsSL https://ollama.ai/install.sh | sh
ollama pull mistral
```

To switch to Claude API, edit `data/config.json`:
```json
{
  "llm_provider": "claude",
  "llm_model": "claude-sonnet-4-20250514",
  "api_key": "your-key-here"
}
```

## License

MIT OR Apache-2.0
