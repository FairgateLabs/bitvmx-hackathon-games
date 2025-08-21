# Tic-Tac-Toe Backend

A Rust backend for the BitVMX Hackathon tic-tac-toe game, built with Axum web framework.

## Key Features

- **REST API**: Built with Axum web framework, includes Swagger with using utoipa framwork
- **TypeScript Types**: Auto generated TypeScript type definitions

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

The API is documented using **OpenAPI/Swagger**, you can access the interactive documentation at:

- **Swagger UI**: http://localhost:8080/swagger-ui
- **OpenAPI JSON**: http://localhost:8080/api-docs/openapi.json


## TypeScript Types

TypeScript types are **automatically generated** using `ts-rs` from the Rust types in the bindings folder.

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

## Testing

Run tests with:

```bash
cargo test
```

## Error Handling

The application implements comprehensive error handling at the endpoint level:

### Error Response Format
All errors return a structured `ErrorResponse`:
```json
{
  "error": "Error message",
  "code": "ERROR_CODE"
}
```

### HTTP Status Codes
- `200 OK` - Successful operations
- `201 Created` - Game created successfully
- `400 Bad Request` - Invalid request data or game logic errors
- `404 Not Found` - Game not found
- `409 Conflict` - Invalid move (cell already occupied, wrong player turn, etc.)
- `422 Unprocessable Entity` - Game already finished

### Error Scenarios
- **Invalid moves**: Cell already occupied, wrong player turn, game finished
- **Game not found**: Invalid game ID
- **Invalid request data**: Missing required fields, invalid JSON
- **Game logic errors**: Attempting to play on finished game

## Configuration

The application uses a `config.yaml` file for configuration, but all settings can also be overridden using environment variables with the `APP_` prefix.

### Configuration File

```yaml
server:
  host: "0.0.0.0"
  port: 8080

logging:
  level: "info"

cors:
  allowed_origins: ["*"]
  allowed_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
  allowed_headers: ["*"]
```

### Environment Variables

All configuration values can be set using environment variables with the `APP_` prefix:

| Environment Variable | Description | Default |
|---------------------|-------------|---------|
| `APP_SERVER__HOST` | Server host address | `0.0.0.0` |
| `APP_SERVER__PORT` | Server port number | `8080` |
| `APP_LOGGING__LEVEL` or `RUST_LOG` | Logging level (debug, info, warn, error) | `info` |
| `APP_CORS__ALLOWED_ORIGINS` | Comma-separated list of allowed origins | `*` |
| `APP_CORS__ALLOWED_METHODS` | Comma-separated list of allowed HTTP methods | `GET,POST,PUT,DELETE,OPTIONS` |
| `APP_CORS__ALLOWED_HEADERS` | Comma-separated list of allowed headers | `*` |

**Note**: Environment variables take precedence over the configuration file values.

## License

MIT License - see LICENSE file for details.
