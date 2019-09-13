all:
	# this build a debug library; we will need to change to
	#		`cargo build --release`
	# for deployment
	cargo build
	# we use cbindgen crate to automatically generate the header for C
	cbindgen --config cbindgen.toml --crate bls_sigs_ref-rs --output c_wrapper/bls_c.h
	# this is an example of how the APIs can be called in c
	# for released version of the library, change into
	# 	`gcc c_wrapper/*.c -L./target/release -lpixel -lpthread -ldl -o c_wrapper/c_example`

test:
	cargo build
	cbindgen --config cbindgen.toml --crate bls_sigs_ref-rs --output c_wrapper/bls_c.h
	gcc c_wrapper/*.c -L./target/debug -lbls_sigs_ref_rs -lpthread -ldl -o c_wrapper/c_example
	c_wrapper/c_example


clean:
	cargo clean
