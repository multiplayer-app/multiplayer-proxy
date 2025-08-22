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

- **Data Masking**: Automatically masks sensitive data in:
  - Request/response headers (e.g., authorization, cookies)
  - Request/response bodies (e.g., passwords, tokens, API keys)
  - Configurable field lists for both headers and body content

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
              "otlp_collector_url": "http://otel-collector:4318"
            }
```

### Configuration Parameters

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

#### Configuration Options

- `otlp_collector_cluster_name` (required): Name of the Envoy cluster for the OTLP collector
- `otlp_collector_authority` (required): Authority/hostname for the OTLP collector
- `otlp_collector_path` (optional): Path for the OTLP traces endpoint
- `otlp_collector_api_key` (optional): API key for authentication with the OTLP collector
- `capture_request_headers` (optional, default: true): Whether to capture request headers
- `capture_request_body` (optional, default: true): Whether to capture request body
- `capture_response_headers` (optional, default: true): Whether to capture response headers
- `capture_response_body` (optional, default: true): Whether to capture response body
- `max_body_size_bytes` (optional, default: 1048576): Maximum body size to capture in bytes (1MB)
- `headers_to_include` (optional): List of headers to always include in capture
- `headers_to_exclude` (optional): List of headers to exclude from capture (for security)
- `is_mask_body_enabled` (optional, default: true): Whether to enable masking of sensitive fields in request/response bodies
- `is_mask_headers_enabled` (optional, default: true): Whether to enable masking of sensitive headers
- `mask_body_fields_list` (optional): List of field names to mask in JSON bodies (uses default sensitive fields if not specified)
- `mask_headers_list` (optional): List of header names to mask (uses default sensitive headers if not specified)

#### Example Configuration

```json
{
  "otlp_collector_cluster_name": "otel-collector",
  "otlp_collector_authority": "api.multiplayer.app",
  "otlp_collector_path": "/v1/traces",
  "otlp_collector_api_key": "eyJhbGciOiJ...4hnOOYvZT4HTYVUvvUtjI",
  "capture_request_headers": true,
  "capture_request_body": true,
  "capture_response_headers": true,
  "capture_response_body": true,
  "max_body_size_bytes": 1048576,
  "headers_to_include": ["content-type", "user-agent", "x-request-id"],
  "headers_to_exclude": ["authorization", "cookie", "x-api-key"],
  "is_mask_body_enabled": true,
  "is_mask_headers_enabled": true,
  "mask_body_fields_list": ["password", "token", "secret", "api_key"],
  "mask_headers_list": ["authorization", "cookie", "set-cookie"]
}
```

## Data Masking

The extension automatically masks sensitive data to protect privacy and security. By default, it masks common sensitive fields and headers.

### Default Sensitive Fields (Body)

The extension automatically masks these field names in JSON request/response bodies:
- Authentication: `password`, `pass`, `passwd`, `pwd`, `token`, `access_token`, `refresh_token`, `secret`, `api_key`, `jwt`
- Session: `session_id`, `sessionToken`, `auth_token`
- Security: `client_secret`, `private_key`, `encryption_key`
- Financial: `credit_card`, `card_number`, `cvv`, `cvc`, `ssn`, `pin`
- Personal: `email`, `phone_number`, `address`, `dob`, `tax_id`, `passport_number`

### Default Sensitive Headers

The extension automatically masks these headers:
- `authorization`, `cookie`, `set-cookie`, `proxyAuthorization`

### Customizing Masking

You can customize which fields and headers are masked using the configuration options:
- `is_mask_body_enabled`: Enable/disable body masking
- `is_mask_headers_enabled`: Enable/disable header masking  
- `mask_body_fields_list`: Custom list of body fields to mask
- `mask_headers_list`: Custom list of headers to mask

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

Check for [example](../../envoy-config.yaml) envoy proxy config


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
