# Rust Song API with MCP Integration

This workspace implements a RESTful Song API service with Model Context Protocol (MCP) integration. The project demonstrates how to build and deploy a web service that provides random song information while also supporting AI tool integration through MCP.

[Youtube](https://youtu.be/NhkicMSey8o)
[Azure documentation](https://learn.microsoft.com/azure/api-management/export-rest-mcp-server#configure-policies-for-the-mcp-server)

## Project Structure

```
rust-mcp/
├── server/          # Main API server implementation
│   ├── src/        # Server source code
│   ├── openapi.json # API specification
│   └── Cargo.toml  # Server dependencies
├── song-lib/       # Song generation library
│   ├── src/        # Library source code
│   └── Cargo.toml  # Library dependencies
├── infra/          # Infrastructure as Code
│   ├── main.bicep  # Azure deployment template
│   └── main.bicepparam # Azure deployment parameters
├── http/           # API test files
│   ├── song.http   # REST API tests
│   └── song-mcp.http # MCP integration tests
└── Dockerfile      # Container definition
```

## Features

- RESTful API endpoint for random song generation
- Modular architecture with separate song library
- Azure Container Apps deployment support
- Model Context Protocol (MCP) integration for AI tools
- OpenAPI specification
- Docker containerization
- Infrastructure as Code using Bicep

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Docker (for container builds)
- Azure CLI (for deployment)

### Local Development

1. Build the workspace:
```bash
cargo build --workspace
```

2. Run the tests:
```bash
cargo test --workspace
```

3. Start the server:
```bash
cd server
cargo run
```

4. Try the API:
```bash
curl http://localhost:3000/song
```

### Docker Build

Build and run the container locally:

```bash
docker build -t song-api .
docker run -p 3000:3000 song-api
```

### Azure Deployment

Deploy to Azure Container Apps using the provided Bicep templates:

```bash
cd infra
az deployment sub create --location eastus2 --template-file main.bicep --parameters main.bicepparam
```

## Components

### Song Library (`song-lib`)
A Rust library crate that provides the core functionality for random song generation. See [song-lib/README.md](song-lib/README.md) for details.

### API Server (`server`)
The main web service that exposes the song generation functionality via REST API and MCP. See [server/README.md](server/README.md) for API documentation.

### Infrastructure (`infra`)
Contains Bicep templates for deploying the service to Azure Container Apps with proper configuration and scaling rules.

## API Documentation

The API is documented using OpenAPI 3.0 specification. You can find the full API documentation in `server/openapi.json`.

## Model Context Protocol Integration

The server implements MCP tools for AI integration:
- `get_random_song` - Returns a random song with title, artist, and lyrics
- Support for both REST and MCP interfaces

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
