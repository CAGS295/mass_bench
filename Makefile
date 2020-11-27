latest= V0.1.0-alpha-2d91e4dc

bench_save:
	cargo bench --bench mass  -- --save-baseline $latest

bench:
	cargo bench --bench mass -- --baseline $latest

flame:
	flamegraph -o flamegraphs/$(latest).svg -- target/release/main

