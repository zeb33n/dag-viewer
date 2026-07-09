all: serve

rust:
	cargo build --release --target=wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/dag_viewer.wasm dag_viewer/assets/dag_viewer.wasm

build: rust
	uv build

# Finds the most recently modified .whl file in dist/ and installs it
install: build
	uv pip install $$(ls -t dist/*.whl | head -n 1)

site: install
	uv run zensical build --clean

serve: site
	python -m http.server -d site 8000

