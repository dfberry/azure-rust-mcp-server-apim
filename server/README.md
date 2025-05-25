# Rust Song API Server

A high-performance RESTful API service built with Axum that provides random song information. The service returns song titles, artists, and memorable lyrics in JSON format, designed to be easily integrated with Azure API Management.

## Features

- RESTful API endpoint for fetching random songs
- JSON response format
- Built with Axum web framework
- CORS support
- Structured logging with tracing
- Containerized deployment ready
- Azure Container Apps compatible

## Prerequisites

- Rust (latest stable version)
- Docker (for containerized deployment)
- Azure CLI (for deployment to Azure)

## Getting Started

1. Clone the repository and navigate to the server directory:
   ```bash
   cd server
   ```

2. Install dependencies and build the project:
   ```bash
   cargo build
   ```

3. Run the server locally:
   ```bash
   cargo run
   ```

The server will start on the default port (usually 3000).

## API Endpoints

### GET /song

Retrieves a random song from the collection.

**Response Format:**
```json
{
  "title": "string",
  "artist": "string",
  "lyric": "string"
}
```

**Example Response:**
```json
{
  "title": "Bohemian Rhapsody",
  "artist": "Queen",
  "lyric": "Is this the real life? Is this just fantasy?"
}
```

## Environment Variables

The server supports configuration through environment variables. Create a `.env` file in the server directory with the following variables:

```env
PORT=3000
RUST_LOG=info
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

### Running with Docker

1. Build the Docker image:
   ```bash
   docker build -t song-api .
   ```

2. Run the container:
   ```bash
   docker run -p 3000:3000 song-api
   ```

## Deployment

The service is designed to be deployed to Azure Container Apps. Refer to the `/infra` directory for the Bicep infrastructure as code templates.

## Dependencies

Key dependencies include:
- `axum`: Web framework
- `tokio`: Async runtime
- `serde`: Serialization/deserialization
- `tower-http`: HTTP middleware
- `tracing`: Logging and instrumentation

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
