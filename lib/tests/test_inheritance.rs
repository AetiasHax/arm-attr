use arm_attr::{enums::*, globals::*, read::Endian, BuildAttrs};

#[test]
fn test_inheritance() {
    #[rustfmt::skip]
    let raw = [
        b'A', // version
        0x3e, 0x00, 0x00, 0x00, // size
        b'a', b'e', b'a', b'b', b'i', 0, // "aeabi" subsection
        Tag_File, 0x34, 0x00, 0x00, 0x00, // whole file
            Tag_CPU_arch, 4, // V5TE
            Tag_Section, 0x13, 0x00, 0x00, 0x00, 53, 42, 0, // sections 53 and 42
                Tag_Symbol, 0x09, 0x00, 0x00, 0x00, 99, 0, // symbol 99
                    Tag_THUMB_ISA_use, 1, // Allowed16Bit
                Tag_FP_arch, 1, // V1
            Tag_Section, 0x1a, 0x00, 0x00, 0x00, 1, 0, // section 1
                Tag_Symbol, 0x08, 0x00, 0x00, 0x00, 77, 66, 0, // symbols 77 and 66
                Tag_Symbol, 0x09, 0x00, 0x00, 0x00, 88, 0, // symbol 88
                    Tag_THUMB_ISA_use, 1, // Allowed16Bit
                Tag_nodefaults, 0,
    ];

    let build_attrs = BuildAttrs::new(&raw, Endian::Little).unwrap();
    let mut subsections = build_attrs.subsections().map(|s| s.unwrap());
    let subsection = subsections.next().unwrap();
    assert!(subsection.is_aeabi());

    let file = subsection.into_public_attributes().unwrap();
    assert_eq!(file.attributes.cpu_arch, Some(CpuArch::V5TE));

    let section = file.sections.get([53, 42].as_ref()).unwrap();
    assert_eq!(section.attributes.cpu_arch, None);
    assert_eq!(section.attributes.thumb_isa_use, None);

    let symbol = section.symbols.get([99].as_ref()).unwrap();
    assert_eq!(symbol.attributes.cpu_arch, None);
    assert_eq!(symbol.attributes.thumb_isa_use, Some(ThumbIsaUse::Allowed16Bit));

    let section = file.sections.get([1].as_ref()).unwrap();
    assert_eq!(section.attributes.cpu_arch, Some(CpuArch::V5TE));

    let symbol = section.symbols.get([77, 66].as_ref()).unwrap();
    assert_eq!(symbol.attributes.cpu_arch, None);

    let symbol = section.symbols.get([88].as_ref()).unwrap();
    assert_eq!(symbol.attributes.cpu_arch, None);
    assert_eq!(symbol.attributes.thumb_isa_use, Some(ThumbIsaUse::Allowed16Bit));
}
