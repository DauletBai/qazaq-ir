pub mod api;
pub mod execution_engine;
pub mod mempool;
pub mod state;

use api::{AppState, create_router};
use colored::*;
use execution_engine::ExecutionEngine;
use mempool::TransactionPool;
use state::State;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        "=== Orda Node (Post-Quantum API Gateway) ===".bold().cyan()
    );

    // Initialize Global State
    let state = Arc::new(Mutex::new(State::new()));
    let mempool = Arc::new(Mutex::new(TransactionPool::new()));

    // Spawn the Execution Engine on a background thread
    let execution_mempool = mempool.clone();
    let execution_state = state.clone();
    tokio::spawn(async move {
        ExecutionEngine::run_loop(execution_mempool, execution_state).await;
    });

    // Package the state for the Router
    let app_state = AppState { mempool, state };

    let app = create_router(app_state);

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!(
        "{} TCP Listener Started. Awaiting P2P Intents on {}...",
        "»".yellow(),
        addr.bold().green()
    );

    axum::serve(listener, app).await.unwrap();
}
