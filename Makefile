RUSTC?=rustc
CARGO?=cargo
NASM?=nasm
LD?=ld
QEMU?=qemu-system-i386

ARCH_DEPENDENCIES=$(wildcard arch/x86/*/*.rs)
KERNEL_DEPENDENCIES=$(wildcard kernel/*.rs) $(wildcard kernel/*/*.rs)
RUST_DEPENDENCIES=$(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES) bin/librlibc.rlib
ASSEMBLIES=$(patsubst %.asm, %.o, $(wildcard arch/x86/asm/*.asm))
TARGET=i686-unknown-linux-gnu
RUSTLIB=bin/libkernel.a
BINARY=bin/kernel.bin
DEBUG=bin/kernel.sym
RUSTC_OPTIONS=--target $(TARGET)

all: $(BINARY)

debug: RUSTC += -g
debug: QEMU += -s -S
debug: $(BINARY) $(DEBUG)

release: RUSTC += -O
release: LD += -S
release: $(BINARY)

.PHONY: run
run: $(BINARY)
	$(QEMU) -kernel $<

.PHONY: clean
clean:
	$(RM) $(BINARY) $(DEBUG) *.o $(ASSEMBLIES) $(RUSTLIB) bin/librlibc.rlib bin/kernel.elf

$(ASSEMBLIES): %.o : %.asm
	$(NASM) -f elf32 -o $@ $<

$(RUSTLIB): kernel_x86.rs $(RUST_DEPENDENCIES) bin/librlibc.rlib
	$(RUSTC) -L rustlibdir -L bin $(RUSTC_OPTIONS) $< --out-dir=bin

$(BINARY): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) --gc-sections -m elf_i386 -T link.ld -o $@ $^

$(DEBUG): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) --gc-sections -m elf_i386 -T link.ld -o bin/kernel.elf $^
	objcopy --only-keep-debug bin/kernel.elf $@

bin/librlibc.rlib: rlibc/src/lib.rs
	$(RUSTC) -L rustlibdir --out-dir=bin --crate-type=rlib --crate-name=rlibc $(RUSTC_OPTIONS) $<
