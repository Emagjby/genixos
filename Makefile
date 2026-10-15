.PHONY: run build bootimage
run: bootimage
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-genixos/debug/bootimage-genixos.bin -serial stdio

bootimage:
	cargo bootimage

build:
	cargo build
