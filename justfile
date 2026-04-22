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
	cargo run -- rollback --preview

rollback-preview:
	cargo run -- rollback --preview

rollback-force:
	cargo run -- rollback --force

rollback-force-active:
	cargo run -- rollback --force --allow-active-sessions

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
