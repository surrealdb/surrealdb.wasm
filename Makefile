.PHONY: default
default:
	@echo "Choose a Makefile target:"
	@$(MAKE) -pRrq -f $(lastword $(MAKEFILE_LIST)) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make data base/ {if ($$1 !~ "^[#.]") {print "  - " $$1}}' | sort

.PHONY: setup
setup:
	cargo update
	cargo upgrade --pinned --to-lockfile

.PHONY: check
check:
	cargo check
	cargo fmt --all -- --check
	cargo clippy -- -W warnings

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean

.PHONY: serve
serve:
	wasm-pack build
	npm run start

.PHONY: build
build:
	wasm-pack build
	npm run build
	wasm-pack pack
