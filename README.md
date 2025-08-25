# 🔐 Sigillum - Document Authentication System

A system for cryptographic document authenticity verification using blockchain technology and Merkle trees.

## 🎯 Project Goal

To provide proof of document existence at a specific point in time and guarantee its immutability after notarization.

## ✨ Features

- **Document Hashing** - SHA-256 for confidentiality
- **Blockchain Notarization** - Recording hashes in immutable blocks
- **Merkle Trees** - Efficient document inclusion verification
- **Web Interface** - User-friendly UI for system interaction
- **REST API** - Programmatic access to functionality

## 🛠 Technology Stack

### Backend
- **Rust** - Programming language
- **Axum** - Web framework
- **Tera** - HTML templating
- **SHA2/SHA3** - Cryptographic hashing
- **Serde** - Serialization/deserialization
- **Tokio** - Async runtime

### Frontend
- **HTML5/CSS3** - User interface
- **JavaScript** - Interactivity
- **Bootstrap-like** - Styling (built-in styles)

## 📦 Installation and Running

### Requirements
- Rust 1.70+
- Cargo
- Git

### Cloning and Building

```bash
# Clone repository
git clone <repository-url>
cd notary-system

# Build project
cargo build --release

# Run server
cargo run