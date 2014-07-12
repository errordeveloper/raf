LIBCLANG	?= "-L/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/"

LIBAUDIOFILE	?= -L$(shell brew --prefix)/opt/audiofile/lib/ -laudiofile

RUSTFLAGS	+= -C link-args="$(LIBAUDIOFILE)"

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
		| sed 's/off_t/::libc::off_t/' \
		| sed 's/\(^pub static .*: ::libc::c_\)u\(int = [0-9]*;$$\)/\1\2 \/\*_patched_\*\//' \
	> $@
