all: python site serve

rust:
	cargo build --release --target=wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/dag_viewer.wasm dag_viewer/assets/dag_viewer.wasm

python:
	pip install .

site: python
	zensical build --clean

serve: site
	python -m http.server -d site 8000

