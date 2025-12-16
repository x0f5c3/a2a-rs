# Installation

## Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Comes with Rust
- **Protocol Buffers** (for gRPC): `protoc` compiler

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing Protocol Buffers (for gRPC)

**Ubuntu/Debian:**
```bash
sudo apt-get install protobuf-compiler
```

**macOS:**
```bash
brew install protobuf
```

## Adding to Your Project

```toml
[dependencies]
a2a-rs = { git = "https://github.com/x0f5c3/a2a-rs", features = ["server"] }
tokio = { version = "1", features = ["full"] }
```

### Feature Flags

- `http-server` - HTTP JSON-RPC server
- `ws-server` - WebSocket server
- `grpc-server` - gRPC server (NEW!)
- `http-client` - HTTP client
- `ws-client` - WebSocket client
- `grpc-client` - gRPC client
- `auth` - JWT, OAuth2 support
- `storage-sqlite` - SQLite storage
- `storage-postgres` - PostgreSQL storage

## Building from Source

```bash
git clone https://github.com/x0f5c3/a2a-rs.git
cd a2a-rs
cargo build --all-features
cargo test --all-features
```

## Next Steps

Proceed to the [Quick Start](./quick-start.md) guide!
