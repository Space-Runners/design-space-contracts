
TARGET=./target/deploy/space_runners.so

.PHONY: all

all: build

build: 
	cargo build-bpf && ls -lh ${TARGET}

deploy: build
	echo && solana program deploy ${TARGET}

