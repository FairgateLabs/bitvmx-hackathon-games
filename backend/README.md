# BitVMX Tic-Tac-Toe Backend

A Rust backend for the BitVMX Tic-Tac-Toe game using Axum framework.

## Features

- RESTful API for Tic-Tac-Toe game management
- Add Numbers game functionality
- Health check endpoints
- Request ID tracking for all logs
- OpenAPI/Swagger documentation
- CORS support

### How it works

1. **Automatic Span Generation**: Each request automatically gets a tracing span
2. **Function Instrumentation**: Handlers are decorated with `#[instrument]` for automatic logging
3. **Log Correlation**: All logs within a request span are automatically correlated
4. **Structured Logging**: Logs include function names, parameters, and timing information

### Using Tracing in Handlers

Simply add the `#[instrument]` attribute to your handler functions:

```rust
use tracing::instrument;

#[instrument]
pub async fn my_handler() -> Json<Response> {
    tracing::info!("Processing request");
    
    // Your handler logic here
    
    tracing::info!("Request completed successfully");
    
    Json(response)
}
```

### Example Log Output

With tracing enabled, your logs will look like:

```
2024-01-15T10:30:45.123Z INFO health_check{request_id=550e8400-e29b-41d4-a716-446655440000}: Health check requested
2024-01-15T10:30:45.124Z INFO health_check{request_id=550e8400-e29b-41d4-a716-446655440000}: Health check completed with timestamp=1705315845125
```

### Advanced Usage

You can customize the instrumentation by adding parameters to `#[instrument]`:

```rust
#[instrument(skip(store), fields(player_name = %request.player_name))]
pub async fn create_game(
    State(store): State<Arc<Mutex<GameStore>>>,
    request: CreateGameRequest,
) -> Result<CreateGameResponse, (StatusCode, Json<ErrorResponse>)> {
    tracing::info!("Creating new game");
    // ...
}
```

## Running the Application

```bash
cargo run
```

The server will start on the configured address (default: `0.0.0.0:8080`).

## API Documentation

Once the server is running, you can access the Swagger UI documentation at:
- http://localhost:8080/

## Configuration

The application uses configuration files in the `configs/` directory. See the configuration module for details.

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
