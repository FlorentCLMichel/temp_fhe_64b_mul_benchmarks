.PHONY: clean

CC := gcc

build/half_64b_mul_plaintext_1: src/include/mul_plaintext.h src/lib/mul_plaintext.c tests/half_64b_mul_plaintext_1.c
	mkdir -p build
	$(CC) src/lib/mul_plaintext.c tests/half_64b_mul_plaintext_1.c -o $@ -Isrc/include

build/full_64b_mul_plaintext_1: src/include/mul_plaintext.h src/lib/mul_plaintext.c tests/full_64b_mul_plaintext_1.c
	mkdir -p build
	$(CC) src/lib/mul_plaintext.c tests/full_64b_mul_plaintext_1.c -o $@ -Isrc/include

clean: 
	rm -rf build
