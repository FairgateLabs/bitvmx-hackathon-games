# BitVMX - Add Numbers Game

Add Numbers is a BitVMX-based game where two players compete to solve a mathematical addition. The game uses Bitcoin's dispute resolution protocol to ensure fair play and automatic fund distribution. To understand more deeply about BitVMX and the game flow, please visit [Game Flow Readme](GAME_FLOW.md). You can also see more information about the [program and protocol](GAME_PROTOCOL_AND_PROGRAM.md) used. To understand how to play the game, go to [Game Play Readme](GAME_PLAY.md).

## ⚠️ Disclaimer

This repository was created as an example for the Berlin Hackathon. It is intended solely for demonstration purposes and has not been audited or tested in a production environment.

## About This Repository

This repository hosts an Add Numbers game application, composed of both backend and frontend components. The backend, located in the `/backend` directory, is crafted using Rust and the Axum framework to deliver REST APIs and facilitate communication with the BitVMX client via Tarpc. For detailed information, please consult the [Backend Readme](./backend/README.md).

The frontend resides in the `/frontend` directory and is developed using Next.js. It serves as the user interface and communicates with the backend. For further details, refer to the [Frontend Readme](./frontend/README.md).

## Prerequisites

- Docker (for Bitcoin Regtest node)
- Rust (for backend)
- Node.js (for frontend)

## Quick Start

1. **Clone the Repository**: First, clone the repository to your local machine.

   ```bash
   git clone https://github.com/FairgateLabs/bitvmx-hackathon-game
   ```

2. **Initialize and Update Submodules**: After cloning, navigate to the root of the repository and initialize and update the git submodules.

   ```bash
   cd bitvmx-hackathon-game
   git submodule init
   git submodule update
   ```

3. **Navigate to Each Folder**: Once the submodules are updated, navigate to each folder (`frontend` and `backend`) to proceed with their respective setups.
To properly run the example, you need to start both the frontend and backend projects. Follow these steps: For detailed instructions on running the frontend, navigate to the [Frontend Readme](./frontend/README.md) and follow the setup guide. Similarly, for the backend, refer to the [Backend Readme](./backend/README.md) for comprehensive running instructions.

## License

MIT License - see [LICENSE](LICENSE) file for details.
