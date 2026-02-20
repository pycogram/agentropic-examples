use agentropic_core::prelude::*;
use agentropic_messaging::prelude::*;

struct ChatAgent {
    id: AgentId,
    name: String,
}

impl ChatAgent {
    fn new(name: impl Into<String>) -> Self {
        Self {
            id: AgentId::new(),
            name: name.into(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl Agent for ChatAgent {
    fn id(&self) -> &AgentId {
        &self.id
    }

    async fn initialize(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} connected to chat", self.name));
        Ok(())
    }

    async fn execute(&mut self, _ctx: &AgentContext) -> AgentResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self, ctx: &AgentContext) -> AgentResult<()> {
        ctx.log_info(&format!("{} left the chat", self.name));
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    println!("=== Messaging Example ===\n");

    let alice = ChatAgent::new("Alice");
    let bob = ChatAgent::new("Bob");

    println!("Created {} ({})", alice.name(), alice.id());
    println!("Created {} ({})", bob.name(), bob.id());

    let router = Router::new();
    let mut alice_inbox = router.register(*alice.id()).expect("register alice");
    let mut bob_inbox = router.register(*bob.id()).expect("register bob");

    println!("\nBoth agents registered with router");

    let msg1 = Message::new(
        *alice.id(),
        *bob.id(),
        Performative::Request,
        "Hey Bob, can you process this data?",
    );
    println!("\n--- Alice -> Bob ---");
    println!("Performative: {}", msg1.performative());
    println!("Content: {}", msg1.content());
    router.send(msg1).expect("send msg1");

    let received = bob_inbox.recv().await.expect("bob receive");
    println!("\nBob received: \"{}\"", received.content());

    let reply = MessageBuilder::new()
        .sender(*bob.id())
        .receiver(*alice.id())
        .performative(Performative::Inform)
        .content("Done! Here are the results.")
        .conversation_id("data-processing-001".to_string())
        .in_reply_to(received.id())
        .build()
        .expect("build reply");

    println!("\n--- Bob -> Alice ---");
    println!("Performative: {}", reply.performative());
    println!("Content: {}", reply.content());
    router.send(reply).expect("send reply");

    let response = alice_inbox.recv().await.expect("alice receive");
    println!("\nAlice received: \"{}\"", response.content());

    let ack = Message::new(
        *alice.id(),
        *bob.id(),
        Performative::Confirm,
        "Thanks Bob, confirmed!",
    );
    router.send(ack).expect("send ack");

    let final_msg = bob_inbox.recv().await.expect("bob receive ack");
    println!("Bob received: \"{}\" ({})", final_msg.content(), final_msg.performative());

    println!("\n=== Done ===");
}
