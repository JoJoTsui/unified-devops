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

atuin-login:
	./scripts/atuin-bootstrap.sh login

atuin-register:
	./scripts/atuin-bootstrap.sh register

atuin-sync:
	./scripts/atuin-bootstrap.sh sync

atuin-setup:
	./scripts/atuin-bootstrap.sh setup
