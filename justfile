log := 'info'

export RUST_LOG := log

run:
	cargo run

watch:
	cargo watch --clear --ignore 'report/*' --ignore summary.csv --exec 'run generate'
