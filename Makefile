
build:
	cargo build --release

run: 
	cargo run ./test.lox

test: build
	cargo test
	python3 ./tests/tester.py ./target/release/rs_interpreter ./tests/



# # build the wasm-build:
# web: 
# #   emcc -O3 $(WEBFILES) -o build_wasm/index.html --shell-file srcweb/shell_minimal.html -s NO_EXIT_RUNTIME=1 -s "EXPORTED_RUNTIME_METHODS=['ccall']"
# 	cp srcweb/*.js build_wasm/
# 	cp srcweb/*.css build_wasm/
# #	emcc $(WEBFILES) -o build_wasm/compiler.html -sEXPORTED_FUNCTIONS=_compileSourceCode -sEXPORTED_RUNTIME_METHODS=ccall,cwrap


# # to remove all artifacts/binary
# clean:
# 	rm build_wasm/*.html
# 	rm build_wasm/*.js
# 	rm build_wasm/*.css
# 	rm build_wasm/*.wasm

