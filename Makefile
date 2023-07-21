TARGET ?= debug
RELEASE_FLAG :=
ifeq ($(TARGET),release)
RELEASE_FLAG = --release
endif


WASM_TOOLS_BIN = ./bin/wasm-tools
WASM_TOOLS_PATH ?= $(HOME)/Developer/bca/wasm-tools
WIT_BINDGEN_BIN = ./bin/wit-bindgen
WIT_BINDGEN_PATH ?= $(HOME)/Developer/bca/wit-bindgen


RUST_EXAMPLE = rust_example
GO_EXAMPLE = go_example

all: build-rust build-go

$(WASM_TOOLS_BIN):
	cd $(WASM_TOOLS_PATH) && cargo build --release && cp ./target/release/wasm-tools $(CURDIR)/bin/

$(WIT_BINDGEN_BIN):
	cd $(WIT_BINDGEN_PATH) && cargo build --release && cp ./target/release/wit-bindgen $(CURDIR)/bin/

.PHONY: deps
deps: $(WASM_TOOLS_BIN) $(WIT_BINDGEN_BIN)
	./bin/wasm-tools -V
	./bin/wit-bindgen -V


$(RUST_EXAMPLE).wasm:
	cd rust-example && cargo build --target wasm32-wasi $(RELEASE_FLAG) && mv ./target/wasm32-wasi/$(TARGET)/$@ ../$@

.PHONY: build-rust
build-rust: $(RUST_EXAMPLE).wasm
	$(WIT_BINDGEN_BIN) rust ./rust-example/wit/host.wit --ownership=owning
	$(WASM_TOOLS_BIN) component new $< -o component.$< --adapt ./rust-example/wasi_snapshot_preview1.wasm
	$(WASM_TOOLS_BIN) component wit component.$<

$(GO_EXAMPLE).wasm:
	# ./bin/wit-bindgen tiny-go ./go-example/wit/host.wit --out-dir ./go-example/host
	cd go-example && tinygo build -target=wasi -o $@ main.go && \
	mv $@ ../$@

.PHONY: build-go
build-go: $(GO_EXAMPLE).wasm
	$(WASM_TOOLS_BIN) component embed ./go-example/wit/host.wit $< -o embed.$<
	$(WASM_TOOLS_BIN) component new embed.$< -o component.$< --adapt ./go-example/wasi_snapshot_preview1.wasm
	$(WASM_TOOLS_BIN) component wit component.$<

.PHONY: run
run: build-rust build-go
	cd host && cargo build --release && mv ./target/release/host ../bin/host
	./bin/host ./component.$(RUST_EXAMPLE).wasm
	./bin/host ./component.$(GO_EXAMPLE).wasm


.PHONY: clean
clean:
	rm -rf *.wasm