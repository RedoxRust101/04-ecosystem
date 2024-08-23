# Eco system

## tag v4-3-open-telemetry
```Shell
> sudo docker run -d -p 16686:16686 -p 4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest 

> cargo run --example axum_tracing 
```
