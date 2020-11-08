latest= v0.1.0

bench_save:
	cargo bench --bench mass  -- --save-baseline $latest

bench:
	cargo bench --bench mass -- --baseline $latest

flame:
	flamegraph -o flamegraphs/v0.1.0.svg -- target/release/main

