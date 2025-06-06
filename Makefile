.PHONY: clean tests_plaintext tests_impl_0

CC := gcc
PLAINTEXT_IMPL = plaintext_impl
IMPL_0 = implementation_0_tfhe_rs

build/half_64b_mul_plaintext_1:\
	${PLAINTEXT_IMPL}/src/include/mul_plaintext.h\
	${PLAINTEXT_IMPL}/src/lib/mul_plaintext.c\
	${PLAINTEXT_IMPL}/tests/half_64b_mul_plaintext_1.c
	mkdir -p build
	$(CC) ${PLAINTEXT_IMPL}/src/lib/mul_plaintext.c\
	      ${PLAINTEXT_IMPL}/tests/half_64b_mul_plaintext_1.c\
		  -o $@\
		  -I${PLAINTEXT_IMPL}/src/include

build/full_64b_mul_plaintext_1:\
	${PLAINTEXT_IMPL}/src/include/mul_plaintext.h\
	${PLAINTEXT_IMPL}/src/lib/mul_plaintext.c\
	${PLAINTEXT_IMPL}/tests/full_64b_mul_plaintext_1.c
	mkdir -p build
	$(CC) ${PLAINTEXT_IMPL}/src/lib/mul_plaintext.c\
	      ${PLAINTEXT_IMPL}/tests/full_64b_mul_plaintext_1.c\
		  -o $@\
		  -I${PLAINTEXT_IMPL}/src/include

tests_plaintext: build/half_64b_mul_plaintext_1 build/full_64b_mul_plaintext_1
	cp -ru ${PLAINTEXT_IMPL}/tests/data ./build/
	cd build && ./half_64b_mul_plaintext_1
	cd build && ./full_64b_mul_plaintext_1

tests_impl_0:
	cd ${IMPL_0} && cargo build --release && ./target/release/${IMPL_0}

clean: 
	rm -rf build
	cd ${IMPL_0} && cargo clean
	rm -f ${IMPL_0}/Cargo.lock
