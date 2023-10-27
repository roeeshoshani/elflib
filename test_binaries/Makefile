CFLAGS = -fPIC -pie

CCS = gcc i686-linux-gnu-gcc mips64-linux-gnuabi64-gcc

define newline


endef

.phony: all mains
all: mains
mains: main.c
	mkdir -p build
	$(foreach cc,$(CCS),$(cc) $(CFLAGS) main.c -o build/$(cc)-main $(newline))

clean:
	rm -r build
