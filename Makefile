RUSTC?=rustc
NASM?=nasm
LD?=ld

RUSTC_FLAGS?=
KERNEL_RUSTC_FLAGS?=-L bin
NASM_FLAGS?=
LD_FLAGS?=

# Possible Arches:
# x86
# x86_64
ARCH?=x86

ifeq ($(ARCH), x86)
TARGET=i686-unknown-linux-gnu
RUSTC_FLAGS += --target $(TARGET) -L rustlibdir
NASM_FLAGS += -f elf32
LD_FLAGS += -m elf_i386
QEMU=qemu-system-i386
else ifeq ($(ARCH), x86_64)
TARGET=x86_64-unknown-linux-gnu
RUSTC_FLAGS += --target $(TARGET)
NASM_FLAGS += -f elf64
LD_FLAGS += -m elf_x86_64
QEMU=qemu-system-x86_64
endif

# Recursive Wildcard Function
rwildcard=$(wildcard $1$2) $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2))

ARCH_DEPENDENCIES := $(call rwildcard,arch/$(ARCH)/,*.rs)
KERNEL_DEPENDENCIES := $(call rwildcard,kernel/,*.rs)
RUST_DEPENDENCIES := $(ARCH_DEPENDENCIES) $(KERNEL_DEPENDENCIES) bin/librlibc.rlib
ASSEMBLIES := $(patsubst %.asm, %.o, $(call rwildcard,arch/$(ARCH)/,*.asm))
RUSTLIB := bin/libkernel.a
DEBUG_BIN := bin/kernel.elf
RELEASE_BIN := bin/kernel.bin
BINARY := $(DEBUG_BIN)

all: debug

debug: $(DEBUG_BIN)
debug: RUSTC_FLAGS += -g
debug: BINARY := DEBUG_BIN

release: $(RELEASE_BIN)
release: RUSTC_FLAGS += -O
release: KERNEL_RUSTC_FLAGS += -C lto
release: LD_FLAGS += -S
release: BINARY := RELEASE_BIN

.PHONY: run
run: $(DEBUG_BIN)
	$(QEMU) -curses -kernel $<

.PHONY: release-run
release-run: $(RELEASE_BIN)
	$(QEMU) -curses -kernel $<

.PHONY: clean
clean:
	$(RM) $(DEBUG_BIN) $(RELEASE_BIN) *.o $(ASSEMBLIES) $(RUSTLIB) bin/librlibc.rlib bin/*.deflate

$(ASSEMBLIES): %.o : %.asm
	$(NASM) $(NASM_FLAGS) -o $@ $<

$(RUSTLIB): kernel.rs $(RUST_DEPENDENCIES) bin/librlibc.rlib
	$(RUSTC) $(RUSTC_FLAGS) $(KERNEL_RUSTC_FLAGS) $< --out-dir=bin

$(RELEASE_BIN): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) $(LD_FLAGS) -T link.ld -o $@ $^

$(DEBUG_BIN): $(ASSEMBLIES) $(RUSTLIB)
	$(LD) $(LD_FLAGS) -T link.ld -o $@ $^

bin/librlibc.rlib: rlibc/src/lib.rs
	$(RUSTC) $(RUSTC_FLAGS) --out-dir=bin --crate-type=rlib --crate-name=rlibc $(RUSTC_OPTIONS) $<
