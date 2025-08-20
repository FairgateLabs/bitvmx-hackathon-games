# Tic-Tac-Toe Backend

A Rust backend for the BitVMX Hackathon tic-tac-toe game, built with Axum web framework.

## Features

- **REST API**: Built with Axum web framework
- **TypeScript Types**: Manual TypeScript type definitions
- **Game Logic**: Complete tic-tac-toe game implementation
- **CORS Support**: Configured for cross-origin requests
- **Logging**: Structured logging with tracing

## Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Cargo (comes with Rust)

## Setup

1. **Build the project:**
   ```bash
   cargo build
   ```

2. **Run the server:**
   ```bash
   cargo run
   ```

The server will start on `http://localhost:8080`

## API Documentation

The API is documented using **OpenAPI/Swagger with automatic generation** from Rust types using `utoipa`. You can access the interactive documentation at:

- **Swagger UI**: http://localhost:8080/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api-docs/openapi.json

### Automatic Documentation Generation

The OpenAPI specification is **automatically generated** from:
- **Rust structs and enums** with `#[derive(ToSchema)]`
- **API endpoints** with `#[utoipa::path]` annotations
- **Request/response types** with proper schema definitions

### Endpoints

- `GET /` - Health check endpoint
- `POST /api/game` - Create a new game
- `GET /api/game/{id}` - Get game details
- `POST /api/game/{id}/move` - Make a move
- `GET /api/game/{id}/status` - Get game status

### Using Swagger UI

1. Start the server: `cargo run`
2. Open your browser and go to: http://localhost:8080/swagger-ui
3. You can now:
   - View all available endpoints
   - Test the API directly from the browser
   - See request/response schemas (automatically generated from Rust types)
   - Try out different parameters
   - View detailed type information



## TypeScript Types

TypeScript types are available at:
- `../types/types.ts` (project root directory)

The Rust types are annotated with `#[derive(TS)]` and `#[ts(export)]` attributes for TypeScript generation. Currently, the types are manually maintained, but you can use ts-rs to generate them automatically.

To generate TypeScript types from Rust types, you can use the ts-rs crate. The types are already annotated with the necessary attributes.

## API Examples

### Create a Game
```bash
curl -X POST http://localhost:8080/api/game \
  -H "Content-Type: application/json" \
  -d '{"player_name": "Alice"}'
```

### Make a Move
```bash
curl -X POST http://localhost:8080/api/game/{game-id}/move \
  -H "Content-Type: application/json" \
  -d '{
    "player": "X",
    "position": {
      "row": 0,
      "col": 0
    }
  }'
```

### Get Game Status
```bash
curl http://localhost:8080/api/game/{game-id}/status
```

## Game Rules

- Players take turns placing X and O on a 3x3 grid
- First player to get 3 in a row (horizontally, vertically, or diagonally) wins
- If all cells are filled without a winner, the game is a draw
- Game states: Waiting, InProgress, Won, Draw

## Development

### Project Structure
```
src/
├── main.rs          # Application entry point
├── types.rs         # TypeScript-exportable types
├── models.rs        # Game logic and state management
└── api/
    ├── mod.rs       # API module
    ├── health.rs    # Health check endpoints
    └── game.rs      # Game management endpoints
```

### Adding New Types

To add new types that should be exported to TypeScript:

1. Add the `#[derive(TS)]` and `#[ts(export)]` attributes to your struct/enum
2. Import `ts_rs::TS` in the file
3. Update the corresponding TypeScript types in `../types/types.ts`

Example:
```rust
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MyType {
    pub field: String,
}
```

Then add to `../types/types.ts`:
```typescript
export interface MyType {
  field: string;
}
```

### Adding New Endpoints

1. Create the endpoint function
2. Add the route to `main.rs`
3. Update the TypeScript types if needed

## Environment Variables

- `RUST_LOG`: Log level (default: "info")
- `PORT`: Server port (default: 8080)

## Testing

Run tests with:
```bash
cargo test
```

## License

MIT License - see LICENSE file for details.
