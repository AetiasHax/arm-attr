#![allow(non_upper_case_globals)]

use std::io::Cursor;

use crate::{
    enums::*,
    error::TagError,
    globals::*,
    read::{read_string, read_u32, read_u8, read_uleb128, read_uleb128_list, Endian},
};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Tag<'a> {
    /// Tag_File
    File { size: u32 },
    /// Tag_Section
    Section { size: u32, sections: &'a [u8] },
    /// Tag_Symbol
    Symbol { size: u32, symbols: &'a [u8] },
    /// Tag_CPU_raw_name
    CpuRawName(&'a str),
    /// Tag_CPU_name
    CpuName(CpuName<'a>),
    /// Tag_CPU_arch
    CpuArch(CpuArch),
    /// Tag_CPU_arch_profile
    CpuArchProfile(CpuArchProfile),
    /// Tag_ARM_ISA_use
    ArmIsaUse(ArmIsaUse),
    /// Tag_THUMB_ISA_use
    ThumbIsaUse(ThumbIsaUse),
    /// Tag_FP_arch
    FpArch(FpArch),
    /// Tag_WMMX_arch
    WmmxArch(WmmxArch),
    /// Tag_Advanced_SIMD_arch
    AsimdArch(AsimdArch),
    /// Tag_PCS_config
    PcsConfig(PcsConfig),
    /// Tag_ABI_PCS_R9_use
    AbiPcsR9Use(AbiPcsR9Use),
    /// Tag_ABI_PCS_RW_data
    AbiPcsRwData(AbiPcsRwData),
    /// Tag_ABI_PCS_RO_data
    AbiPcsRoData(AbiPcsRoData),
    /// Tag_ABI_PCS_GOT_use
    AbiPcsGotUse(AbiPcsGotUse),
    /// Tag_ABI_PCS_wchar_t
    AbiPcsWcharT(AbiPcsWcharT),
    /// Tag_ABI_FP_rounding
    AbiFpRounding(AbiFpRounding),
    /// Tag_ABI_FP_denormal
    AbiFpDenormal(AbiFpDenormal),
    /// Tag_ABI_FP_exceptions
    AbiFpExceptions(AbiFpExceptions),
    /// Tag_ABI_FP_user_exceptions
    AbiFpUserExceptions(AbiFpUserExceptions),
    /// Tag_ABI_FP_number_model
    AbiFpNumberModel(AbiFpNumberModel),
    /// Tag_ABI_align_needed
    AbiAlignNeeded(AbiAlignNeeded),
    /// Tag_ABI_align_preserved
    AbiAlignPreserved(AbiAlignPreserved),
    /// Tag_ABI_enum_size
    AbiEnumSize(AbiEnumSize),
    /// Tag_ABI_HardFP_use
    AbiHardFpUse(AbiHardFpUse),
    /// Tag_ABI_VFP_args
    AbiVfpArgs(AbiVfpArgs),
    /// Tag_ABI_WMMX_args
    AbiWmmxArgs(AbiWmmxArgs),
    /// Tag_ABI_optimization_goals
    AbiOptGoals(AbiOptGoals),
    /// Tag_ABI_FP_optimization_goals
    AbiFpOptGoals(AbiFpOptGoals),
    /// Tag_compatibility
    Compat(Compat<'a>),
    /// Tag_CPU_unaligned_access
    CpuUnalignedAccess(CpuUnalignedAccess),
    /// Tag_FP_HP_extension
    FpHpExt(FpHpExt),
    /// Tag_ABI_FP_16bit_format
    AbiFp16BitFormat(AbiFp16BitFormat),
    /// Tag_MPextension_use
    MpExtUse(MpExtUse),
    /// Tag_DIV_use
    DivUse(DivUse),
    /// Tag_DSP_extension
    DspExt(DspExt),
    /// Tag_MVE_arch
    MveArch(MveArch),
    /// Tag_PAC_extension
    PacExt(PacExt),
    /// Tag_BTI_extension
    BtiExt(BtiExt),
    /// Tag_nodefaults
    NoDefaults,
    /// Tag_also_compatible_with
    AlsoCompatWith(AlsoCompatWith<'a>),
    /// Tag_conformance
    Conform(Conform<'a>),
    /// Tag_T2EE_use
    T2EeUse(T2EeUse),
    /// Tag_Virtualization_use
    VirtualUse(VirtualUse),
    /// Tag_FramePointer_use
    FramePointerUse(FramePointerUse),
    /// Tag_BTI_use
    BtiUse(BtiUse),
    /// Tag_PACRET_use
    PacretUse(PacretUse),
}

impl<'a> Tag<'a> {
    pub fn is_uleb128(&self) -> bool {
        !matches!(
            self,
            Tag::File { size: _ }
                | Tag::Section { size: _, sections: _ }
                | Tag::Symbol { size: _, symbols: _ }
                | Tag::CpuRawName(_)
                | Tag::CpuName(_)
                | Tag::Compat(_)
                | Tag::AlsoCompatWith(_)
                | Tag::Conform(_)
        )
    }

    pub fn raw_tag(&self) -> u8 {
        match self {
            Tag::File { size: _ } => Tag_File,
            Tag::Section { size: _, sections: _ } => Tag_Section,
            Tag::Symbol { size: _, symbols: _ } => Tag_Symbol,
            Tag::CpuRawName(_) => Tag_CPU_raw_name,
            Tag::CpuName(_) => Tag_CPU_name,
            Tag::CpuArch(_) => Tag_CPU_arch,
            Tag::CpuArchProfile(_) => Tag_CPU_arch_profile,
            Tag::ArmIsaUse(_) => Tag_ARM_ISA_use,
            Tag::ThumbIsaUse(_) => Tag_THUMB_ISA_use,
            Tag::FpArch(_) => Tag_FP_arch,
            Tag::WmmxArch(_) => Tag_WMMX_arch,
            Tag::AsimdArch(_) => Tag_Advanced_SIMD_arch,
            Tag::PcsConfig(_) => Tag_PCS_config,
            Tag::AbiPcsR9Use(_) => Tag_ABI_PCS_R9_use,
            Tag::AbiPcsRwData(_) => Tag_ABI_PCS_RW_data,
            Tag::AbiPcsRoData(_) => Tag_ABI_PCS_RO_data,
            Tag::AbiPcsGotUse(_) => Tag_ABI_PCS_GOT_use,
            Tag::AbiPcsWcharT(_) => Tag_ABI_PCS_wchar_t,
            Tag::AbiFpRounding(_) => Tag_ABI_FP_rounding,
            Tag::AbiFpDenormal(_) => Tag_ABI_FP_denormal,
            Tag::AbiFpExceptions(_) => Tag_ABI_FP_exceptions,
            Tag::AbiFpUserExceptions(_) => Tag_ABI_FP_user_exceptions,
            Tag::AbiFpNumberModel(_) => Tag_ABI_FP_number_model,
            Tag::AbiAlignNeeded(_) => Tag_ABI_align_needed,
            Tag::AbiAlignPreserved(_) => Tag_ABI_align_preserved,
            Tag::AbiEnumSize(_) => Tag_ABI_enum_size,
            Tag::AbiHardFpUse(_) => Tag_ABI_HardFP_use,
            Tag::AbiVfpArgs(_) => Tag_ABI_VFP_args,
            Tag::AbiWmmxArgs(_) => Tag_ABI_WMMX_args,
            Tag::AbiOptGoals(_) => Tag_ABI_optimization_goals,
            Tag::AbiFpOptGoals(_) => Tag_ABI_FP_optimization_goals,
            Tag::Compat(_) => Tag_compatibility,
            Tag::CpuUnalignedAccess(_) => Tag_CPU_unaligned_access,
            Tag::FpHpExt(_) => Tag_FP_HP_extension,
            Tag::AbiFp16BitFormat(_) => Tag_ABI_FP_16bit_format,
            Tag::MpExtUse(_) => Tag_MPextension_use,
            Tag::DivUse(_) => Tag_DIV_use,
            Tag::DspExt(_) => Tag_DSP_extension,
            Tag::MveArch(_) => Tag_MVE_arch,
            Tag::PacExt(_) => Tag_PAC_extension,
            Tag::BtiExt(_) => Tag_BTI_extension,
            Tag::NoDefaults => Tag_nodefaults,
            Tag::AlsoCompatWith(_) => Tag_also_compatible_with,
            Tag::Conform(_) => Tag_conformance,
            Tag::T2EeUse(_) => Tag_T2EE_use,
            Tag::VirtualUse(_) => Tag_Virtualization_use,
            Tag::FramePointerUse(_) => Tag_FramePointer_use,
            Tag::BtiUse(_) => Tag_BTI_use,
            Tag::PacretUse(_) => Tag_PACRET_use,
        }
    }

    pub(crate) fn read(cursor: &mut Cursor<&'a [u8]>, endian: Endian) -> Result<Self, TagError> {
        let tag = read_uleb128(cursor).map_err(TagError::Read)?;
        let tag = match tag {
            Tag_File => Tag::File {
                size: read_u32(cursor, endian).map_err(TagError::Read)?,
            },
            Tag_Section => Tag::Section {
                size: read_u32(cursor, endian).map_err(TagError::Read)?,
                sections: read_uleb128_list(cursor).map_err(TagError::Read)?,
            },
            Tag_Symbol => Tag::Symbol {
                size: read_u32(cursor, endian).map_err(TagError::Read)?,
                symbols: read_uleb128_list(cursor).map_err(TagError::Read)?,
            },
            Tag_CPU_raw_name => Tag::CpuRawName(read_string(cursor).map_err(TagError::Read)?),
            Tag_CPU_name => Tag::CpuName(CpuName::from(read_string(cursor).map_err(TagError::Read)?)),
            Tag_CPU_arch => Tag::CpuArch(CpuArch::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_CPU_arch_profile => Tag::CpuArchProfile(CpuArchProfile::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ARM_ISA_use => Tag::ArmIsaUse(ArmIsaUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_THUMB_ISA_use => Tag::ThumbIsaUse(ThumbIsaUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_FP_arch => Tag::FpArch(FpArch::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_WMMX_arch => Tag::WmmxArch(WmmxArch::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_Advanced_SIMD_arch => Tag::AsimdArch(AsimdArch::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_PCS_config => Tag::PcsConfig(PcsConfig::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_PCS_R9_use => Tag::AbiPcsR9Use(AbiPcsR9Use::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_PCS_RW_data => Tag::AbiPcsRwData(AbiPcsRwData::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_PCS_RO_data => Tag::AbiPcsRoData(AbiPcsRoData::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_PCS_GOT_use => Tag::AbiPcsGotUse(AbiPcsGotUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_PCS_wchar_t => Tag::AbiPcsWcharT(AbiPcsWcharT::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_FP_rounding => Tag::AbiFpRounding(AbiFpRounding::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_FP_denormal => Tag::AbiFpDenormal(AbiFpDenormal::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_FP_exceptions => {
                Tag::AbiFpExceptions(AbiFpExceptions::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_ABI_FP_user_exceptions => {
                Tag::AbiFpUserExceptions(AbiFpUserExceptions::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_ABI_FP_number_model => {
                Tag::AbiFpNumberModel(AbiFpNumberModel::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_ABI_align_needed => Tag::AbiAlignNeeded(AbiAlignNeeded::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_align_preserved => {
                Tag::AbiAlignPreserved(AbiAlignPreserved::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_ABI_enum_size => Tag::AbiEnumSize(AbiEnumSize::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_HardFP_use => Tag::AbiHardFpUse(AbiHardFpUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_VFP_args => Tag::AbiVfpArgs(AbiVfpArgs::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_WMMX_args => Tag::AbiWmmxArgs(AbiWmmxArgs::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_optimization_goals => Tag::AbiOptGoals(AbiOptGoals::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_FP_optimization_goals => {
                Tag::AbiFpOptGoals(AbiFpOptGoals::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_compatibility => Tag::Compat(Compat::new(
                read_uleb128(cursor).map_err(TagError::Read)?,
                read_string(cursor).map_err(TagError::Read)?,
            )),
            Tag_CPU_unaligned_access => {
                Tag::CpuUnalignedAccess(CpuUnalignedAccess::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_FP_HP_extension => Tag::FpHpExt(FpHpExt::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_ABI_FP_16bit_format => {
                Tag::AbiFp16BitFormat(AbiFp16BitFormat::from(read_uleb128(cursor).map_err(TagError::Read)?))
            }
            Tag_MPextension_use => Tag::MpExtUse(MpExtUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_DIV_use => Tag::DivUse(DivUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_DSP_extension => Tag::DspExt(DspExt::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_MVE_arch => Tag::MveArch(MveArch::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_PAC_extension => Tag::PacExt(PacExt::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_BTI_extension => Tag::BtiExt(BtiExt::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_nodefaults => {
                let ignored = read_u8(cursor).map_err(TagError::Read)?;
                if ignored != 0 {
                    return Err(TagError::ExpectedNull);
                }
                Tag::NoDefaults
            }
            Tag_also_compatible_with => {
                let sub_tag = Tag::read(cursor, endian)?;
                if sub_tag.is_uleb128() {
                    let null = read_u8(cursor).map_err(TagError::Read)?;
                    if null != 0 {
                        return Err(TagError::ExpectedNull);
                    }
                }
                Tag::AlsoCompatWith(AlsoCompatWith::new(sub_tag))
            }
            Tag_conformance => Tag::Conform(Conform::from(read_string(cursor).map_err(TagError::Read)?)),
            Tag_T2EE_use => Tag::T2EeUse(T2EeUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_Virtualization_use => Tag::VirtualUse(VirtualUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_FramePointer_use => Tag::FramePointerUse(FramePointerUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_BTI_use => Tag::BtiUse(BtiUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            Tag_PACRET_use => Tag::PacretUse(PacretUse::from(read_uleb128(cursor).map_err(TagError::Read)?)),
            _ => return Err(TagError::IncompatibleTagValue(tag)),
        };
        Ok(tag)
    }
}
