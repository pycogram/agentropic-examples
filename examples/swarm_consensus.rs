use agentropic_core::prelude::*;
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Swarm Consensus Example ===\n");

    let mut swarm = Swarm::new("reconnaissance_drones");

    let mut drone_ids = Vec::new();
    for i in 0..10 {
        let id = AgentId::new();
        swarm.add_member(id);
        drone_ids.push((i, id));
    }

    println!("Swarm: {}", swarm.name());
    println!("Drones deployed: {}\n", swarm.size());

    let flocking = Flocking::new()
        .with_separation(2.5)
        .with_alignment(1.0)
        .with_cohesion(0.8);

    println!("--- Flocking Configuration ---");
    println!("  Separation: {}", flocking.separation_weight());
    println!("  Alignment:  {}", flocking.alignment_weight());
    println!("  Cohesion:   {}", flocking.cohesion_weight());

    let foraging = Foraging::new()
        .with_pheromone_strength(1.5)
        .with_evaporation_rate(0.15)
        .with_exploration_rate(0.3);

    println!("\n--- Foraging Configuration ---");
    println!("  Pheromone strength: {}", foraging.pheromone_strength());
    println!("  Evaporation rate:   {}", foraging.evaporation_rate());
    println!("  Exploration rate:   {}", foraging.exploration_rate());

    let behavior = Behavior::new(BehaviorType::Exploration)
        .with_parameter("search_radius", 500.0)
        .with_parameter("report_interval", 10.0);
    swarm.set_behavior(behavior);

    println!(
        "\nActive behavior: {:?}",
        swarm.behavior().unwrap().behavior_type()
    );

    println!("\n--- Consensus Vote: Select Target ---");
    let mut consensus = Consensus::new(0.6);

    for (i, id) in &drone_ids {
        let vote = match i {
            0..=5 => "target_north",
            6..=7 => "target_east",
            _ => "target_south",
        };
        consensus.vote(*id, vote);
        println!("  Drone {} votes: {}", i, vote);
    }

    println!("\nConsensus reached: {}", consensus.is_reached());
    if let Some(winner) = consensus.winner() {
        println!("Decision: {} (6/10 = 60% threshold met)", winner);
    }

    println!("\n--- Round 2: Stricter Consensus (80% required) ---");
    let mut strict_consensus = Consensus::new(0.8);
    for (_, id) in &drone_ids {
        strict_consensus.vote(*id, "target_north");
    }

    println!("All drones vote north");
    println!("Consensus reached: {}", strict_consensus.is_reached());
    println!("Decision: {:?}", strict_consensus.winner());

    println!("\n=== Done ===");
}
