use agentropic_core::prelude::*;
use agentropic_runtime::prelude::*;
use std::time::Duration;

fn main() {
    println!("=== Supervised Agents Example ===\n");

    let worker_a = AgentId::new();
    let worker_b = AgentId::new();
    let worker_c = AgentId::new();

    let mut supervisor = Supervisor::new("production_supervisor");

    let always_restart = RestartPolicy::new(RestartStrategy::Always)
        .with_max_retries(10)
        .with_backoff_seconds(2);
    let on_failure = RestartPolicy::new(RestartStrategy::OnFailure)
        .with_max_retries(3)
        .with_backoff_seconds(5);
    let never_restart = RestartPolicy::new(RestartStrategy::Never);

    supervisor.supervise(worker_a, always_restart);
    supervisor.supervise(worker_b, on_failure);
    supervisor.supervise(worker_c, never_restart);

    println!("--- Supervisor: {} ---", supervisor.name());
    println!("Supervised agents: {}\n", supervisor.supervised_count());

    let workers = [
        ("Worker A (always restart)", worker_a),
        ("Worker B (on failure)", worker_b),
        ("Worker C (never restart)", worker_c),
    ];

    for (name, id) in &workers {
        let policy = supervisor.get_policy(id).unwrap();
        println!(
            "  {} -> {:?}, max_retries: {:?}, backoff: {}s",
            name, policy.strategy(), policy.max_retries(), policy.backoff_seconds()
        );
    }

    println!("\n--- Health Check Simulation ---");

    if let Some(hc) = supervisor.get_health_check_mut(&worker_a) {
        hc.record_healthy();
        println!("  Worker A: {:?} (failures: {})", hc.status(), hc.failures());
    }

    if let Some(hc) = supervisor.get_health_check_mut(&worker_b) {
        hc.record_unhealthy();
        hc.record_unhealthy();
        hc.record_healthy();
        println!("  Worker B: {:?} (failures: {})", hc.status(), hc.failures());
    }

    if let Some(hc) = supervisor.get_health_check_mut(&worker_c) {
        hc.record_unhealthy();
        hc.record_unhealthy();
        hc.record_unhealthy();
        println!("  Worker C: {:?} (failures: {})", hc.status(), hc.failures());
    }

    println!("\n--- Circuit Breaker ---");
    let mut breaker = CircuitBreaker::new(3, Duration::from_secs(30));

    println!("Initial state: {:?}", breaker.state());

    breaker.record_failure();
    breaker.record_failure();
    println!("After 2 failures: {:?} (allowed: {})", breaker.state(), breaker.is_allowed());

    breaker.record_failure();
    println!("After 3 failures: {:?} (allowed: {})", breaker.state(), breaker.is_allowed());

    breaker.record_success();
    println!("After success:    {:?} (allowed: {})", breaker.state(), breaker.is_allowed());

    println!("\n--- Exponential Backoff ---");
    let mut backoff = ExponentialBackoff::new(
        Duration::from_millis(100),
        Duration::from_secs(10),
    );

    for i in 1..=6 {
        let delay = backoff.next_delay();
        println!("  Retry {}: wait {:?}", i, delay);
    }

    println!("  Resetting...");
    backoff.reset();
    println!("  After reset: wait {:?}", backoff.next_delay());

    println!("\n--- Metrics Collection ---");
    let mut registry = MetricsRegistry::new();

    let mut agent_metrics = Collector::new();
    agent_metrics.record(
        Metric::new("messages_sent", MetricType::Counter, 142.0)
            .with_label("agent", "worker_a"),
    );
    agent_metrics.record(
        Metric::new("cpu_usage", MetricType::Gauge, 45.2)
            .with_label("agent", "worker_a"),
    );
    agent_metrics.record(
        Metric::new("response_time_ms", MetricType::Histogram, 12.5)
            .with_label("agent", "worker_b"),
    );

    registry.register("agent_metrics", agent_metrics);

    let exporter = MetricsExporter::new(registry);
    match exporter.export_json() {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Export failed: {}", e),
    }

    println!("\n--- Task Queue ---");
    let mut queue = TaskQueue::new();
    queue.push(Task::new(worker_a, 3));
    queue.push(Task::new(worker_b, 1));
    queue.push(Task::new(worker_c, 2));

    println!("Tasks queued: {}", queue.len());
    while let Some(task) = queue.pop() {
        println!("  Processing agent {} (priority {})", task.agent_id(), task.priority());
    }
    println!("Queue empty: {}", queue.is_empty());

    println!("\n=== Done ===");
}
