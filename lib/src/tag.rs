#![allow(non_upper_case_globals)]

use std::io::Cursor;

use crate::{
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
    CpuName(&'a str),
    /// Tag_CPU_arch
    CpuArch(u8),
    /// Tag_CPU_arch_profile
    CpuArchProfile(u8),
    /// Tag_ARM_ISA_use
    ArmIsaUse(u8),
    /// Tag_THUMB_ISA_use
    ThumbIsaUse(u8),
    /// Tag_FP_arch
    FpArch(u8),
    /// Tag_WMMX_arch
    WmmxArch(u8),
    /// Tag_Advanced_SIMD_arch
    AdvancedSimdArch(u8),
    /// Tag_PCS_config
    PcsConfig(u8),
    /// Tag_ABI_PCS_R9_use
    AbiPcsR9Use(u8),
    /// Tag_ABI_PCS_RW_data
    AbiPcsRwData(u8),
    /// Tag_ABI_PCS_RO_data
    AbiPcsRoData(u8),
    /// Tag_ABI_PCS_GOT_use
    AbiPcsGotUse(u8),
    /// Tag_ABI_PCS_wchar_t
    AbiPcsWcharT(u8),
    /// Tag_ABI_FP_rounding
    AbiFpRounding(u8),
    /// Tag_ABI_FP_denormal
    AbiFpDenormal(u8),
    /// Tag_ABI_FP_exceptions
    AbiFpExceptions(u8),
    /// Tag_ABI_FP_user_exceptions
    AbiFpUserExceptions(u8),
    /// Tag_ABI_FP_number_model
    AbiFpNumberModel(u8),
    /// Tag_ABI_align_needed
    AbiAlignNeeded(u8),
    /// Tag_ABI_align_preserved
    AbiAlignPreserved(u8),
    /// Tag_ABI_enum_size
    AbiEnumSize(u8),
    /// Tag_ABI_HardFP_use
    AbiHardFpUse(u8),
    /// Tag_ABI_VFP_args
    AbiVfpArgs(u8),
    /// Tag_ABI_WMMX_args
    AbiWmmxArgs(u8),
    /// Tag_ABI_optimization_goals
    AbiOptimizationGoals(u8),
    /// Tag_ABI_FP_optimization_goals
    AbiFpOptimizationGoals(u8),
    /// Tag_compatibility
    Compatibility { flag: u8, vendor_name: &'a str },
    /// Tag_CPU_unaligned_access
    CpuUnalignedAccess(u8),
    /// Tag_FP_HP_extension
    FpHpExtension(u8),
    /// Tag_ABI_FP_16bit_format
    AbiFp16BitFormat(u8),
    /// Tag_MPextension_use
    MpExtensionUse(u8),
    /// Tag_DIV_use
    DivUse(u8),
    /// Tag_DSP_extension
    DspExtension(u8),
    /// Tag_MVE_arch
    MveArch(u8),
    /// Tag_PAC_extension
    PacExtension(u8),
    /// Tag_BTI_extension
    BtiExtension(u8),
    /// Tag_nodefaults
    NoDefaults(u8),
    /// Tag_also_compatible_with
    AlsoCompatibleWith(Box<Tag<'a>>),
    /// Tag_conformance
    Conformance(&'a str),
    /// Tag_T2EE_use
    T2EeUse(u8),
    /// Tag_Virtualization_use
    VirtualizationUse(u8),
    /// Tag_FramePointer_use
    FramePointerUse(u8),
    /// Tag_BTI_use
    BtiUse(u8),
    /// Tag_PACRET_use
    PacretUse(u8),
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
                | Tag::Compatibility { flag: _, vendor_name: _ }
                | Tag::AlsoCompatibleWith(_)
                | Tag::Conformance(_)
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
            Tag::AdvancedSimdArch(_) => Tag_Advanced_SIMD_arch,
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
            Tag::AbiOptimizationGoals(_) => Tag_ABI_optimization_goals,
            Tag::AbiFpOptimizationGoals(_) => Tag_ABI_FP_optimization_goals,
            Tag::Compatibility { flag: _, vendor_name: _ } => Tag_compatibility,
            Tag::CpuUnalignedAccess(_) => Tag_CPU_unaligned_access,
            Tag::FpHpExtension(_) => Tag_FP_HP_extension,
            Tag::AbiFp16BitFormat(_) => Tag_ABI_FP_16bit_format,
            Tag::MpExtensionUse(_) => Tag_MPextension_use,
            Tag::DivUse(_) => Tag_DIV_use,
            Tag::DspExtension(_) => Tag_DSP_extension,
            Tag::MveArch(_) => Tag_MVE_arch,
            Tag::PacExtension(_) => Tag_PAC_extension,
            Tag::BtiExtension(_) => Tag_BTI_extension,
            Tag::NoDefaults(_) => Tag_nodefaults,
            Tag::AlsoCompatibleWith(_) => Tag_also_compatible_with,
            Tag::Conformance(_) => Tag_conformance,
            Tag::T2EeUse(_) => Tag_T2EE_use,
            Tag::VirtualizationUse(_) => Tag_Virtualization_use,
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
            Tag_CPU_name => Tag::CpuName(read_string(cursor).map_err(TagError::Read)?),
            Tag_CPU_arch => Tag::CpuArch(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_CPU_arch_profile => Tag::CpuArchProfile(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ARM_ISA_use => Tag::ArmIsaUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_THUMB_ISA_use => Tag::ThumbIsaUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_FP_arch => Tag::FpArch(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_WMMX_arch => Tag::WmmxArch(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_Advanced_SIMD_arch => Tag::AdvancedSimdArch(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_PCS_config => Tag::PcsConfig(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_PCS_R9_use => Tag::AbiPcsR9Use(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_PCS_RW_data => Tag::AbiPcsRwData(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_PCS_RO_data => Tag::AbiPcsRoData(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_PCS_GOT_use => Tag::AbiPcsGotUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_PCS_wchar_t => Tag::AbiPcsWcharT(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_rounding => Tag::AbiFpRounding(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_denormal => Tag::AbiFpDenormal(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_exceptions => Tag::AbiFpExceptions(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_user_exceptions => Tag::AbiFpUserExceptions(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_number_model => Tag::AbiFpNumberModel(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_align_needed => Tag::AbiAlignNeeded(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_align_preserved => Tag::AbiAlignPreserved(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_enum_size => Tag::AbiEnumSize(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_HardFP_use => Tag::AbiHardFpUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_VFP_args => Tag::AbiVfpArgs(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_WMMX_args => Tag::AbiWmmxArgs(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_optimization_goals => Tag::AbiOptimizationGoals(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_optimization_goals => Tag::AbiFpOptimizationGoals(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_compatibility => Tag::Compatibility {
                flag: read_uleb128(cursor).map_err(TagError::Read)?,
                vendor_name: read_string(cursor).map_err(TagError::Read)?,
            },
            Tag_CPU_unaligned_access => Tag::CpuUnalignedAccess(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_FP_HP_extension => Tag::FpHpExtension(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_ABI_FP_16bit_format => Tag::AbiFp16BitFormat(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_MPextension_use => Tag::MpExtensionUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_DIV_use => Tag::DivUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_DSP_extension => Tag::DspExtension(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_MVE_arch => Tag::MveArch(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_PAC_extension => Tag::PacExtension(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_BTI_extension => Tag::BtiExtension(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_nodefaults => Tag::NoDefaults(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_also_compatible_with => {
                let sub_tag = Tag::read(cursor, endian)?;
                if sub_tag.is_uleb128() {
                    let null = read_u8(cursor).map_err(TagError::Read)?;
                    if null != 0 {
                        return Err(TagError::ExpectedNull);
                    }
                }
                Tag::AlsoCompatibleWith(Box::new(sub_tag))
            }
            Tag_conformance => Tag::Conformance(read_string(cursor).map_err(TagError::Read)?),
            Tag_T2EE_use => Tag::T2EeUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_Virtualization_use => Tag::VirtualizationUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_FramePointer_use => Tag::FramePointerUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_BTI_use => Tag::BtiUse(read_uleb128(cursor).map_err(TagError::Read)?),
            Tag_PACRET_use => Tag::PacretUse(read_uleb128(cursor).map_err(TagError::Read)?),
            _ => return Err(TagError::IncompatibleTagValue(tag)),
        };
        Ok(tag)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Scope<'a> {
    /// Applies to whole file
    File,
    /// Applies to given section numbers
    Sections(&'a [u8]),
    /// Applies to given symbol numbers
    Symbols(&'a [u8]),
}

impl<'a> Scope<'a> {
    pub fn new(tag: Tag<'a>) -> Result<Self, TagError> {
        let scope = match tag {
            Tag::File { size: _ } => Scope::File,
            Tag::Section { size: _, sections } => Scope::Sections(sections),
            Tag::Symbol { size: _, symbols } => Scope::Symbols(symbols),
            _ => return Err(TagError::InvalidScopeTag),
        };
        Ok(scope)
    }
}
