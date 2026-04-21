default:
	just --list

doctor:
	cargo run -- doctor

plan:
	cargo run -- plan

apply:
	cargo run -- apply

sync:
	cargo run -- sync

rollback:
	cargo run -- rollback

integrate:
	cargo run -- integrate
