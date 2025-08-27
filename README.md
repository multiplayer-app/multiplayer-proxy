![Description](./docs/img/header-proxy.png)

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

# Multiplayer Proxy

The Multiplayer Proxy captures request, response and header data for Multiplayer Full Stack Session Recordings.

It's built on [Envoy Proxy](https://www.envoyproxy.io/) with an [WASM extension](./envoy/extensions/payload-otlp/).

## Example Configuration

An example [Envoy proxy configuration file](./envoy/envoy-config.yaml) in this repository has the following settings. Update as required for your project.

- Listen on port 8080
- Route requests with `/v1` prefix to a backend service
- Capture request/response payloads using a custom WASM extension
- Send telemetry data to Multiplayer OTLP collector

## Extension configuration



The extension accepts a JSON configuration object with the following parameters:

```json
{
  "otlp_collector_cluster_name": "otel-collector",
  "otlp_collector_authority": "api.multiplayer.app",
  "otlp_collector_path": "/v1/traces",
  "otlp_collector_api_key": "your-api-key-here",
  "capture_request_headers": true,
  "capture_request_body": true,
  "capture_response_headers": true,
  "capture_response_body": true,
  "max_body_size_bytes": 1048576,
  "headers_to_include": ["content-type", "user-agent", "x-request-id"],
  "headers_to_exclude": ["authorization", "cookie", "x-api-key"]
}
```

Configuration for extension should be passed to envoy proxy [configuration](./envoy/envoy-config.yaml) (line 38).

For more details about configuring extension and masking rules see the [Multiplayer Proxy Extension](./envoy/extensions/payload-otlp/README.md)


## Example Deployments

### Edge proxy

Route all your traffic through envoy proxy (docker compose example):

```yaml
version: '3.8'

services:
  envoy:
    build:
      context: ./envoy
    container_name: envoy
    environment:
      - MULTIPLAYER_OTLP_KEY="{{YOUR_BACKEND_OTEL_TOKEN}}"
    ports:
      - "8080:8080"
```

To start it, run: `docker compose up -d`

### Service proxy

### Embeded

### Sidecar



## License

MIT — see [LICENSE](./LICENSE).
