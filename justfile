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

rollback-preview:
	cargo run -- rollback --preview

integrate:
	cargo run -- integrate

atuin-bootstrap:
	./scripts/atuin-bootstrap.sh
