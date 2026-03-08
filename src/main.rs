use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct TxObject {
    id: u64,
    sender: String,
    receiver: String,
    amount: u64,
}

#[derive(Debug)]
enum Decision {
    Approve,
    Reject(String),
    Challenge(String),
}

struct DecisionEngine { daily_limit: u64 }

impl DecisionEngine {
    fn new() -> Self { DecisionEngine { daily_limit: 1_000_000 } }
    fn evaluate(&self, tx: &TxObject) -> Decision {
        if tx.amount > self.daily_limit {
            return Decision::Reject(format!("Exceeds limit: {}", tx.amount));
        }
        if tx.amount > self.daily_limit / 2 {
            return Decision::Challenge("Large amount".to_string());
        }
        Decision::Approve
    }
}

struct LedgerEntry { tx_id: u64, decision: String, hash: u64, prev_hash: u64 }

struct SovereignLedger { entries: Vec<LedgerEntry> }

impl SovereignLedger {
    fn new() -> Self { SovereignLedger { entries: vec![] } }
    fn record(&mut self, tx: &TxObject, decision: &str) {
        let prev_hash = self.entries.last().map(|e| e.hash).unwrap_or(0);
        let mut hasher = DefaultHasher::new();
        tx.id.hash(&mut hasher);
        decision.hash(&mut hasher);
        prev_hash.hash(&mut hasher);
        let hash = hasher.finish();
        self.entries.push(LedgerEntry { tx_id: tx.id, decision: decision.to_string(), hash, prev_hash });
        println!("  Hash: {:x} | Prev: {:x}", hash, prev_hash);
    }
}

fn main() {
    println!("Sovereign Wallet MVP\n");
    let engine = DecisionEngine::new();
    let mut ledger = SovereignLedger::new();
    let txs = vec![
        TxObject { id: 1, sender: "Alice".to_string(), receiver: "Bob".to_string(), amount: 50_000 },
        TxObject { id: 2, sender: "Alice".to_string(), receiver: "Eve".to_string(), amount: 2_000_000 },
        TxObject { id: 3, sender: "Bob".to_string(), receiver: "Carol".to_string(), amount: 600_000 },
    ];
    for tx in &txs {
        println!("TX#{}: {} -> {} | {}", tx.id, tx.sender, tx.receiver, tx.amount);
        let decision = engine.evaluate(tx);
        let label = match &decision {
            Decision::Approve => "APPROVE",
            Decision::Reject(_) => "REJECT",
            Decision::Challenge(_) => "CHALLENGE",
        };
        println!("Decision: {:?}", decision);
        ledger.record(tx, label);
        println!();
    }
    println!("Total: {} entries", ledger.entries.len());
  }
