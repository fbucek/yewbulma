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
build: 
	wasm-pack build --target web --out-name wasm --out-dir wasm/
test:
	wasm-pack test --chrome --headless;


watch: 
	cargo watch -i "pkg/**" -s "wasm-pack build";

wtest:
	cargo watch -i "pkg/**" -s "wasm-pack test --chrome --headless";

wtable:
	cd examples/table; trunk serve --release

publish: all
	# not published yet
	cargo publish
