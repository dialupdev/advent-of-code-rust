default:
	@just --list

run year day:
	@cargo run --package "advent-of-code-{{year}}" --bin "{{day}}"

test year day:
	@cargo test --package "advent-of-code-{{year}}" --bin "{{day}}"
