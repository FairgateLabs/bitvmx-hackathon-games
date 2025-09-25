# BitVMX Hackathon Games

Add Numbers game is a BitVMX-based game where two players compete to solve a mathematical challenge. The game uses Bitcoin's dispute resolution protocol to ensure fair play and automatic fund distribution. You can see more detail at [GAME_FLOW.md](GAME_FLOW.md)

The backend is located in the `/backend` directory, is developed using Rust with the Axum framework to provide REST APIs and interact with Bitvmx client using Tarpc. For more information about the backend, please refer to the [Backend README](./backend/README.md).

The frontend can be found in the `/frontend` directory, is built using TypeScript with React framework and interact with the backend trough Next.js APIs. For more information about the frontend, please refer to the [Frontend README](./frontend/README.md).

## ⚠️ Disclaimer

This library is currently under development and may not be fully stable.
It is not production-ready, has not been audited, and future updates may introduce breaking changes without preserving backward compatibility.

## Prerequisites

- Docker (for Bitcoin Regtest node)
- Rust (for backend)
- Node.js (for frontend)

## Quick Start

### 1. Start Bitcoin Regtest and BitVMX Client

First, start the Bitcoin Regtest container and BitVMX client using the provided script:

```bash
cd backend
bash start.sh
```

This script will:

- Stop and remove any existing `bitcoin-regtest` container
- Start a new Bitcoin Regtest node on port 18443
- Clean up temporary BitVMX client data
- Start the BitVMX client with operation `op_1`

### 2. Run the Backend

In a new terminal, start the backend server:

```bash
cd backend
cargo run
```

The backend will start on `http://0.0.0.0:8080` by default.

### 3. Access the Application

- **API Documentation**: <http://localhost:8080/>
- **Health Check**: <http://localhost:8080/health>

## Configuration

The backend supports multiple configuration files. See the [backend README](backend/README.md) for detailed configuration options.

### Environment Variables

- `CONFIG_FILE`: Configuration file name (default: `player_1`)
- `RUST_LOG`: Logging level (default: `info`)

Example:
```bash
CONFIG_FILE=player_2 RUST_LOG=debug cargo run
```

## Development

### Backend Development

The backend is built with:

- **Rust** with **Axum** web framework
- **BitVMX Broker** rpc framework
- **OpenAPI/Swagger** for API documentation

See [backend/README.md](backend/README.md) for detailed development information.

## License

MIT License - see [LICENSE](LICENSE) file for details.
