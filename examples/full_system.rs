use agentropic_core::prelude::*;
use agentropic_cognition::UtilityFunction;
use agentropic_messaging::prelude::*;
use agentropic_patterns::prelude::*;
use agentropic_patterns::federation::PolicyType as FedPolicyType;
use agentropic_runtime::prelude::*;

struct TradingAgent {
    id: AgentId,
    name: String,
    strategy: UtilityFunction,
}

impl TradingAgent {
    fn new(name: impl Into<String>, strategy: UtilityFunction) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            strategy,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn evaluate(&self, market_state: &[String]) -> f64 {
        self.strategy.evaluate(market_state)
    }
}

#[async_trait]
impl Agent for TradingAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} online, strategy: {}", self.name, self.strategy.name()));
        Ok(())
    }

    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} executing trade cycle", self.name));
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} closing positions", self.name));
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("=== AGENTROPIC Full System Demo ===\n");

    // 1. Create agents (core + cognition)
    println!("--- 1. Agent Creation ---\n");

    let mut alpha = TradingAgent::new(
        "Alpha",
        UtilityFunction::new("momentum", |state: &[String]| {
            if state.iter().any(|s| s.contains("uptrend")) { 0.9 } else { 0.3 }
        }),
    );
    let mut beta = TradingAgent::new(
        "Beta",
        UtilityFunction::new("mean_reversion", |state: &[String]| {
            if state.iter().any(|s| s.contains("oversold")) { 0.85 } else { 0.4 }
        }),
    );
    let mut gamma = TradingAgent::new("Gamma", UtilityFunction::simple("passive"));

    for agent in [&mut alpha, &mut beta, &mut gamma] {
        let ctx = AgentContext::new(*agent.id());
        agent.initialize(&ctx).await.unwrap();
    }

    // 2. Messaging
    println!("\n--- 2. Messaging Layer ---\n");

    let router = Router::new();
    let mut alpha_rx = router.register(*alpha.id()).unwrap();
    let mut beta_rx = router.register(*beta.id()).unwrap();
    let _gamma_rx = router.register(*gamma.id()).unwrap();

    let proposal = MessageBuilder::new()
        .sender(*alpha.id())
        .receiver(*beta.id())
        .performative(Performative::Propose)
        .content("Buy ETH/USD at 3200, size 10")
        .conversation_id("trade-001".to_string())
        .build()
        .unwrap();

    println!("Alpha -> Beta: [{}] {}", proposal.performative(), proposal.content());
    router.send(proposal).unwrap();

    let received = beta_rx.recv().await.unwrap();
    println!("Beta received: \"{}\"", received.content());

    let acceptance = Message::new(*beta.id(), *alpha.id(), Performative::Accept, "Accepted. Executing buy order.");
    router.send(acceptance).unwrap();

    let response = alpha_rx.recv().await.unwrap();
    println!("Alpha received: \"{}\" ({})", response.content(), response.performative());

    // 3. Strategy evaluation (cognition)
    println!("\n--- 3. Strategy Evaluation ---\n");

    let market_state = vec!["uptrend".to_string(), "high_volume".to_string(), "oversold".to_string()];

    let agents: Vec<&TradingAgent> = vec![&alpha, &beta, &gamma];
    for agent in &agents {
        println!("  {} ({}): {:.2}", agent.name(), agent.strategy.name(), agent.evaluate(&market_state));
    }

    let best = agents.iter().max_by(|a, b| {
        a.evaluate(&market_state).partial_cmp(&b.evaluate(&market_state)).unwrap()
    }).unwrap();
    println!("  -> Lead trader: {}", best.name());

    // 4. Coalition (patterns)
    println!("\n--- 4. Coalition Formation ---\n");

    let mut coalition = Coalition::new("trading_syndicate");
    coalition.add_member(*alpha.id());
    coalition.add_member(*beta.id());
    coalition.add_member(*gamma.id());

    let strategy = Strategy::new(StrategyType::MaximizeUtility)
        .with_parameter("risk_tolerance", 0.7)
        .with_parameter("position_limit", 100.0);
    coalition.set_strategy(strategy);
    coalition.set_value(25000.0);

    println!("Coalition: {}", coalition.name());
    println!("Members: {}", coalition.size());
    println!("Value: ${:.2}", coalition.value());

    // 5. Federation (patterns)
    println!("\n--- 5. Federation Governance ---\n");

    let mut federation = Federation::new("trading_federation");
    federation.add_member(*alpha.id());
    federation.add_member(*beta.id());
    federation.add_member(*gamma.id());
    federation.set_weight(*alpha.id(), 2.0);
    federation.set_weight(*beta.id(), 1.5);
    federation.set_weight(*gamma.id(), 1.0);

    let policy = Policy::new("trade_approval", FedPolicyType::WeightedVote)
        .with_threshold(0.6)
        .with_rule("All trades > $10k require approval")
        .with_rule("Maximum 3 open positions per agent");
    federation.add_policy(policy);

    println!("Federation: {} ({} members)", federation.name(), federation.size());
    if let Some(p) = federation.get_policy("trade_approval") {
        println!("Policy: {} ({:?}, threshold: {:.0}%)", p.name(), p.policy_type(), p.threshold() * 100.0);
        for rule in p.rules() {
            println!("  - {}", rule);
        }
    }

    // 6. Supervision (runtime)
    println!("\n--- 6. Runtime Supervision ---\n");

    let mut supervisor = Supervisor::new("trading_supervisor");
    supervisor.supervise(*alpha.id(), RestartPolicy::new(RestartStrategy::Always).with_max_retries(5));
    supervisor.supervise(*beta.id(), RestartPolicy::new(RestartStrategy::OnFailure).with_max_retries(3));
    supervisor.supervise(*gamma.id(), RestartPolicy::new(RestartStrategy::Never));

    println!("Supervisor: {} ({} agents)", supervisor.name(), supervisor.supervised_count());

    for agent in &agents {
        if let Some(hc) = supervisor.get_health_check_mut(agent.id()) {
            hc.record_healthy();
        }
    }
    if let Some(hc) = supervisor.get_health_check_mut(beta.id()) {
        hc.record_unhealthy();
    }

    for agent in &agents {
        if let Some(hc) = supervisor.get_health_check(agent.id()) {
            let status = if hc.is_healthy() { "OK" } else { "!!" };
            println!("  [{}] {} ({:?})", status, agent.name(), hc.status());
        }
    }

    // 7. Metrics
    println!("\n--- 7. Metrics ---\n");

    let mut registry = MetricsRegistry::new();
    let mut collector = Collector::new();
    collector.record(Metric::new("trades_executed", MetricType::Counter, 47.0).with_label("coalition", "syndicate"));
    collector.record(Metric::new("pnl_usd", MetricType::Gauge, 1250.50).with_label("coalition", "syndicate"));
    collector.record(Metric::new("latency_ms", MetricType::Histogram, 3.2).with_label("agent", "alpha"));
    registry.register("trading", collector);

    let exporter = MetricsExporter::new(registry);
    if let Ok(json) = exporter.export_json() {
        println!("{}", json);
    }

    // Shutdown
    println!("\n--- Shutdown ---\n");
    for agent in [&mut alpha, &mut beta, &mut gamma] {
        let ctx = AgentContext::new(*agent.id());
        agent.shutdown(&ctx).await.unwrap();
    }

    println!("\n=== All systems nominal ===");
}
