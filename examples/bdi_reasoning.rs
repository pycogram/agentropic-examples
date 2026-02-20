use agentropic_cognition::UtilityFunction;

fn main() {
    println!("=== Utility-Based Decision Making ===\n");

    let aggressive = UtilityFunction::new("aggressive", |state: &[String]| {
        if state.iter().any(|s| s.contains("high_reward")) {
            0.9
        } else if state.iter().any(|s| s.contains("medium_reward")) {
            0.5
        } else {
            0.1
        }
    });

    let conservative = UtilityFunction::new("conservative", |state: &[String]| {
        if state.iter().any(|s| s.contains("low_risk")) {
            0.85
        } else if state.iter().any(|s| s.contains("medium_risk")) {
            0.4
        } else {
            0.1
        }
    });

    let balanced = UtilityFunction::new("balanced", |state: &[String]| {
        let mut score: f64 = 0.5;
        for s in state {
            if s.contains("reward") { score += 0.2; }
            if s.contains("risk") { score -= 0.15; }
        }
        score.clamp(0.0, 1.0)
    });

    let fallback = UtilityFunction::simple("fallback");

    println!("--- Scenario 1: High-Risk, High-Reward ---");
    let scenario1 = vec![
        "high_reward".to_string(),
        "high_risk".to_string(),
        "volatile_market".to_string(),
    ];

    let strategies = [&aggressive, &conservative, &balanced, &fallback];
    for strategy in &strategies {
        println!("  {} strategy: {:.2}", strategy.name(), strategy.evaluate(&scenario1));
    }

    let best = strategies.iter().max_by(|a, b| {
        a.evaluate(&scenario1).partial_cmp(&b.evaluate(&scenario1)).unwrap()
    }).unwrap();
    println!("  -> Best strategy: {}\n", best.name());

    println!("--- Scenario 2: Low-Risk, Medium-Reward ---");
    let scenario2 = vec![
        "medium_reward".to_string(),
        "low_risk".to_string(),
        "stable_market".to_string(),
    ];

    for strategy in &strategies {
        println!("  {} strategy: {:.2}", strategy.name(), strategy.evaluate(&scenario2));
    }

    let best = strategies.iter().max_by(|a, b| {
        a.evaluate(&scenario2).partial_cmp(&b.evaluate(&scenario2)).unwrap()
    }).unwrap();
    println!("  -> Best strategy: {}\n", best.name());

    println!("--- Scenario 3: Unknown Conditions ---");
    let scenario3 = vec!["unknown".to_string()];
    for strategy in &strategies {
        println!("  {} strategy: {:.2}", strategy.name(), strategy.evaluate(&scenario3));
    }

    println!("\n=== Done ===");
}
