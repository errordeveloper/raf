HOST_OS := $(shell uname -s)

ifeq ($(HOST_OS),Darwin)
LIBCLANG	?= -L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/
LIBAUDIOFILE	?= -L$(shell brew --prefix)/opt/audiofile/lib/ -laudiofile
endif

ifeq ($(HOST_OS),Linux)
LIBCLANG	?= -L/usr/lib/
LIBAUDIOFILE	?= -L/usr/lib/ -laudiofile
endif

RUSTFLAGS	+= -g -C link-args="$(LIBAUDIOFILE)"

test:
	rustc $(RUSTFLAGS) --cfg test audiofile.rs -o ./audiofile_test
	./audiofile_test

regen:
	rm -f afapi.rs
	$(MAKE) afapi.rs

bindgen:
	rustc imports/rust-bindgen/lib.rs -C link-args="$(LIBCLANG)"
	rustc imports/rust-bindgen/bindgen.rs -L . -C link-args="$(LIBCLANG)"

afapi.rs: bindgen
	./$< imports/audiofile/libaudiofile/audiofile.h \
		-I imports/audiofile/libaudiofile/ \
		-match audiofile.h \
		-match aupvlist.h \
		-override-enum-type sint \
		| sed 's/off_t/::libc::off_t/' \
	> $@
