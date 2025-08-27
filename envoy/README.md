# Envoy Proxy

This directory contains an Envoy proxy setup with custom WASM extensions for OTLP (OpenTelemetry Protocol) payload capture.

## Overview

The Envoy proxy is configured to:
- Listen on port 8080
- Route requests with `/v1` prefix to a backend service
- Capture request/response payloads using a custom WASM extension
- Automatically mask sensitive data (passwords, tokens, headers) for security
- Send telemetry data to an OTLP collector

## Prerequisites

Before starting the docker-compose setup, ensure you have:

1. **Docker and Docker Compose** installed on your system

## Quick Start

### 1. Build the WASM Extension or use prebuilt one

To build wasm extension from source:

```bash
cd extensions/payload-otlp
./build.sh
```

This script will:
- Check for Rust installation
- Install the appropriate WASM target for your OS
- Build the OTLP capture payload extension
- Output the WASM file to `build/otlp_capture_payload.wasm`

### 2. Set OTLP key

**Important**: You also need to update the `MULTIPLAYER_OTLP_KEY` placeholder in the Envoy configuration file. Open [envoy-config.yaml](./envoy-config.yaml) and replace `{{MULTIPLAYER_OTLP_KEY}}` with your actual API key value.

### 3. Start the Services

From the `envoy` directory, start the docker-compose services:

```bash
docker-compose up
```

## License

MIT — see [LICENSE](https://github.com/multiplayer-app/multiplayer-session-recorder-javascript/blob/main/LICENSE).
