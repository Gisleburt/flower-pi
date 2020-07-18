.PHONY: build deploy run

flower-binary = target/arm-unknown-linux-musleabi/debug/flower

$(flower-binary): $(wildcard src/* Cargo.*)
	docker run --rm -it -v "$(PWD)":/home/rust/src messense/rust-musl-cross:arm-musleabi cargo build

build: $(flower-binary)

deploy: build
	scp $(flower-binary) pi@piflower.local:/home/pi

run: deploy
	ssh pi@piflower.local '~/flower'
