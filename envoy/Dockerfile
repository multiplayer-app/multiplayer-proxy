FROM envoyproxy/envoy:v1.33-latest

COPY ./envoy-config.yaml etc/envoy/envoy.yaml
COPY ./extensions/payload-otlp/build/otlp_capture_payload.wasm /etc/envoy/extensions/otlp_capture_payload.wasm

RUN chmod go+r /etc/envoy/envoy.yaml

CMD ["/usr/local/bin/envoy", "-c", "/etc/envoy/envoy.yaml"]
