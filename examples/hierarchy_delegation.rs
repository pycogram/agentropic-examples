use agentropic_core::prelude::*;
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Hierarchy & Delegation Example ===\n");

    let mut org = Hierarchy::new("agentropic_corp");

    let exec_level = Level::new("Executive", LevelType::Strategic, 3);
    let mgmt_level = Level::new("Management", LevelType::Tactical, 2);
    let ops_level = Level::new("Operations", LevelType::Operational, 1);

    org.add_level(exec_level.clone());
    org.add_level(mgmt_level.clone());
    org.add_level(ops_level.clone());

    println!("Organization: {}", org.name());
    println!("Levels: {}\n", org.levels().len());

    assert!(exec_level.is_above(&mgmt_level));
    assert!(mgmt_level.is_above(&ops_level));

    let ceo = AgentId::new();
    let cto = AgentId::new();
    let eng_manager = AgentId::new();
    let dev_lead = AgentId::new();
    let dev_1 = AgentId::new();
    let dev_2 = AgentId::new();

    org.assign_agent(ceo, exec_level.clone());
    org.assign_agent(cto, exec_level);
    org.assign_agent(eng_manager, mgmt_level.clone());
    org.assign_agent(dev_lead, mgmt_level);
    org.assign_agent(dev_1, ops_level.clone());
    org.assign_agent(dev_2, ops_level);

    let agents = [
        ("CEO", ceo),
        ("CTO", cto),
        ("Eng Manager", eng_manager),
        ("Dev Lead", dev_lead),
        ("Developer 1", dev_1),
        ("Developer 2", dev_2),
    ];

    println!("--- Agent Assignments ---");
    for (name, id) in &agents {
        if let Some(level) = org.get_level(id) {
            println!("  {} -> {} ({:?})", name, level.name(), level.level_type());
        }
    }

    println!("\n--- Task Delegation Chain ---");

    let task1 = Delegation::new(ceo, eng_manager, "Launch v2.0 by Q3", 3);
    println!("  CEO -> Eng Manager: \"{}\" (authority: {})", task1.task(), task1.authority_level());
    org.delegate(task1);

    let task2 = Delegation::new(eng_manager, dev_lead, "Implement runtime scheduler", 2);
    println!("  Eng Manager -> Dev Lead: \"{}\" (authority: {})", task2.task(), task2.authority_level());
    org.delegate(task2);

    let task3 = Delegation::new(dev_lead, dev_1, "Write round-robin scheduler", 1);
    let task4 = Delegation::new(dev_lead, dev_2, "Write priority scheduler", 1);
    println!("  Dev Lead -> Dev 1: \"{}\" (authority: {})", task3.task(), task3.authority_level());
    println!("  Dev Lead -> Dev 2: \"{}\" (authority: {})", task4.task(), task4.authority_level());
    org.delegate(task3);
    org.delegate(task4);

    println!("\nTotal delegations: {}", org.delegations().len());

    println!("\n--- Team View ---");
    let mut team = Team::new("core_team");
    team.assign_role(
        eng_manager,
        Role::new("Engineering Manager", RoleType::Coordinator)
            .with_responsibility("Sprint planning")
            .with_responsibility("Code reviews"),
    );
    team.assign_role(
        dev_lead,
        Role::new("Technical Lead", RoleType::Leader)
            .with_responsibility("Architecture decisions")
            .with_responsibility("Mentoring"),
    );
    team.assign_role(
        dev_1,
        Role::new("Backend Developer", RoleType::Executor)
            .with_responsibility("Runtime implementation"),
    );
    team.assign_role(
        dev_2,
        Role::new("Backend Developer", RoleType::Executor)
            .with_responsibility("Scheduler implementation"),
    );
    team.set_leader(eng_manager);

    println!("Team: {}", team.name());
    println!("Members: {}", team.members().len());
    for member_id in team.members() {
        if let Some(role) = team.get_role(member_id) {
            println!("  {} ({:?}): {:?}", role.name(), role.role_type(), role.responsibilities());
        }
    }

    println!("\n=== Done ===");
}
