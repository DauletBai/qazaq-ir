use crate::mempool::TransactionPool;
use crate::state::State as NodeState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use qazaq_ir::RootEntity;
use std::sync::{Arc, Mutex};

/// Thread-safe application state shared across all HTTP handlers and the background Execution Engine.
#[derive(Clone)]
pub struct AppState {
    pub mempool: Arc<Mutex<TransactionPool>>,
    pub state: Arc<Mutex<NodeState>>,
    pub p2p_sender: tokio::sync::mpsc::UnboundedSender<String>, // Async channel to P2P network
}

/// Initializes the Axum Router with the P2P API Endpoints.
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/intent", post(submit_intent))
        .route("/balance/{address}", get(get_balance))
        .with_state(app_state)
}

/// POST /intent
/// Accepts a raw JSON `RouterPayload` string mapped from a DApp/LLM client.
/// Validates mathematically in O(1) time and adds to the Mempool on success.
async fn submit_intent(State(app_state): State<AppState>, body: String) -> impl IntoResponse {
    let mut mempool = app_state.mempool.lock().unwrap();

    match mempool.process_incoming_intent(&body) {
        Ok(_) => {
            println!("📥 [API] Intent successfully ingested into Mempool.");

            // Broadcast intent to the global P2P network
            if let Err(e) = app_state.p2p_sender.send(body) {
                println!("⚠️ [P2P] Failed to send intent to P2P Swarm thread: {}", e);
            }

            (
                StatusCode::ACCEPTED,
                format!(
                    "Intent accepted and validated via Qazaq IR. Unconfirmed Tx Count: {}",
                    mempool.unconfirmed_count()
                ),
            )
        }
        Err(e) => {
            println!("⚠️ [API] Intent rejected: {}", e);
            (StatusCode::BAD_REQUEST, format!("Intent Rejected: {}", e))
        }
    }
}

/// GET /balance/:address
/// Queries the In-Memory State Machine for a specific MemoryPointer balance.
async fn get_balance(
    State(app_state): State<AppState>,
    Path(address): Path<usize>,
) -> impl IntoResponse {
    let state = app_state.state.lock().unwrap();
    let root = RootEntity::MemoryPointer(address);
    let balance = state.get_balance(&root);

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "address": address,
            "balance": balance
        })),
    )
}
