latest= V0.1.0-alpha-52c95647
previous= V0.1.0-alpha-81a7f91c

bench_save:
	cargo bench --bench mass  -- --save-baseline $(latest)

bench:
	cargo bench --bench mass -- --baseline $(previous)

flame: build
	flamegraph -o flamegraphs/$(latest).svg -- target/release/main

build:
	cargo build --release