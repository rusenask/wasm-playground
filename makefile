
.PHONY: go-wasm
go-wasm:
	GOOS=js GOARCH=wasm go build -o ./go-wasm/go.wasm ./go-wasm/main.go

.PHONY: tinygo
tinygo:
	cd tinygo && tinygo build -o handler.wasm -target wasm ./main.go

rust-clean:
	rm -rf rust/echo.wasm

.PHONY: rust
rust: rust-clean
	# cd rust/echo-wasm && cargo build --release
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib echo.rs -o echo.wasm