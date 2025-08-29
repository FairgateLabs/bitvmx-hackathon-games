# BitVMX Hackathon Games

A decentralized game built on BitVMX, featuring a robust architecture with both backend and frontend components.

The backend, located in the `/backend` directory, is developed using Rust with the Axum framework. 
The frontend, found in the `/frontend` directory, is built with TypeScript.

## Frontend
For more information about the frontend, please refer to the [BitVMX Hackathon frontend README](./frontend/README.md).

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
