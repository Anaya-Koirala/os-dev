make:src/*
	cargo build
	cargo bootimage
	mv target/os/debug/bootimage-os-dev.bin bin/os.bin
run:
	qemu-system-x86_64 -drive format=raw,file=bin/os.bin
	
clean:
	rm -rf bin/*
	make clean

