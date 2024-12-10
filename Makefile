prod-debug:
	RUST_LOG=debug cargo run --bin prod

prod-trace:
	RUST_LOG=trace cargo run --bin prod

detailed-prod-debug:
	TRACING_CONFIG=detailed-spans RUST_LOG=debug cargo run --bin prod

detailed-prod-trace:
	TRACING_CONFIG=detailed-spans RUST_LOG=trace cargo run --bin prod

detailed-prod-trace-metrics:
	TRACING_CONFIG=detailed-spans RUST_LOG=trace cargo run --bin prod --features=enable-metrics
