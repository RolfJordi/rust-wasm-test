init:
	git config core.hooksPath .githooks

format:
	cargo fmt -- --force --write-mode overwrite
