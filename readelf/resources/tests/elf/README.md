# ELF Test Files

These files are not the complete ELF file, but are the headers of various
real-world formats found. Thus they are kept small and can be submitted directly
with the repository.

They were sourced from various locations from the Internet. Only small portions
of the original file are required for integration testing. Fuzzing and other
tests are done in test code itself. The directory is the name of the source
(usually a downloaded ISO) and the file name remains as the original, so it is
easier to reconstruct from the original without storing large binaries in the
repository.

```sh
head -c64 <source> > ./resources/test/elf/<test>
```
