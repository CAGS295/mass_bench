
latest= V0.1.0-alpha-78ef69c9
previous= V0.1.0-alpha-bench-3c78dc82

bench_save:
	cargo bench --bench mass  -- --save-baseline $(latest)

bench:
	cargo bench --bench mass -- --baseline $(previous)

flame: build
	flamegraph -o flamegraphs/$(latest).svg -- target/release/main

build:
	cargo build --release