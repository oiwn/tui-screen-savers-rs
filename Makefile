rain-bench-base:
	cargo test --release --bench matrix_benchmarks -- --save-baseline started_from_here

run-release:
	cargo build --release && ./target/release/matrix-rs

coverage:
	export RUSTFLAGS="-Cinstrument-coverage" && \
	cargo build && \
	LLVM_PROFILE_FILE="dc-%p-%m.profraw" && \
	cargo test && \
	grcov . -s . --binary-path ./target/debug/ -t html \
	--branch --ignore-not-existing -o ./target/debug/coverage/