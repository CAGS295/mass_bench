latest= V0.1.0-alpha-851ebbd9
previous= V0.1.0-alpha-3c4cf5aa

bench_save:
	cargo bench --bench mass  -- --save-baseline $(latest)

bench:
	cargo bench --bench mass -- --baseline $(previous)

flame: build
	flamegraph -o flamegraphs/$(latest).svg -- target/release/main

build:
	cargo build --release