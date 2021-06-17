##project info#############################################################
PROJECT=uart
TARGET=aarch64-unknown-none
ver ?= debug
QEMU_DIST=stdio

##files####################################################################
ELF=target/$(TARGET)/$(ver)/$(PROJECT)
IMG=kernel8.img
DUMP=$(PROJECT).dump
SRCS=$(wildcard src/*.rs)

###########################################################################
all: build $(DUMP) 
build : $(IMG)
dump: $(DUMP)
	less $<
check: $(SRC)
	cargo check

###########################################################################
$(ELF): $(SRCS)
	cargo rustc

$(IMG): $(ELF)
	rust-objcopy -O binary $< $@

clean:
	$(RM) src/*~ *~

distclean: clean
	cargo clean
	$(RM) *.lock $(DUMP) $(DOC_LN) $(IMG)


$(DUMP): $(ELF)
	@date > $(PROJECT).dump
	rust-objdump -d $< >> $(PROJECT).dump
