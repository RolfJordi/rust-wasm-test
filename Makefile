init:
	git config core.hooksPath .githooks

format:
	cargo fmt -- --write-mode overwrite
