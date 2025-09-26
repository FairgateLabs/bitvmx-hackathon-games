# BitVMX - Add Numbers Frontend

This is a web application designed for playing the "Add Numbers" game. It is developed using Next.js.

## Prerequisites

- Nodejs v22 <https://nodejs.org/en/download>
- Yarn <https://classic.yarnpkg.com/lang/en/docs/install>

## Setup Guide

```bash
yarn install
```

## Running the Frontend for Multiplayer Simulation

To simulate a multiplayer environment, run two instances of the frontend application, each connecting to a different backend. This setup allows you to simulate two players interacting with the game simultaneously, each with their own dedicated backend connection via the BitVMX protocol.

### Port and Backend Mapping

- **Player 1**: Frontend on port 3000 connects to Backend 1 at http://localhost:8080.
- **Player 2**: Frontend on port 3001 connects to Backend 2 at http://localhost:8081.

The frontend automatically connects to the appropriate backend based on the port it's running on.

### Running Multiple Instances

To run the frontend on different ports:

```bash
# Terminal 1 - Frontend on port 3000 (connects to Backend 1)
yarn dev:3000

# Terminal 2 - Frontend on port 3001 (connects to Backend 2)  
yarn dev:3001

# Or all together with the following command:
yarn dev:multiple

```

### Backend Setup

Make sure your backends are running on the configured ports:

- Backend 1: http://localhost:8080
- Backend 2: http://localhost:8081
