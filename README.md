# arm-attr

Parses ARM build attributes from ELF files according to [the 2023Q3 ARM ABI](https://github.com/ARM-software/abi-aa/blob/5c0a393f50e083ccca9f32ca94995211141fe858/addenda32/addenda32.rst#addendum-build-attributes).

## Contents

- [Examples](Examples)
  - [By iterator](#by-iterator)
  - [By struct](#by-struct)

## Examples

The two examples below show two different methods to read build attributes: iterator (lazy and fast) and struct (eager and slow but
correct).

### By iterator

This first example reads the tags directly. It's faster, but:

- It doesn't consider that one type of tag may appear multiple times.
- It doesn't handle enclosing scopes, i.e. section or symbol-specific attributes.
- If a tag could not be parsed, the iterator will stop with no error.

```rust
use arm_attr::{read::Endian, tag::Tag, BuildAttrs};

let data = [/* byte contents of .ARM.attributes */];
let build_attrs = BuildAttrs::new(&data, Endian::Little).unwrap();
for subsection in build_attrs.subsections() {
    let subsection = subsection.unwrap();
    if subsection.is_aeabi() {
        for (_, tag) in subsection.into_public_tag_iter().unwrap() {
            match tag {
                Tag::CpuName(name) => println!("CPU name: {name}"),
                _ => {}
            }
        }
    }
}
```

### By struct

This second example collects all tags using `into_public_attributes`. It's slower but doesn't suffer from the flaws mentioned
in the first example.

```rust
let data = [/* byte contents of .ARM.attributes */];
let build_attrs = BuildAttrs::new(&data, Endian::Little).unwrap();
for subsection in build_attrs.subsections() {
    let subsection = subsection.unwrap();
    if subsection.is_aeabi() {
        let file = subsection.into_public_attributes().unwrap();
        if let Some(name) = file.attributes.cpu_name {
            println!("CPU name: {name}");
        }
        for (sections, section) in file.sections {
            if let Some(name) = section.attributes.cpu_name {
                println!("CPU name in sections {sections:?}: {name}");
            }
            for (symbols, symbol) in section.symbols {
                if let Some(name) = symbol.attributes.cpu_name {
                    println!("CPU name in symbols {symbols:?} of sections {sections:?}: {name}");
                }
            }
        }
    }
}
```
