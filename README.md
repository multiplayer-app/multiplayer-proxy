![Description](.github/header-proxy.png)

<div align="center">
<a href="https://github.com/multiplayer-app/multiplayer-proxy">
  <img src="https://img.shields.io/github/stars/multiplayer-app/multiplayer-proxy.svg?style=social&label=Star&maxAge=2592000" alt="GitHub stars">
</a>
  <a href="https://github.com/multiplayer-app/multiplayer-proxy/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/multiplayer-app/multiplayer-proxy" alt="License">
  </a>
  <a href="https://multiplayer.app">
    <img src="https://img.shields.io/badge/Visit-multiplayer.app-blue" alt="Visit Multiplayer">
  </a>
  
</div>
<div>
  <p align="center">
    <a href="https://x.com/trymultiplayer">
      <img src="https://img.shields.io/badge/Follow%20on%20X-000000?style=for-the-badge&logo=x&logoColor=white" alt="Follow on X" />
    </a>
    <a href="https://www.linkedin.com/company/multiplayer-app/">
      <img src="https://img.shields.io/badge/Follow%20on%20LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white" alt="Follow on LinkedIn" />
    </a>
    <a href="https://discord.com/invite/q9K3mDzfrx">
      <img src="https://img.shields.io/badge/Join%20our%20Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white" alt="Join our Discord" />
    </a>
  </p>
</div>

# Envoy Proxy

This directory contains an Envoy proxy setup with custom WASM extensions for OTLP (OpenTelemetry Protocol) payload capture.

## Overview

The Envoy proxy is configured to:

- Listen on port 8080
- Route requests with `/v1` prefix to a backend service
- Capture request/response payloads using a custom WASM extension
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
