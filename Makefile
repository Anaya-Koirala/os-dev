all:
	cargo build
	cargo bootimage
	mv target/x86_64-compiler-config/debug/bootimage-os.bin bin/
	cp bin/bootimage-os.bin bin/os.bin
	rm bin/bootimage-os.bin

run:
	qemu-system-x86_64 -drive format=raw,file=bin/os.bin

clean:
	cargo clean
	rm -rf bin/*
