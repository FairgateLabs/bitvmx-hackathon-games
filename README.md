# BitVMX - Add Numbers Game

Add Numbers is a BitVMX-based game where two players compete to solve a mathematical addition. The game uses Bitcoin's dispute resolution protocol to ensure fair play and automatic validation on chain by the BitVMX client. To understand more deeply about BitVMX and the game flow, please visit [Game Flow Readme](GAME_FLOW.md). You can also see more information about the [program and protocol](GAME_PROTOCOL_AND_PROGRAM.md) used. To understand how to play the game, go to [Game Play Readme](GAME_PLAY.md).

## ⚠️ Disclaimer

This repository was created as an example for the Berlin Hackathon. It is intended solely for demonstration purposes and has not been audited or tested in a production environment.

## About This Repository

This repository hosts an Add Numbers game application, composed of backend and frontend. 

The backend, located in the `/backend` directory, is crafted using Rust and the Axum framework to deliver REST APIs and facilitate communication with the BitVMX client via Tarpc. For detailed information, please consult the [Backend Readme](./backend/README.md).

The frontend resides in the `/frontend` directory and is developed using Next.js. It serves as the user interface for the game. For further details, refer to the [Frontend Readme](./frontend/README.md).

## Prerequisites

- Docker (for Bitcoin Regtest node)
- Rust (for backend)
- Node.js (for frontend)
- Yarn (for frontend)

## Quick Start

1. **Clone the Repository**

   ```bash
   git clone https://github.com/FairgateLabs/bitvmx-hackathon-game
   ```

2. **Initialize and Update Submodules**: 
  Navigate to the root of the repository, then initialize and update the git submodules to pull dependencies.

   ```bash
   cd bitvmx-hackathon-game
   git submodule init
   git submodule update
   ```

1. **Set Up Frontend and Backend**: 
   Configure the frontend and backend by navigating to their directories after updating submodules. Ensure both projects are initiated. For setup and execution, see the [Frontend Readme](./frontend/README.md) and [Backend Readme](./backend/README.md).



## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

## 🧩 Part of the BitVMX Ecosystem

This repository is a component of the **BitVMX Ecosystem**, an open framework for disputable computation secured by Bitcoin.  
You can find the index of all BitVMX open-source components at [**FairgateLabs/BitVMX**](https://github.com/FairgateLabs/BitVMX).

---
