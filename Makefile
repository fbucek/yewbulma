.PHONY: all watch watchtest wtable

all: clean check test build doc

check:
	cargo check
	cargo clippy
	cargo fmt

doc:
	cargo doc --no-deps --document-private-items --open

clean: 
	cargo clean --doc

test:
	wasm-pack test --chrome --headless;


watch: 
	cargo watch -i "pkg/**" -s "wasm-pack build";

wtest:
	cargo watch -i "pkg/**" -s "wasm-pack test --chrome --headless";

wtable:
	cd examples/table; trunk serve --release

publish: all
	cd aprun && cargo publish
