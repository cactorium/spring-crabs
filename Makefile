build: spring_crabs.wasm

test:
	cargo test

spring_crabs.wasm: target/wasm32-unknown-unknown/release/spring_crabs.wasm
	wasm-gc target/wasm32-unknown-unknown/release/spring_crabs.wasm -o spring_crabs.wasm

target/wasm32-unknown-unknown/release/spring_crabs.wasm: $(wildcard src/*.rs)
	cargo build --target wasm32-unknown-unknown --release -v

clean:
	rm target/asmjs-unknown-emscripten/release/spring_crabs.wasm
