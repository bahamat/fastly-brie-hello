export TIMESTAMP := $(shell date -u "+%Y%m%dT%H%M%SZ")

.PHONY: all clean serve publish

all: target/wasm32-wasip1/debug/fastly-compute-project.wasm

target/wasm32-wasip1/debug/fastly-compute-project.wasm: src/main.rs
	cargo build

serve: target/wasm32-wasip1/debug/fastly-compute-project.wasm
	fastly compute serve

publish: src/main.rs
	fastly compute publish

clean:
	rm -rf target
