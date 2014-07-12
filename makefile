LIBAUDIOFILE ?= -L$(shell brew --prefix)/opt/audiofile/lib/ #libaudiofile.a
RUSTFLAGS += -C link-args="$(LIBAUDIOFILE)"

test:
	@mkdir -p bin
	rustc $(RUSTFLAGS) --cfg test audiofile.rs -o bin/test
	bin/test

all: bin/printinfo

bin/%: utils/%.rs
	@mkdir -p bin
	rustc $(RUSTFLAGS) $< -o $@
