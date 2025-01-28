.PHONY: check

check:
	cargo clippy -- -W clippy::all -W clippy::pedantic

fix:
	cargo clippy --fix -- -W clippy::all  -W clippy::pedantic

check-all:
	cargo clippy -- -W clippy::all -W clippy::pedantic  -W clippy::nursery -W clippy::cargo -W clippy::restriction