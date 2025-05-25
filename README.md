# Rust MCP (Model Context Protocol) Implementation

This workspace contains two crates:

1. **mcp-client-rust**: A client implementation for connecting to LLM services using the Model Context Protocol
2. **mcp-server-rust**: A server implementation that registers tools and processes tool calls via the MCP protocol

## Structure

```
rust-mcp/
├── client/          # Client implementation
│   └── src/
│       ├── error.rs
│       ├── main.rs
│       └── mcp_client.rs
├── server/          # Server implementation
│   └── src/
│       ├── error.rs
│       ├── main.rs
│       ├── mcp_server.rs
│       └── transport/
│           └── stdio.rs
└── Cargo.toml       # Workspace manifest
```

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- For the client, you'll need an API key for supported LLM providers (e.g., Anthropic)

### Running the Client

```bash
# Navigate to the client directory
cd client

# Set up environment variables (create a .env file)
echo "ANTHROPIC_API_KEY=your_api_key_here" > .env

# Run the client
cargo run
```

### Running the Server

```bash
# Navigate to the server directory
cd server

# Run the server (will use stdio for communication)
cargo run
```

The server implements two example tools:
- `say-hello` - Returns a greeting message
- `echo` - Echoes back the provided message

The server uses stdio transport for communication, making it compatible with any client that implements the MCP protocol.

## Development

To build both crates at once:

```bash
cargo build --workspace
```

To run tests for all crates:

```bash
cargo test --workspace
```

## Understanding the Implementation

### MCP Client

The client is built using the Anthropic Claude API. It sends requests to LLM services and handles responses.

### MCP Server

The server implements the Model Context Protocol, allowing it to:

1. Register tools with names, descriptions, and parameter schemas
2. Process incoming tool call requests
3. Execute the appropriate tool handler
4. Return formatted responses

The server follows the same architecture as the TypeScript example:

```typescript
// Create server instance
const server = new McpServer({
  name: "hello-world",
  version: "1.0.0",
});

// Register tools with schema and handlers
server.tool("say-hello", "Returns a greeting message", {...});
server.tool("echo", "Echoes back the provided message", {...});

// Start the server with a transport
const transport = new StdioServerTransport();
await server.connect(transport);
```

The Rust implementation provides similar functionality with a type-safe API.

## Additional Resources

For more information on the Model Context Protocol, see the [MCP documentation](https://modelcontextprotocol.io/).
