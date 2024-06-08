use arm_attr::{enums::*, globals::*, read::Endian, tag::Tag, BuildAttrs};

macro_rules! assert_tag {
    ($attr:expr, $tag:expr) => {
        assert_eq!($attr.next().unwrap().unwrap(), $tag)
    };
}

#[test]
fn test_tags() {
    #[rustfmt::skip]
    let raw = [
        b'A', // version
        0xa0, 0x00, 0x00, 0x00, // size
        b'a', b'e', b'a', b'b', b'i', 0, // "aeabi" subsection
        Tag_File, // file scope
        0x96, 0x00, 0x00, 0x00, // scope size
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
        Tag_compatibility, 0, 0, // Always
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

    let mut attributes = subsection.into_public_attr_iter().unwrap();
    assert_tag!(attributes, Tag::File { size: 0x96 });
    assert_tag!(attributes, Tag::CpuRawName("V5TE"));
    assert_tag!(attributes, Tag::CpuName(CpuName::Arm946ES));
    assert_tag!(attributes, Tag::CpuArch(CpuArch::V5TE));
    assert_tag!(attributes, Tag::CpuArchProfile(CpuArchProfile::NotApplicable));
    assert_tag!(attributes, Tag::ArmIsaUse(ArmIsaUse::Allowed));
    assert_tag!(attributes, Tag::ThumbIsaUse(ThumbIsaUse::Allowed16Bit));
    assert_tag!(attributes, Tag::FpArch(FpArch::V1));
    assert_tag!(attributes, Tag::WmmxArch(WmmxArch::V2));
    assert_tag!(attributes, Tag::AsimdArch(AsimdArch::V8_1A));
    assert_tag!(attributes, Tag::PcsConfig(PcsConfig::LinuxDso));
    assert_tag!(attributes, Tag::AbiPcsR9Use(AbiPcsR9Use::Sb));
    assert_tag!(attributes, Tag::AbiPcsRwData(AbiPcsRwData::SbRel));
    assert_tag!(attributes, Tag::AbiPcsRoData(AbiPcsRoData::None));
    assert_tag!(attributes, Tag::AbiPcsGotUse(AbiPcsGotUse::Direct));
    assert_tag!(attributes, Tag::AbiPcsWcharT(AbiPcsWcharT::Size4));
    assert_tag!(attributes, Tag::AbiFpRounding(AbiFpRounding::Nearest));
    assert_tag!(attributes, Tag::AbiFpDenormal(AbiFpDenormal::PreserveSign));
    assert_tag!(attributes, Tag::AbiFpExceptions(AbiFpExceptions::CheckInexact));
    assert_tag!(attributes, Tag::AbiFpUserExceptions(AbiFpUserExceptions::Enabled));
    assert_tag!(attributes, Tag::AbiFpNumberModel(AbiFpNumberModel::InfNaN));
    assert_tag!(attributes, Tag::AbiAlignNeeded(AbiAlignNeeded::Align4));
    assert_tag!(attributes, Tag::AbiAlignNeeded(AbiAlignNeeded::Align2n(4)));
    assert_tag!(attributes, Tag::AbiAlignNeeded(AbiAlignNeeded::Align2n(12)));
    assert_tag!(attributes, Tag::AbiAlignPreserved(AbiAlignPreserved::Align8));
    assert_tag!(attributes, Tag::AbiAlignPreserved(AbiAlignPreserved::Align2n(4)));
    assert_tag!(attributes, Tag::AbiAlignPreserved(AbiAlignPreserved::Align2n(12)));
    assert_tag!(attributes, Tag::AbiEnumSize(AbiEnumSize::SmallestSize));
    assert_tag!(attributes, Tag::AbiHardFpUse(AbiHardFpUse::Implied));
    assert_tag!(attributes, Tag::AbiVfpArgs(AbiVfpArgs::Toolchain));
    assert_tag!(attributes, Tag::AbiWmmxArgs(AbiWmmxArgs::Base));
    assert_tag!(attributes, Tag::AbiOptGoals(AbiOptGoals::OptimizeSpeed));
    assert_tag!(attributes, Tag::AbiFpOptGoals(AbiFpOptGoals::FavorAccuracy));
    assert_tag!(attributes, Tag::Compat(Compat::Always));
    assert_tag!(attributes, Tag::Compat(Compat::ByToolchain("XYZ")));
    assert_tag!(attributes, Tag::Compat(Compat::Private { flag: 42, vendor: "foo" }));
    assert_tag!(attributes, Tag::CpuUnalignedAccess(CpuUnalignedAccess::Allowed));
    assert_tag!(attributes, Tag::FpHpExt(FpHpExt::IfExists));
    assert_tag!(attributes, Tag::AbiFp16BitFormat(AbiFp16BitFormat::Alternative));
    assert_tag!(attributes, Tag::MpExtUse(MpExtUse::Allowed));
    assert_tag!(attributes, Tag::DivUse(DivUse::None));
    assert_tag!(attributes, Tag::DspExt(DspExt::IfExists));
    assert_tag!(attributes, Tag::MveArch(MveArch::IntFloat));
    assert_tag!(attributes, Tag::PacExt(PacExt::OnlyNopSpace));
    assert_tag!(attributes, Tag::BtiExt(BtiExt::Allowed));
    assert_tag!(attributes, Tag::NoDefaults);
    assert_tag!(attributes, Tag::AlsoCompatWith(AlsoCompatWith::Arch(CpuArch::V9A)));
    assert_tag!(
        attributes,
        Tag::AlsoCompatWith(AlsoCompatWith::Reserved(Box::new(Tag::CpuName(CpuName::Arm7Tdmi))))
    );
    assert_tag!(attributes, Tag::Conform(Conform::V2023Q3));
    assert_tag!(attributes, Tag::T2EeUse(T2EeUse::None));
    assert_tag!(attributes, Tag::VirtualUse(VirtualUse::VExts));
    assert_tag!(attributes, Tag::FramePointerUse(FramePointerUse::WithRecords));
    assert_tag!(attributes, Tag::BtiUse(BtiUse::Enabled));
    assert_tag!(attributes, Tag::PacretUse(PacretUse::Enabled));
}
