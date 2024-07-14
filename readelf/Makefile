include ../Makefile.mk
LIB = readelf

all: $(LIB)

release: readelf readelf_release

clean: readelf_clean

cov: readelf_cov

readelf: readelf_format readelf_build readelf_lint readelf_test
	@cargo build --example readelf

# Template. Must be at the end, so it doesn't define rules in precedence to
# `all`.
$(eval $(call CRATE_template,$(LIB)))
