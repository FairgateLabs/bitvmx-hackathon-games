# BitVMX Tic-Tac-Toe Backend

A Rust-based backend service for the BitVMX Tic-Tac-Toe game, built with Axum web framework and integrated with BitVMX RPC for peer-to-peer communication.

## Architecture Overview

The backend is structured as a modular Axum application with the following key components:

- **Routes**: API endpoint definitions with OpenAPI documentation
- **Handlers**: Business logic for processing requests
- **Stores**: Thread-safe shared state management
- **Types**: Data structures with TypeScript bindings
- **BitVMX Integration**: RPC client for peer-to-peer communication

## BitVMX Integration Flow

The BitVMX integration enables peer-to-peer communication between game participants. Here's the detailed flow:

### 1. Application Startup

```mermaid
sequenceDiagram
    participant Main as main.rs
    participant Config as config.rs
    participant BitVMXClient as BitVMXClient
    participant Store as BitVMXStore
    participant Server as Axum Server

    Main->>Config: Load configuration
    Config-->>Main: Config object
    Main->>BitVMXClient: Initialize singleton client
    BitVMXClient->>Store: Set connected status
    Main->>Server: Start Axum server
    Main->>BitVMXClient: Start message receiving loop
    Note over BitVMXClient: Continuously listen for RPC messages
```

### 2. P2P Communication Setup

```mermaid
sequenceDiagram
    participant Client as API Client
    participant API as /bitvmx/comm-info
    participant Handler as bitvmx handler
    participant Store as BitVMXStore
    participant BitVMX as BitVMX RPC

    Client->>API: GET /bitvmx/comm-info
    API->>Handler: get_comm_info()
    Handler->>Store: get_p2p_address()
    Store-->>Handler: P2PAddress {address, peer_id}
    Handler-->>API: JSON response
    API-->>Client: P2P communication info

    Note over Client,BitVMX: Client uses P2P info to establish direct connection
```

### 3. Aggregated Key Submission

```mermaid
sequenceDiagram
    participant Client as API Client
    participant API as /bitvmx/setup-aggregated-key
    participant Handler as bitvmx handler
    participant BitVMXClient as BitVMXClient
    participant BitVMX as BitVMX RPC

    Client->>API: POST /bitvmx/setup-aggregated-key
    Note over Client: Body: {id: "uuid", addresses: [{address, peer_id}]}
    
    API->>Handler: submit_aggregated_key(setup_key)
    Handler->>Handler: Validate setup_key
    Note over Handler: Check id not empty, addresses not empty
    
    alt Validation passes
        Handler->>BitVMXClient: send_message(SetupKey)
        BitVMXClient->>BitVMX: Submit to RPC
        BitVMX-->>BitVMXClient: Success response
        BitVMXClient-->>Handler: Success
        Handler-->>API: Ok(())
        API-->>Client: 200 OK
    else Validation fails
        Handler-->>API: 400 Bad Request
        API-->>Client: Error response
    end
```

### 4. RPC Message Handling

```mermaid
sequenceDiagram
    participant BitVMX as BitVMX RPC
    participant BitVMXClient as BitVMXClient
    participant Handler as bitvmx_rpc handler
    participant Store as BitVMXStore
    participant API as API Handlers

    BitVMX->>BitVMXClient: Incoming RPC message
    BitVMXClient->>Handler: receive_message()
    Handler->>Handler: Parse message type
    
    alt Message type: P2PAddress
        Handler->>Store: set_p2p_address(address)
        Store-->>Handler: Success
    else Message type: SetupKey
        Handler->>Store: notify_handlers()
        Store->>API: Call registered handlers
        API-->>Store: Process message
    else Other message types
        Handler->>Handler: Handle according to type
    end
    
    Handler-->>BitVMXClient: Processed
    BitVMXClient-->>BitVMX: Acknowledge
```



## Configuration

Configuration is managed through YAML files in the `configs/` directory:

```yaml
# configs/player_1.yaml
bitvmx:
  broker_port: 8080
  l2_id: "player_1"
```

### Environment Variables

The following environment variables can be used to configure the application:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `CONFIG_FILE` | Configuration file name (without .yaml extension) | `player_1` | `CONFIG_FILE=player_2` |
| `RUST_LOG` | Logging level (debug, info, warn, error) | `info` | `RUST_LOG=debug` |
| `APP_SERVER__HOST` | Server host address | `0.0.0.0` | `APP_SERVER__HOST=127.0.0.1` |
| `APP_SERVER__PORT` | Server port number | `8080` | `APP_SERVER__PORT=3000` |
| `APP_CORS__ALLOWED_ORIGINS` | Comma-separated list of allowed origins | `*` | `APP_CORS__ALLOWED_ORIGINS=http://localhost:3000,https://example.com` |

### Available Configuration Files

The application comes with two pre-configured files in the `configs/` directory:

- `player_1.yaml` - Default configuration for player 1
- `player_2.yaml` - Configuration for player 2

### Running with Custom Configuration

```bash
# Use a different configuration file
CONFIG_FILE=player_2 cargo run
```

## Testing

The application includes comprehensive testing:

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end API testing
- **TypeScript Bindings**: Generated from Rust types for frontend integration

### Running Tests
```bash
cargo test                    # Run all tests
cargo test --test bitvmx     # Run BitVMX integration tests
```

## Development

### Prerequisites
- Rust 1.70+
- Cargo
- BitVMX RPC server running

### Building
```bash
cargo build
cargo build --release
```

### Running
```bash
cargo run
```

### TypeScript Bindings Generation
```bash
cargo test --lib  # Generates bindings during test compilation
```

## API Documentation

The application automatically generates OpenAPI/Swagger documentation using Utoipa. Access the documentation at:

- **Swagger UI**: `http://localhost:3000/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:3000/api-docs/openapi.json`

## Message Flow Summary

1. **Startup**: Application initializes BitVMX client and starts message receiving loop
2. **P2P Setup**: Clients retrieve P2P addresses via `/bitvmx/comm-info`
3. **Key Submission**: Clients submit aggregated keys via `/bitvmx/setup-aggregated-key`
4. **RPC Communication**: Continuous message exchange with BitVMX RPC
5. **State Management**: Centralized store maintains connection state and P2P information

This architecture provides a robust, scalable foundation for peer-to-peer game communication while maintaining clean separation of concerns and comprehensive error handling.
