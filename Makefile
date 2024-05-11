build_dir = build/x86_64-unknown-none
iso = target/os.iso
grub_cfg = grub_config/grub.cfg
linker_script = scripts/linker.ld
rust_out_file = target/x86_64-unknown-none/debug/librustos.a
kernel = target/x86_64-unknown-none/debug/rustos.bin
assembly_source_files := $(wildcard src/bootloader/*.s)
assembly_object_files := $(patsubst src/bootloader/%.s, \
	$(build_dir)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p $(build_dir)/boot/grub
	@cp $(kernel) $(build_dir)/boot/kernel.bin
	@cp $(grub_cfg) $(build_dir)/boot/grub
	@grub-mkrescue -o $(iso) $(build_dir)
	@rm -r $(build_dir)

$(kernel): $(assembly_object_files) $(linker_script) kernel $(rust_out_file)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_out_file)

kernel:
	@cargo build

# compile assembly files
$(build_dir)/%.o: src/bootloader/%.s
	@mkdir -p build/x86_64-unknown-none
	@nasm -felf64 $< -o $@