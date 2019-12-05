
.PHONY: go-wasm
go-wasm:
	GOOS=js GOARCH=wasm go build -o ./go-wasm/go.wasm ./go-wasm/main.go

.PHONY: tinygo
tinygo:
	cd tinygo && tinygo build -o handler.wasm -target wasm ./main.go

rust-clean:
	rm -rf rust/echo.wasm
	rm -rf rust/modify.wasm
	rm -rf rust/modify_header.wasm
	rm -rf rust/modify_raw_query.wasm

.PHONY: rust
rust: rust-clean
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib echo.rs -o echo.wasm
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib modify.rs -o modify.wasm
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib modify_header.rs -o modify_header.wasm
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib modify_raw_query.rs -o modify_raw_query.wasm
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib sleep.rs -o sleep.wasm
	cd rust && rustc --target wasm32-unknown-unknown -O --crate-type=cdylib transform_to_slack.rs -o transform_to_slack.wasm