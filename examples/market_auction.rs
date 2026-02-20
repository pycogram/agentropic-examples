use agentropic_core::prelude::*;
use agentropic_patterns::prelude::*;

fn main() {
    println!("=== Market Auction Example ===\n");

    let mut market = Market::new("cloud_compute_market");

    println!("--- Auction: GPU Hours (English, reserve $50) ---\n");

    let mut gpu_auction =
        Auction::new(AuctionType::English, "gpu_hours").with_reserve_price(50.0);

    let trader_a = AgentId::new();
    let trader_b = AgentId::new();
    let trader_c = AgentId::new();
    let lowball = AgentId::new();

    gpu_auction.add_bid(Bid::new(trader_a, 75.0, "gpu_hours"));
    gpu_auction.add_bid(Bid::new(trader_b, 120.0, "gpu_hours"));
    gpu_auction.add_bid(Bid::new(trader_c, 95.0, "gpu_hours"));
    gpu_auction.add_bid(Bid::new(lowball, 30.0, "gpu_hours"));

    println!("Bids received: {}", gpu_auction.bids().len());
    for bid in gpu_auction.bids() {
        println!("  Agent {} bid ${:.2}", bid.bidder(), bid.amount());
    }

    match gpu_auction.winner() {
        Some(winner) => {
            println!("\nWinner: Agent {} at ${:.2}", winner.bidder(), winner.amount());
            market.allocation_mut().allocate(*winner.bidder(), "gpu_hours_x100");
            println!("Resource allocated to winner");
        }
        None => println!("\nNo winner (bids below reserve)"),
    }

    market.add_auction(gpu_auction);

    println!("\n--- Auction: Storage (Sealed Bid, no reserve) ---\n");

    let mut storage_auction = Auction::new(AuctionType::SealedBid, "storage_1tb");
    storage_auction.add_bid(Bid::new(trader_a, 25.0, "storage_1tb"));
    storage_auction.add_bid(Bid::new(trader_c, 40.0, "storage_1tb"));

    for bid in storage_auction.bids() {
        println!("  Agent {} bid ${:.2}", bid.bidder(), bid.amount());
    }

    if let Some(winner) = storage_auction.winner() {
        println!("Winner: Agent {} at ${:.2}", winner.bidder(), winner.amount());
        market.allocation_mut().allocate(*winner.bidder(), "storage_1tb");
    }

    market.add_auction(storage_auction);

    println!("\n--- Market Summary ---");
    println!("Market: {}", market.name());
    println!("Auctions completed: {}", market.auctions().len());

    let alloc = market.allocation();
    if let Some(resources) = alloc.get(&trader_b) {
        println!("Trader B won: {:?}", resources);
    }
    if let Some(resources) = alloc.get(&trader_c) {
        println!("Trader C won: {:?}", resources);
    }

    println!("\n=== Done ===");
}
