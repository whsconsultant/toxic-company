.PHONY: run test fmt pages

run:
	cargo run --release --features server -- 0.0.0.0:7420

pages:
	chmod +x scripts/export-pages.sh
	./scripts/export-pages.sh

test:
	cargo test

fmt:
	cargo fmt
