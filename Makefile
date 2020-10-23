.PHONY: all watch watchtest wtable

watch: 
	cargo watch -i "pkg/**" -s "wasm-pack build";

wtest:
	cargo watch -i "pkg/**" -s "wasm-pack test --chrome --headless";

wtable:
	# https://github.com/TheWaWaR/simple-http-server
	# python3 -m http.server 8000 --directory examples/static &
	cd examples/table; trunk serve --release
	# cargo watch  -i examples/static -s 'examples/run.sh table && simple-http-server -i --nocache --cors examples/static'
