use arm_attr::{enums::*, globals::*, read::Endian, tag::Tag, BuildAttrs};

macro_rules! assert_tag {
    ($attr:expr, $tag:expr) => {
        assert_eq!($attr.next(), Some($tag))
    };
}

#[test]
fn test_tags() {
    #[rustfmt::skip]
    let raw = [
        b'A', // version
        0x9f, 0x00, 0x00, 0x00, // size
        b'a', b'e', b'a', b'b', b'i', 0, // "aeabi" subsection
        Tag_File, // file scope
        0x95, 0x00, 0x00, 0x00, // scope size
        Tag_CPU_raw_name, b'V', b'5', b'T', b'E', 0, // "V5TE"
        Tag_CPU_name, b'A', b'R', b'M', b'9', b'4', b'6', b'E', b'-', b'S', 0, // ARM946E-S
        Tag_CPU_arch, 4, // V5TE
        Tag_CPU_arch_profile, 0, // NotApplicable
        Tag_ARM_ISA_use, 1, // Allowed
        Tag_THUMB_ISA_use, 1, // Allowed16Bit
        Tag_FP_arch, 1, // V1
        Tag_WMMX_arch, 2, // V2
        Tag_Advanced_SIMD_arch, 4, // V8_1A
        Tag_PCS_config, 3, // LinuxDso
        Tag_ABI_PCS_R9_use, 1, // Sb
        Tag_ABI_PCS_RW_data, 2, // SbRel
        Tag_ABI_PCS_RO_data, 2, // None
        Tag_ABI_PCS_GOT_use, 1, // Direct
        Tag_ABI_PCS_wchar_t, 4, // Size4
        Tag_ABI_FP_rounding, 0, // Nearest
        Tag_ABI_FP_denormal, 2, // PreserveSign
        Tag_ABI_FP_exceptions, 1, // CheckInexact
        Tag_ABI_FP_user_exceptions, 1, // Enabled
        Tag_ABI_FP_number_model, 2, // InfNaN
        Tag_ABI_align_needed, 2, // Align4
        Tag_ABI_align_needed, 4, // Align2n(4)
        Tag_ABI_align_needed, 12, // Align2n(12)
        Tag_ABI_align_preserved, 1, // Align8
        Tag_ABI_align_preserved, 4, // Align2n(4)
        Tag_ABI_align_preserved, 12, // Align2n(12)
        Tag_ABI_enum_size, 1, // SmallestSize
        Tag_ABI_HardFP_use, 0, // Implied
        Tag_ABI_VFP_args, 2, // Toolchain
        Tag_ABI_WMMX_args, 0, // Base
        Tag_ABI_optimization_goals, 2, // OptimizeSpeed
        Tag_ABI_FP_optimization_goals, 5, // FavorAccuracy
        Tag_compatibility, 0, // Always
        Tag_compatibility, 1, b'X', b'Y', b'Z', 0, // ByToolchain("XYZ")
        Tag_compatibility, 42, b'f', b'o', b'o', 0, // Private { flag: 42, vendor: "foo" }
        Tag_CPU_unaligned_access, 1, // Allowed
        Tag_FP_HP_extension, 0, // IfExists
        Tag_ABI_FP_16bit_format, 2, // Alternative
        Tag_MPextension_use, 1, // Allowed
        Tag_DIV_use, 1, // None
        Tag_DSP_extension, 0, // IfExists
        Tag_MVE_arch, 2, // IntFloat
        Tag_PAC_extension, 1, // OnlyNopSpace
        Tag_BTI_extension, 2, // Allowed
        Tag_nodefaults, 0,
        Tag_also_compatible_with, Tag_CPU_arch, 22, 0, // Arch(CpuArch::V9A)
        Tag_also_compatible_with, Tag_CPU_name, b'A', b'R', b'M', b'7', b'T', b'D', b'M', b'I', 0, // Reserved(Tag::CpuName("ARM7TDMI"))
        Tag_conformance, b'2', b'0', b'2', b'3', b'Q', b'3', 0, // Conform("2023Q3")
        Tag_T2EE_use, 0, // None
        Tag_Virtualization_use, 2, // VExts
        Tag_FramePointer_use, 1, // WithRecords
        Tag_BTI_use, 1, // Enabled
        Tag_PACRET_use, 1, // Enabled
    ];

    let build_attrs = BuildAttrs::new(&raw, Endian::Little).unwrap();
    let mut subsections = build_attrs.subsections().map(|s| s.unwrap());
    let subsection = subsections.next().unwrap();
    assert!(subsection.is_aeabi());

    let mut attributes = subsection.into_public_tag_iter().unwrap();
    assert_tag!(attributes, (0, Tag::File { end_offset: 149 }));
    assert_tag!(attributes, (5, Tag::CpuRawName("V5TE")));
    assert_tag!(attributes, (11, Tag::CpuName(CpuName::Arm946ES)));
    assert_tag!(attributes, (22, Tag::CpuArch(CpuArch::V5TE)));
    assert_tag!(attributes, (24, Tag::CpuArchProfile(CpuArchProfile::NotApplicable)));
    assert_tag!(attributes, (26, Tag::ArmIsaUse(ArmIsaUse::Allowed)));
    assert_tag!(attributes, (28, Tag::ThumbIsaUse(ThumbIsaUse::Allowed16Bit)));
    assert_tag!(attributes, (30, Tag::FpArch(FpArch::V1)));
    assert_tag!(attributes, (32, Tag::WmmxArch(WmmxArch::V2)));
    assert_tag!(attributes, (34, Tag::AsimdArch(AsimdArch::V8_1A)));
    assert_tag!(attributes, (36, Tag::PcsConfig(PcsConfig::LinuxDso)));
    assert_tag!(attributes, (38, Tag::AbiPcsR9Use(AbiPcsR9Use::Sb)));
    assert_tag!(attributes, (40, Tag::AbiPcsRwData(AbiPcsRwData::SbRel)));
    assert_tag!(attributes, (42, Tag::AbiPcsRoData(AbiPcsRoData::None)));
    assert_tag!(attributes, (44, Tag::AbiPcsGotUse(AbiPcsGotUse::Direct)));
    assert_tag!(attributes, (46, Tag::AbiPcsWcharT(AbiPcsWcharT::Size4)));
    assert_tag!(attributes, (48, Tag::AbiFpRounding(AbiFpRounding::Nearest)));
    assert_tag!(attributes, (50, Tag::AbiFpDenormal(AbiFpDenormal::PreserveSign)));
    assert_tag!(attributes, (52, Tag::AbiFpExceptions(AbiFpExceptions::CheckInexact)));
    assert_tag!(attributes, (54, Tag::AbiFpUserExceptions(AbiFpUserExceptions::Enabled)));
    assert_tag!(attributes, (56, Tag::AbiFpNumberModel(AbiFpNumberModel::InfNaN)));
    assert_tag!(attributes, (58, Tag::AbiAlignNeeded(AbiAlignNeeded::Align4)));
    assert_tag!(attributes, (60, Tag::AbiAlignNeeded(AbiAlignNeeded::Align2n(4))));
    assert_tag!(attributes, (62, Tag::AbiAlignNeeded(AbiAlignNeeded::Align2n(12))));
    assert_tag!(attributes, (64, Tag::AbiAlignPreserved(AbiAlignPreserved::Align8)));
    assert_tag!(attributes, (66, Tag::AbiAlignPreserved(AbiAlignPreserved::Align2n(4))));
    assert_tag!(attributes, (68, Tag::AbiAlignPreserved(AbiAlignPreserved::Align2n(12))));
    assert_tag!(attributes, (70, Tag::AbiEnumSize(AbiEnumSize::SmallestSize)));
    assert_tag!(attributes, (72, Tag::AbiHardFpUse(AbiHardFpUse::Implied)));
    assert_tag!(attributes, (74, Tag::AbiVfpArgs(AbiVfpArgs::Toolchain)));
    assert_tag!(attributes, (76, Tag::AbiWmmxArgs(AbiWmmxArgs::Base)));
    assert_tag!(attributes, (78, Tag::AbiOptGoals(AbiOptGoals::OptimizeSpeed)));
    assert_tag!(attributes, (80, Tag::AbiFpOptGoals(AbiFpOptGoals::FavorAccuracy)));
    assert_tag!(attributes, (82, Tag::Compat(Compat::Always)));
    assert_tag!(attributes, (84, Tag::Compat(Compat::ByToolchain("XYZ"))));
    assert_tag!(attributes, (90, Tag::Compat(Compat::Private { flag: 42, vendor: "foo" })));
    assert_tag!(attributes, (96, Tag::CpuUnalignedAccess(CpuUnalignedAccess::Allowed)));
    assert_tag!(attributes, (98, Tag::FpHpExt(FpHpExt::IfExists)));
    assert_tag!(attributes, (100, Tag::AbiFp16BitFormat(AbiFp16BitFormat::Alternative)));
    assert_tag!(attributes, (102, Tag::MpExtUse(MpExtUse::Allowed)));
    assert_tag!(attributes, (104, Tag::DivUse(DivUse::None)));
    assert_tag!(attributes, (106, Tag::DspExt(DspExt::IfExists)));
    assert_tag!(attributes, (108, Tag::MveArch(MveArch::IntFloat)));
    assert_tag!(attributes, (110, Tag::PacExt(PacExt::OnlyNopSpace)));
    assert_tag!(attributes, (112, Tag::BtiExt(BtiExt::Allowed)));
    assert_tag!(attributes, (114, Tag::NoDefaults));
    assert_tag!(attributes, (116, Tag::AlsoCompatWith(AlsoCompatWith::Arch(CpuArch::V9A))));
    assert_tag!(
        attributes,
        (
            120,
            Tag::AlsoCompatWith(AlsoCompatWith::Reserved(Box::new(Tag::CpuName(CpuName::Arm7Tdmi))))
        )
    );
    assert_tag!(attributes, (131, Tag::Conform(Conform::V2023Q3)));
    assert_tag!(attributes, (139, Tag::T2EeUse(T2EeUse::None)));
    assert_tag!(attributes, (141, Tag::VirtualUse(VirtualUse::VExts)));
    assert_tag!(attributes, (143, Tag::FramePointerUse(FramePointerUse::WithRecords)));
    assert_tag!(attributes, (145, Tag::BtiUse(BtiUse::Enabled)));
    assert_tag!(attributes, (147, Tag::PacretUse(PacretUse::Enabled)));
}
