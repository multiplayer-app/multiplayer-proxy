# Envoy OTLP Extension

This Envoy proxy extension captures HTTP request/response headers and body, and creates OpenTelemetry (OTLP) spans when OTLP-compatible headers are detected in incoming requests.

## Features

- **OTLP Header Detection**: Automatically detects OTLP-compatible headers:
  - `traceparent` (W3C Trace Context)
  - `b3` (Zipkin B3)
  - `x-trace-id` and `x-span-id` (Custom headers)

- **Request/Response Capture**: Captures and validates:
  - HTTP request headers
  - HTTP response headers
  - Request body (JSON/XML only)
  - Response body (JSON/XML only)

- **OTLP Span Creation**: Creates new spans with:
  - Parent trace context from incoming headers
  - New span ID for the proxy request
  - Captured data as span attributes

- **Span Attributes**: Sets the following attributes on spans:
  - `multiplayer.http.request.headers`
  - `multiplayer.http.response.headers`
  - `multiplayer.http.request.body`
  - `multiplayer.http.response.body`

## Building

### Prerequisites

- Rust toolchain (1.70+)
- `wasm32-wasi` target
- Envoy proxy with WASM support

### Build Commands

```bash
# Install wasm32-wasi target
rustup target add wasm32-wasi

# Build the extension
cargo build --target wasm32-wasi --release

# The WASM file will be created at:
# target/wasm32-wasi/release/payload_otlp.wasm
```

## Configuration

### Envoy Configuration

Add the WASM filter to your Envoy configuration:

```yaml
http_filters:
- name: envoy.filters.http.wasm
  typed_config:
    "@type": type.googleapis.com/udpa.type.v1.TypedStruct
    type_url: type.googleapis.com/envoy.extensions.filters.http.wasm.v3.Wasm
    value:
      config:
        vm_config:
          runtime: "envoy.wasm.runtime.v8"
          code:
            local:
              filename: "/path/to/payload_otlp.wasm"
        configuration:
          "@type": type.googleapis.com/google.protobuf.StringValue
          value: |
            {
              "otlp_collector_url": "http://otel-collector:4317"
            }
```

### Configuration Parameters

- `otlp_collector_url` (required): URL of the OTLP collector endpoint
  - Example: `"http://otel-collector:4317"`
  - Example: `"https://api.honeycomb.io:443"`

## Usage

### OTLP Headers

The extension will create spans when it detects any of these headers:

#### W3C Trace Context
```
traceparent: 00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01
```

#### Zipkin B3
```
b3: 0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-1
```

#### Custom Headers
```
x-trace-id: 0af7651916cd43dd8448eb211c80319c
x-span-id: b7ad6b7169203331
```

### Content Types

The extension validates and captures bodies for these content types:

- `application/json`
- `text/json`
- `application/xml`
- `text/xml`

## Deployment

Check for [example](./envoy-config.yaml) envoy proxy config


## Span Attributes

Each span created by the extension includes:

- **Request Data**:
  - `multiplayer.http.request.headers`: All request headers
  - `multiplayer.http.request.body`: Request body (if JSON/XML)

- **Response Data**:
  - `multiplayer.http.response.headers`: All response headers
  - `multiplayer.http.response.body`: Response body (if JSON/XML)

- **Trace Context**:
  - `multiplayer.http.proxy`: Indicates that span was created by proxy
  - `multiplayer.http.proxy.type`: Proxy type

### Example Span

```json
{
  "name": "envoy_proxy_request",
  "trace_id": "0af7651916cd43dd8448eb211c80319c",
  "span_id": "new_span_id_here",
  "parent_span_id": "b7ad6b7169203331",
  "attributes": {
    "multiplayer.http.request.headers": "{\"content-type\": \"application/json\", \"traceparent\": \"00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01\"}",
    "multiplayer.http.request.body": "{\"key\": \"value\"}",
    "multiplayer.http.response.headers": "{\"content-type\": \"application/json\", \"content-length\": \"25\"}",
    "multiplayer.http.response.body": "{\"status\": \"success\"}",
    "envoy.trace_id": "0af7651916cd43dd8448eb211c80319c",
    "envoy.span_id": "b7ad6b7169203331"
  }
}
```
