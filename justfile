log := 'info'

export RUST_LOG := log

run:
	cargo run

watch:
	cargo watch --ignore 'report/*' --exec 'run'
