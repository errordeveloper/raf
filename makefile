LIBAUDIOFILE ?= -L$(shell brew --prefix)/opt/audiofile/lib/ #libaudiofile.a
RUSTFLAGS += -C link-args=$(LIBAUDIOFILE)

all: bin/printinfo

bin/%: utils/%.rs
	@mkdir -p bin
	rustc $(RUSTFLAGS) $< -o $@
