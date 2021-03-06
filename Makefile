.PHONY: build deploy run clippy

flower-binary = target/arm-unknown-linux-musleabi/debug/flower
registry-cache = "$(PWD)/target/.registry"

$(flower-binary): $(wildcard src/* Cargo.*)
	mkdir -p "$(registry-cache)"
	docker run --rm -it -v "$(registry-cache)":/root/.cargo/registry -v "$(PWD)":/home/rust/src messense/rust-musl-cross:arm-musleabi cargo build

build: $(flower-binary)

deploy: build
	scp $(flower-binary) pi@piflower.local:/home/pi

run: deploy
	ssh -t -t pi@piflower.local '~/flower'

clippy:
	mkdir -p "$(registry-cache)"
	docker run --rm -it -v "$(registry-cache)":/root/.cargo/registry -v "$(PWD)":/home/rust/src messense/rust-musl-cross:arm-musleabi cargo clippy
