
# Yew Bulma 

Components for Yew in [Bulma CSS](https://bulma.io)

[![build](https://github.com/fbucek/yewbulma/workflows/build/badge.svg)](https://github.com/fbucek/yewbulma/)

## Development

Using examples and watch

1. `cargo watch -s "examples/run.sh table"` or `cargo watch -s "examples/run.sh table-wb` ( wb -> wasm-bindgen )
    * Run server e.g. using serving `example/static` folder
    * using `python3 -m http.server 8000 --directory examples/static`
    * or VSCode extension: `Live Server`
2. `./run.sh watchexample table` ( table is example subproject )

Using wasm-pack build 

* `cargo watch -i "pkg/**" -s "wasm-pack build"`

## Tests

Just prepared to run tests

* `cargo watch -i "pkg/**" -s "wasm-pack test --headless --chrome"`
