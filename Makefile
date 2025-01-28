.PHONY: check

check:
	cargo clippy -- -W clippy::all -W clippy::pedantic

fix:
	cargo clippy --fix -- -W clippy::all  -W clippy::pedantic
