# 1. Integration Tests

## 1.1. ELF Headers

There are a large number of ELF header binaries in the folder
`resources/tests/elf`. These intentionally only contain the first 64-bytes for
testing. Placing the full binary may be problematic:

- Binaries in GIT are not ideal;
- avoid copyright information

Instead, see the sources from the folder name. I downloaded the images and
extracted the files for testing.

## 1.2. ELF Builder for Integration Tests

The `tests/common/builder.rs` module is a simpler builder for ELF files. Having
a builder makes it easier to cover non-common use use cases by mocking our own
ELF files and also injecting errors.

The structure of the fictitious ELF file is determined upfront to make it
simpler to implement.

```text
                                 Offset (32 / 64-bit)
+------------------------------+
| ELF Header (52 or 64 bytes)  |  0x0000
+------------------------------+
| Array of Segments            |  0x0034 (32-bit) / 0x0040 (64-bit)
+------------------------------+
| Array of Sections            |  0x0400 - 0x07FF
+------------------------------+
| Data for Segments / Sections |  0x0800 - 0x1FFF
+------------------------------+
```

This way, we use 4096 byte preallocated array upfront. It's easy to add data,
segments and sections.
