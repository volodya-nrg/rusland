PROJECT_NAME=rusland
GIT_COMMIT=$(shell git rev-parse --short HEAD)

.PHONY: echo_version
echo_version:
	@echo commit:$(GIT_COMMIT)

install_deps:
	cargo install sqlx-cli

migration_run:
	sqlx migrate run --database-url "sqlite:./data/db.sqlite3"
