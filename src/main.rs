mod blockchain;
mod merkle_tree;
mod models;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
    Json,
};
use axum::body::Bytes;
use axum::http::{StatusCode, HeaderMap};
use blockchain::{Blockchain, SharedBlockchain};
use models::{Document, VerificationResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tera::{Tera, Context};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Init Tera templates
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Template parsing error: {}", e);
            std::process::exit(1);
        }
    };
    
    let tera = Arc::new(tera);

    // Init blockchain
    let blockchain = Arc::new(RwLock::new(Blockchain::new()));

    // Set up routes
    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload_document))
        .route("/verify/:hash", get(verify_document))
        .route("/mine", post(mine_block))
        .route("/api/blockchain", get(get_blockchain))
        .with_state((blockchain, tera));

    // Launch server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn index(
    State((blockchain, tera)): State<(SharedBlockchain, Arc<Tera>)>,
) -> impl IntoResponse {
    let blockchain = blockchain.read().await;
    
    let mut context = Context::new();
    context.insert("blocks_count", &blockchain.chain.len());
    context.insert("pending_documents", &blockchain.pending_documents.len());
    
    match tera.render("index.html", &context) {
        Ok(html) => axum::response::Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
        }
    }
}

async fn upload_document(
    State((blockchain, _)): State<(SharedBlockchain, Arc<Tera>)>,
    bytes: Bytes,
) -> impl IntoResponse {
    let filename = "uploaded_file.bin";
    
    let document = Document::new(filename.to_string(), &bytes);
    
    let mut blockchain = blockchain.write().await;
    blockchain.add_document(document.clone());
    
    Json(json!({
        "success": true,
        "id": document.id,
        "filename": document.filename,
        "hash": document.hash,
        "timestamp": document.timestamp,
    }))
}

async fn verify_document(
    State((blockchain, _)): State<(SharedBlockchain, Arc<Tera>)>,
    axum::extract::Path(hash): axum::extract::Path<String>,
) -> impl IntoResponse {
    let blockchain = blockchain.read().await;
    
    if let Some((block_index, merkle_proof)) = blockchain.verify_document(&hash) {
        let block = &blockchain.chain[block_index];
        Json(VerificationResult {
            exists: true,
            block_index: Some(block_index),
            timestamp: Some(block.timestamp as u64),
            merkle_proof: Some(merkle_proof),
        })
    } else {
        Json(VerificationResult {
            exists: false,
            block_index: None,
            timestamp: None,
            merkle_proof: None,
        })
    }
}

async fn mine_block(
    State((blockchain, _)): State<(SharedBlockchain, Arc<Tera>)>,
) -> Response {
    let mut blockchain = blockchain.write().await;
    
    match blockchain.mine_block() {
        Some(block) => {
            let response = json!({
                "success": true,
                "index": block.index,
                "hash": block.hash,
                "documents_count": block.documents.len(),
            });
            Json(response).into_response()
        },
        None => {
            let response = json!({
                "success": false,
                "error": "No pending documents to mine"
            });
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

async fn get_blockchain(
    State((blockchain, _)): State<(SharedBlockchain, Arc<Tera>)>,
) -> impl IntoResponse {
    let blockchain = blockchain.read().await;
    Json(json!({
        "chain": blockchain.chain,
        "pending_documents": blockchain.pending_documents,
        "is_valid": blockchain.is_valid(),
    }))
}