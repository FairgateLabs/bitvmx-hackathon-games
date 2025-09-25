# BitVMX RPC Client

This module provides a client for communicating with the BitVMX broker system. It handles asynchronous message sending and response correlation using a request-response pattern with correlation IDs.

## Overview

The RPC client manages communication between the application and BitVMX through a broker system. It supports both synchronous request-response patterns and fire-and-forget messaging, with automatic correlation ID matching for responses.

## Key Components

### RpcClient

The main client struct that handles all RPC communication:

- **client**: Internal Bitvmx broker RPC client
- **my_id**: Source participant ID
- **to_id**: Target participant ID  
- **pending_responses**: Queue of pending response handlers keyed by correlation ID
- **ready**: Flag indicating if the client is ready to handle messages

### Correlation System

Automatic correlation ID generation and matching between requests and responses based on message content and UUIDs. See [Correlation ID System](#correlation-id-system) for details.

### ChainedMap

A data structure that maps correlation IDs to queues of response handlers, allowing multiple handlers to wait for the same response.

## Core Functions

### `connect(my_id, to_id, broker_port, broker_ip, shutdown_tx)`

**Purpose**: Initialize and start the RPC client connection.

**Parameters**:

- `my_id`: Your participant ID in the BitVMX system
- `to_id`: Target participant ID to send messages to
- `broker_port`: Port of the BitVMX broker
- `broker_ip`: Optional IP address of the broker (defaults to localhost)
- `shutdown_tx`: Shutdown signal sender for graceful termination

**Returns**: A tuple containing:

- `Arc<RpcClient>`: The client instance
- `JoinHandle<Result<(), anyhow::Error>>`: Background task handle for the message listener

**Example**:

```rust
let (rpc_client, listener_task) = RpcClient::connect(
    my_id,
    to_id, 
    broker_port,
    Some(broker_ip),
    &shutdown_tx
);
```

### `send_request(message)`

**Purpose**: Send a message and wait for the corresponding response.

**Parameters**:

- `message`: `IncomingBitVMXApiMessages` to send

**Returns**: `Result<OutgoingBitVMXApiMessages, anyhow::Error>`

**Example**:

```rust
let response = rpc_client.send_request(
    IncomingBitVMXApiMessages::GetVar(program_id, "my_var".to_string())
).await?;
```

**Use case**: When you need a response to your request (e.g., getting variable values, transaction status).

### `send_fire_and_forget(message)`

**Purpose**: Send a message without waiting for a response.

**Parameters**:

- `message`: `IncomingBitVMXApiMessages` to send

**Returns**: `Result<String, anyhow::Error>` - the correlation ID of the sent message

**Example**:

```rust
let correlation_id = rpc_client.send_fire_and_forget(
    IncomingBitVMXApiMessages::SetVar(program_id, "my_var".to_string(), value)
).await?;
```

**Use case**: When you don't need an immediate response (e.g., setting variables, dispatching transactions).

### `wait_for_response(correlation_id)`

**Purpose**: Wait for a specific response using a previously obtained correlation ID.

**Parameters**:

- `correlation_id`: String correlation ID to wait for

**Returns**: `Result<OutgoingBitVMXApiMessages, anyhow::Error>`

**Example**:

```rust
let response = rpc_client.wait_for_response(correlation_id).await?;
```

**Use case**: When you sent a fire-and-forget message and later need to wait for its response.

## Message Flow

1. Application calls send_request() or send_fire_and_forget()
2. RPC client generates correlation ID from message content
3. Message is serialized and sent to BitVMX broker
4. Background listener receives response from broker
5. Response correlation ID is matched with pending handlers
6. Response is delivered to waiting handlers via oneshot channels

## Correlation ID System

Correlation IDs are automatically generated based on message content:

- **UUID-based messages**: Use the UUID directly (e.g., `SetVar`, `GetVar`)
- **Special messages**: Use descriptive strings (e.g., `"ping"` for Ping/Pong)
- **Transaction messages**: Combine UUID with transaction name
- **UTXO messages**: Use transaction ID and output index

## Error Handling

- **Timeout errors**: 120-second timeout for all request-response operations
- **Channel errors**: Handles closed channels gracefully
- **Serialization errors**: JSON serialization/deserialization failures
- **Broker errors**: Connection and message sending failures

## Thread Safety

The RPC client is designed for concurrent use:

- Uses `Arc<Mutex<>>` for shared state
- Background listener task handles incoming messages
- Multiple concurrent requests are supported
- Graceful shutdown via broadcast channels

## Usage Example

```rust
use tokio::sync::broadcast;

// Create shutdown channel
let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

// Connect to BitVMX
let (rpc_client, listener_task) = RpcClient::connect(
    my_id,
    to_id, 
    broker_port,
    Some(broker_ip),
    &shutdown_tx
);

// Wait for client to be ready
rpc_client.wait_for_ready(shutdown_rx.resubscribe()).await;

// Send a request and wait for response
let response = rpc_client.send_request(
    IncomingBitVMXApiMessages::GetVar(program_id, "my_var".to_string())
).await?;

// Send fire-and-forget message
let correlation_id = rpc_client.send_fire_and_forget(
    IncomingBitVMXApiMessages::SetVar(program_id, "my_var".to_string(), value)
).await?;

// Later, wait for the response
let response = rpc_client.wait_for_response(correlation_id).await?;

// Graceful shutdown
let _ = shutdown_tx.send(());
let _ = listener_task.await;
```
