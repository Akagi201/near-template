.PHONY: all
all: help

.PHONY: accounts # Create deploy accounts
accounts:
	./scripts/create_accounts.sh

.PHONY: build # Build all contracts and copy wasm files to res/
build:
	raen build --workspace --release --standards
	cp target/res/*.wasm res/

.PHONY: deploy # Deploy all contract
deploy:
	./scripts/deploy.sh

.PHONY: delete_all # Delete all accounts
delete_all:
	./scripts/delete_all.sh

.PHONY: test # Run unit tests
test:
	cargo test -- --nocapture

.PHONY: clean # Clean build files
clean:
	cargo clean
	rm -rf res/*.wasm

.PHONY: help # Generate list of targets with descriptions
help:
	@grep '^.PHONY: .* #' Makefile | sed 's/\.PHONY: \(.*\) # \(.*\)/\1	\2/' | expand -t20