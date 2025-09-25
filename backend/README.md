# BitVMX Tic-Tac-Toe Backend

A Rust-based backend service for the BitVMX Tic-Tac-Toe game, built with Axum web framework and integrated with BitVMX RPC for peer-to-peer communication.

## Architecture Overview

The backend is structured as a modular Axum application with the following key components:

- **Routes**: API endpoint definitions with OpenAPI documentation
- **Services**: Business logic for processing requests
- **App State**: Thread-safe shared state management
- **Stores**: Thread-safe shared stored information
- **Models**: Data structures with TypeScript bindings
- **Job Worker**: Asynchronous background task processing
- **RPC Client**: BitVMX broker communication with correlation ID matching

## API Documentation

The application automatically generates OpenAPI/Swagger documentation using Utoipa. Access the documentation at:

- **Swagger UI**:
  - Player 1 `http://localhost:8080/`
  - Player 2 `http://localhost:8081/`
- **OpenAPI JSON**:
  - Player 1 `http://localhost:8080/api-docs/openapi.json`
  - Player 2 `http://localhost:8081/api-docs/openapi.json`

## Configuration

Configuration is managed through YAML files in the `configs/` directory:

```yaml
# configs/player_1.yaml
bitvmx:
  broker_port: 22222
```

### Environment Variables

The following environment variables can be used to configure the application:

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `CONFIG_FILE` | Configuration file name (without .yaml extension) | `player_1` | `CONFIG_FILE=player_2` |
| `RUST_LOG` | Logging level (debug, info, warn, error) | `info` | `RUST_LOG=debug` |
| `APP_SERVER__HOST` | Server host address | `0.0.0.0` | `APP_SERVER__HOST=127.0.0.1` |
| `APP_SERVER__PORT` | Server port number | `8080` | `APP_SERVER__PORT=8080` |
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

## Development

### Prerequisites

- Git <https://git-scm.com/downloads>
- Rust 1.85+ and Cargo <https://doc.rust-lang.org/cargo/getting-started/installation.html>
- Docker <https://docs.docker.com/engine/install/>
- Bitcoind running (they will be run when executing `bash start.sh`)
- BitVMX RPC server running (they will be run when executing `bash start.sh`)

### Bitcoin Configuration

The Bitcoin node is configured with the following settings:

- **btc-rpc-explorer**: Runs on port 4000 for [blockchain exploration and transaction monitoring](https://github.com/janoside/btc-rpc-explorer)
- **Auto mining**: Set to mine 1 block per 5 seconds for development and testing purposes at [scripts/start-bitcoin.sh](./scripts/start-bitcoin.sh)

### Building

```bash
bash scripts/build.sh
cargo build
```

### Running

Start bitcoin, explorer, auto miner,  bitvmx, job-dispatcher and backend with:

```bash
bash start
```

If you want to run them independently youn can:

Run bitcoin, explorer:

```bash
bash scripts/start-bitcoin.sh
```

Run bitcoin auto mining:

```bash
bash scripts/auto-mine.sh
```

Run bitvmx operator 1 and 2:

```bash
bash scripts/start-op-1.sh 
bash scripts/start-op-2.sh 
```

Run bitvmx job dispatcher 1 and 2:

```bash
bash scripts/start-dispatcher-1.sh 
bash scripts/start-disptcher-2.sh 
```

Run the backend api server for player 1 and 2:

```bash
bash scripts/start-player-1.sh 
bash scripts/start-player-2.sh 
```

### TypeScript Bindings Generation

```bash
cargo test --lib  # Generates TypeScript bindings during test compilation
```

## Module Documentation

Detailed documentation is available for key modules:

### RPC Client (`src/rpc/`)

- **Purpose**: Manages communication with BitVMX broker system
- **Key Features**: Request-response patterns, fire-and-forget messaging, correlation ID matching
- **Documentation**: [RPC Client README](./src/rpc/README.md)

### Job Worker (`src/jobs/`)

- **Purpose**: Asynchronous background task processing system
- **Key Features**: Job queuing, parallel execution, graceful shutdown
- **Documentation**: [Job Worker README](./src/jobs/README.md)
