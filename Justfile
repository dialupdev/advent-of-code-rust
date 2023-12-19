default:
	@just --list

run year day:
	@cargo run --package "advent-of-code-{{year}}" --bin "{{day}}"

test year day:
	@RUST_MIN_STACK=8388608 cargo test --package "advent-of-code-{{year}}" --bin "{{day}}"
