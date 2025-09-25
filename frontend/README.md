# BitVMX Hackathon frontend

This a web application designed for playing various games, such as "Add Numbers". This application is developed using Next.js.

## Prerequisites

- Nodejs v22 <https://nodejs.org/en/download>
- Yarn <https://nodejs.org/en/download>

## Install

```bash
yarn install
```

## Backend Configuration

This configuration system allows the frontend to automatically connect to different backends based on the port it's running on.

### Port Mapping

- **Port 3000** → Backend 1 (http://localhost:8080)
- **Port 3001** → Backend 2 (http://localhost:8081)

### Running Multiple Instances

To run the frontend on different ports:

```bash
# Terminal 1 - Frontend on port 3000 (connects to Backend 1)
PORT=3000 yarn dev

# Terminal 2 - Frontend on port 3001 (connects to Backend 2)  
PORT=3001 yarn dev

# Or all together with the following command:
yarn dev:multiple

```

### Backend Setup

Make sure your backends are running on the configured ports:

- Backend 1: http://localhost:8080
- Backend 2: http://localhost:8081

### Adding New Backend Configurations

To add support for additional players or change ports, update `frontend/src/config/backend.ts`:

```typescript
const BACKEND_CONFIGS: Record<string, BackendConfig> = {
  '3000': {
    baseUrl: 'http://localhost:8080',
    name: 'Backend 1',
    port: 8080,
  },
  '3001': {
    baseUrl: 'http://localhost:8081',
    name: 'Backend 2', 
    port: 8081,
  },
  '3002': {  // New configuration
    baseUrl: 'http://localhost:8082',
    name: 'Backend 3',
    port: 8082,
  },
};
```
