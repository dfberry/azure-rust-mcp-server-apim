# Rust Song API with MCP Integration

This workspace implements a RESTful Song API service with Model Context Protocol (MCP) integration. The project demonstrates how to build and deploy a web service that provides random song information while also supporting AI tool integration through MCP.

[Youtube](https://youtu.be/NhkicMSey8o)
[Azure documentation](https://learn.microsoft.com/azure/api-management/export-rest-mcp-server#configure-policies-for-the-mcp-server)

The dev container includes all required tools: Azure CLI, Azure Developer CLI (azd), Rust toolchain, and Docker.

## Open in browser

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/dfberry/azure-rust-mcp-server-apim)


## Open in local dev container

1. Local development prerequisites:
   - [VS Code](https://code.visualstudio.com/)
   - [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

2. Clone and open in dev container:
   ```bash
   git clone https://github.com/yourusername/rust-mcp.git
   cd rust-mcp
   code .
   ```
   When prompted, click "Reopen in Container"

## Deploy with Azure Developer CLI

1. Deploy to Azure:
   ```bash
   azd auth login
   azd init
   azd up
   ```

2. Test the deployment:
   ```bash
   curl https://<your-app-url>/song
   ```

Your custom API is now deployed to Azure Container Apps.

## Create an Azure API Management resource

1. Use the [Azure portal](https://portal.azure.com) to create an Azure API Managment resource. 

2. Create the Song API on the resource using the [openapi.json](./server/openapi.json).

3. [Configure resource for MCP servers](https://learn.microsoft.com/azure/api-management/export-rest-mcp-server#configure-policies-for-the-mcp-server). Estimated time is up to 2.5 hours for Azure to set up the MCP connection for the API Management resource.

## Configure Visual Studio Code to use MCP server

1. In the Azure portal, get the APIM endpoint and subscription key.
2. In Visual Studio Code, use the Command Pallete to add an MCP server with the remote URI.
3. Edit the MCP server to add the required header for `Ocp-Apim-Subscription-Key`:

    ```json
    {
        "files.autoSave": "afterDelay",
        "rust-analyzer.initializeStopped": true,
        "@azure.argTenant": "",
        "mcp": {
            "servers": {
                "my-mcp-server-<UNIQUE_ID>": {
                    "url": "<AZURE_API_MANAGEMENT_ENDPOINT_FOR_MCP>/mcp/sse",
                    "headers": {
                        "Ocp-Apim-Subscription-Key": "<AZURE_API_MANAGEMENT_SUBSCRIPTION_KEY>"
                    }
                }
            }
        }
    }
    ```

4. Use the Chat to ask for a song: `What is a good hello song`.



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

### Manual Setup Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))
- Docker (for container builds)
- Azure CLI (for deployment)
- Azure Developer CLI (azd)

### Local Development (Without Dev Container)

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

