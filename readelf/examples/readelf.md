# ReadElf Example

## Building the `readelf` Binary

To build the `readelf` binary, which uses the library, and can read ELF files
from the command line. The following example shows the help.

```sh
$ cargo build --example readelf
$ ./target/debug/examples/readelf -?
```

## Usage

The command line options supported are:

- `?` - Show help.
- `h` - Show the ELF header
- `l` - Show the ELF segments
