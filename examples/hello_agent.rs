use agentropic_core::prelude::*;

struct GreeterAgent {
    id: AgentId,
    name: String,
    greet_count: u32,
}

impl GreeterAgent {
    fn new(name: impl Into<String>) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
            greet_count: 0,
        }
    }
}

#[async_trait]
impl Agent for GreeterAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} is warming up...", self.name));
        Ok(())
    }

    async fn execute(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        self.greet_count += 1;
        ctx.log_info(&format!(
            "{} says: Hello, world! (greeting #{})",
            self.name, self.greet_count
        ));
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!(
            "{} shutting down after {} greetings. Goodbye!",
            self.name, self.greet_count
        ));
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("=== Hello Agent Example ===\n");

    let mut agent = GreeterAgent::new("Alice");
    let ctx = AgentContext::new(*agent.id());

    println!("Agent ID: {}", agent.id());

    let state = AgentState::Created;
    println!("State: {:?}", state);
    assert!(state.can_transition_to(AgentState::Initialized));

    agent.initialize(&ctx).await.expect("init failed");
    agent.execute(&ctx).await.expect("execute failed");
    agent.execute(&ctx).await.expect("execute failed");
    agent.execute(&ctx).await.expect("execute failed");
    agent.shutdown(&ctx).await.expect("shutdown failed");

    println!("\n=== Done ===");
}
