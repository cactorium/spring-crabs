build: spring-crabs.js

spring-crabs.js: target/asmjs-unknown-emscripten/release/spring-crabs.js
	cp target/asmjs-unknown-emscripten/release/spring-crabs.js .

target/asmjs-unknown-emscripten/release/spring-crabs.js:
	cargo build --target asmjs-unknown-emscripten --release -v
