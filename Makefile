build:
	LOG=trace cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
boot:
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios ./bootloader/rustsbi-qemu.bin \
	-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

clean:
	@cargo clean



debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

.PHONY: build env kernel clean disasm disasm-vim run-inner switch-check
