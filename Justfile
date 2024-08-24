tags:
	ctags -R --languages=Rust --langmap=Rust:.rs --exclude=target .

rain-bench-base:
	cargo test --release --bench matrix_benchmarks -- --save-baseline started_from_here



