pub mod enums;
pub mod error;
pub mod globals;
pub mod read;
pub mod tag;

use std::{
    collections::HashMap,
    fmt::Display,
    io::{Cursor, Seek},
};

use enums::*;
use error::{BuildAttrError, PublicAttrsError, ReadError, TagError};
use read::{read_string, read_u32, Endian};
use tag::{Scope, Tag};

pub struct BuildAttrs<'a> {
    data: &'a [u8],
    endian: Endian,
}

impl<'a> BuildAttrs<'a> {
    pub fn new(data: &'a [u8], endian: Endian) -> Result<Self, BuildAttrError> {
        if data.is_empty() {
            Err(BuildAttrError::NoData)
        } else {
            let attrs = Self { data, endian };
            let version = attrs.version();
            if version != b'A' {
                Err(BuildAttrError::IncompatibleVersion(version))
            } else {
                Ok(attrs)
            }
        }
    }

    pub fn version(&self) -> u8 {
        self.data[0]
    }

    pub fn subsections(&self) -> SubsectionIter {
        let data = &self.data[1..];
        SubsectionIter {
            data,
            cursor: Cursor::new(data),
            endian: self.endian,
        }
    }
}

pub struct SubsectionIter<'a> {
    data: &'a [u8],
    cursor: Cursor<&'a [u8]>,
    endian: Endian,
}

impl<'a> Iterator for SubsectionIter<'a> {
    type Item = Result<Subsection<'a>, ReadError>;

    fn next(&mut self) -> Option<Self::Item> {
        let length = match read_u32(&mut self.cursor, self.endian) {
            Ok(length) => length,
            Err(ReadError::Eof) => return None,
            Err(e) => return Some(Err(e)),
        };
        let vendor_name = match read_string(&mut self.cursor) {
            Ok(vendor_name) => vendor_name,
            Err(ReadError::Eof) => return None,
            Err(e) => return Some(Err(e)),
        };
        let name_size = vendor_name.len() + 1;

        let pos = self.cursor.position() as usize;
        let data = &self.data[pos..pos + length as usize - name_size - 4];
        if let Err(e) = self.cursor.seek(std::io::SeekFrom::Current(data.len() as i64)) {
            Some(Err(ReadError::Io(e)))
        } else {
            Some(Ok(Subsection {
                data,
                endian: self.endian,
                vendor_name,
            }))
        }
    }
}

pub struct Subsection<'a> {
    data: &'a [u8],
    endian: Endian,
    vendor_name: &'a str,
}

impl<'a> Subsection<'a> {
    pub fn is_aeabi(&self) -> bool {
        self.vendor_name == "aeabi"
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }

    pub fn vendor_name(&self) -> &str {
        self.vendor_name
    }
}

impl<'a> Subsection<'a> {
    pub fn into_public_attr_iter(self) -> Result<PublicAttrIter<'a>, PublicAttrsError> {
        if self.is_aeabi() {
            Ok(PublicAttrIter {
                cursor: Cursor::new(self.data),
                endian: self.endian,
            })
        } else {
            Err(PublicAttrsError::InvalidName(self.vendor_name.to_string()))
        }
    }

    pub fn into_public_attributes(self) -> Result<PublicAttributes<'a>, PublicAttrsError> {
        let mut tags = self.into_public_attr_iter()?.map(|a| a.map_err(PublicAttrsError::Tag));

        let first_tag = tags.next().unwrap_or(Err(PublicAttrsError::NoTags))?;
        if !matches!(first_tag, Tag::File { size: _ }) {
            return Err(PublicAttrsError::NoFileTag);
        }
        let first_scope = Scope::new(first_tag.clone()).map_err(PublicAttrsError::Tag)?;

        let mut file_scope = AttributeScope::default();
        let mut enclosed_scopes: HashMap<Scope, AttributeScope> = HashMap::new();
        let mut attrs = if first_scope == Scope::File {
            &mut file_scope
        } else {
            enclosed_scopes.entry(first_scope).or_default()
        };

        for tag in tags {
            let tag = tag?;
            match tag {
                Tag::File { size: _ } => {
                    attrs = &mut file_scope;
                }
                Tag::Section { size: _, sections: _ } | Tag::Symbol { size: _, symbols: _ } => {
                    let new_scope = Scope::new(tag).map_err(PublicAttrsError::Tag)?;
                    attrs = enclosed_scopes.entry(new_scope).or_default();
                }
                Tag::CpuRawName(name) => attrs.cpu_raw_name = Some(name),
                Tag::CpuName(name) => attrs.cpu_name = Some(CpuName::from(name)),
                Tag::CpuArch(value) => attrs.cpu_arch = Some(CpuArch::from(value)),
                Tag::CpuArchProfile(value) => attrs.cpu_arch_profile = Some(CpuArchProfile::from(value)),
                Tag::ArmIsaUse(value) => attrs.arm_isa_use = Some(ArmIsaUse::from(value)),
                Tag::ThumbIsaUse(value) => attrs.thumb_isa_use = Some(ThumbIsaUse::from(value)),
                Tag::FpArch(value) => attrs.fp_arch = Some(FpArch::from(value)),
                Tag::WmmxArch(value) => attrs.wmmx_arch = Some(WmmxArch::from(value)),
                Tag::AdvancedSimdArch(value) => attrs.asimd_arch = Some(AsimdArch::from(value)),
                Tag::PcsConfig(value) => attrs.pcs_config = Some(PcsConfig::from(value)),
                Tag::AbiPcsR9Use(value) => attrs.abi_pcs_r9_use = Some(AbiPcsR9Use::from(value)),
                Tag::AbiPcsRwData(value) => attrs.abi_pcs_rw_data = Some(AbiPcsRwData::from(value)),
                Tag::AbiPcsRoData(value) => attrs.abi_pcs_ro_data = Some(AbiPcsRoData::from(value)),
                Tag::AbiPcsGotUse(value) => attrs.abi_pcs_got_use = Some(AbiPcsGotUse::from(value)),
                Tag::AbiPcsWcharT(value) => attrs.abi_pcs_wchar_t = Some(AbiPcsWcharT::from(value)),
                Tag::AbiFpRounding(value) => attrs.abi_fp_rounding = Some(AbiFpRounding::from(value)),
                Tag::AbiFpDenormal(value) => attrs.abi_fp_denormal = Some(AbiFpDenormal::from(value)),
                Tag::AbiFpExceptions(value) => attrs.abi_fp_exceptions = Some(AbiFpExceptions::from(value)),
                Tag::AbiFpUserExceptions(value) => attrs.abi_fp_user_exceptions = Some(AbiFpUserExceptions::from(value)),
                Tag::AbiFpNumberModel(value) => attrs.abi_fp_number_model = Some(AbiFpNumberModel::from(value)),
                Tag::AbiAlignNeeded(value) => attrs.abi_align_needed = Some(AbiAlignNeeded::from(value)),
                Tag::AbiAlignPreserved(value) => attrs.abi_align_preserved = Some(AbiAlignPreserved::from(value)),
                Tag::AbiEnumSize(value) => attrs.abi_enum_size = Some(AbiEnumSize::from(value)),
                Tag::AbiHardFpUse(value) => attrs.abi_hardfp_use = Some(AbiHardFpUse::from(value)),
                Tag::AbiVfpArgs(value) => attrs.abi_vfp_args = Some(AbiVfpArgs::from(value)),
                Tag::AbiWmmxArgs(value) => attrs.abi_wmmx_args = Some(AbiWmmxArgs::from(value)),
                Tag::AbiOptimizationGoals(value) => attrs.abi_opt_goals = Some(AbiOptGoals::from(value)),
                Tag::AbiFpOptimizationGoals(value) => attrs.abi_fp_opt_goals = Some(AbiFpOptGoals::from(value)),
                Tag::Compatibility { flag, vendor_name } => attrs.compat = Some(Compat::new(flag, vendor_name)),
                Tag::CpuUnalignedAccess(value) => attrs.cpu_unaligned_access = Some(CpuUnalignedAccess::from(value)),
                Tag::FpHpExtension(value) => attrs.fp_hp_ext = Some(FpHpExt::from(value)),
                Tag::AbiFp16BitFormat(value) => attrs.abi_fp_16bit_format = Some(AbiFp16BitFormat::from(value)),
                Tag::MpExtensionUse(value) => attrs.mp_ext_use = Some(MpExtUse::from(value)),
                Tag::DivUse(value) => attrs.div_use = Some(DivUse::from(value)),
                Tag::DspExtension(value) => attrs.dsp_ext = Some(DspExt::from(value)),
                Tag::MveArch(value) => attrs.mve_arch = Some(MveArch::from(value)),
                Tag::PacExtension(value) => attrs.pac_ext = Some(PacExt::from(value)),
                Tag::BtiExtension(value) => attrs.bti_ext = Some(BtiExt::from(value)),
                Tag::NoDefaults(_) => attrs.no_defaults = true,
                Tag::AlsoCompatibleWith(tag) => attrs.also_compat_with = Some(AlsoCompatWith::new(*tag)),
                Tag::Conformance(version) => attrs.conform = Some(Conform(version)),
                Tag::T2EeUse(value) => attrs.t2ee_use = Some(T2EeUse::from(value)),
                Tag::VirtualizationUse(value) => attrs.virtual_use = Some(VirtualUse::from(value)),
                Tag::FramePointerUse(value) => attrs.frame_pointer_use = Some(FramePointerUse::from(value)),
                Tag::BtiUse(value) => attrs.bti_use = Some(BtiUse::from(value)),
                Tag::PacretUse(value) => attrs.pacret_use = Some(PacretUse::from(value)),
            }
        }

        Ok(PublicAttributes {
            file_scope,
            enclosed_scopes,
        })
    }
}

pub struct PublicAttrIter<'a> {
    cursor: Cursor<&'a [u8]>,
    endian: Endian,
}

impl<'a> Iterator for PublicAttrIter<'a> {
    type Item = Result<Tag<'a>, TagError>;

    fn next(&mut self) -> Option<Self::Item> {
        match Tag::read(&mut self.cursor, self.endian) {
            Ok(tag) => Some(Ok(tag)),
            Err(TagError::Read(ReadError::Eof)) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

pub struct PublicAttributes<'a> {
    pub file_scope: AttributeScope<'a>,
    pub enclosed_scopes: HashMap<Scope<'a>, AttributeScope<'a>>,
}

#[derive(Default)]
pub struct AttributeScope<'a> {
    // CPU
    pub cpu_raw_name: Option<&'a str>,
    pub cpu_name: Option<CpuName<'a>>,
    pub cpu_arch: Option<CpuArch>,
    pub cpu_arch_profile: Option<CpuArchProfile>,

    // Arch
    pub arm_isa_use: Option<ArmIsaUse>,
    pub thumb_isa_use: Option<ThumbIsaUse>,
    pub fp_arch: Option<FpArch>,
    pub wmmx_arch: Option<WmmxArch>,
    pub asimd_arch: Option<AsimdArch>,
    pub mve_arch: Option<MveArch>,
    pub fp_hp_ext: Option<FpHpExt>,
    pub cpu_unaligned_access: Option<CpuUnalignedAccess>,
    pub t2ee_use: Option<T2EeUse>,
    pub virtual_use: Option<VirtualUse>,
    pub mp_ext_use: Option<MpExtUse>,
    pub div_use: Option<DivUse>,
    pub dsp_ext: Option<DspExt>,
    pub pac_ext: Option<PacExt>,
    pub bti_ext: Option<BtiExt>,

    // ABI
    pub pcs_config: Option<PcsConfig>,
    pub abi_pcs_r9_use: Option<AbiPcsR9Use>,
    pub abi_pcs_rw_data: Option<AbiPcsRwData>,
    pub abi_pcs_ro_data: Option<AbiPcsRoData>,
    pub abi_pcs_got_use: Option<AbiPcsGotUse>,
    pub abi_pcs_wchar_t: Option<AbiPcsWcharT>,
    pub abi_enum_size: Option<AbiEnumSize>,
    pub abi_align_needed: Option<AbiAlignNeeded>,
    pub abi_align_preserved: Option<AbiAlignPreserved>,
    pub abi_fp_rounding: Option<AbiFpRounding>,
    pub abi_fp_denormal: Option<AbiFpDenormal>,
    pub abi_fp_exceptions: Option<AbiFpExceptions>,
    pub abi_fp_user_exceptions: Option<AbiFpUserExceptions>,
    pub abi_fp_number_model: Option<AbiFpNumberModel>,
    pub abi_fp_16bit_format: Option<AbiFp16BitFormat>,
    pub abi_hardfp_use: Option<AbiHardFpUse>,
    pub abi_vfp_args: Option<AbiVfpArgs>,
    pub abi_wmmx_args: Option<AbiWmmxArgs>,
    pub frame_pointer_use: Option<FramePointerUse>,
    pub bti_use: Option<BtiUse>,
    pub pacret_use: Option<PacretUse>,
    pub abi_opt_goals: Option<AbiOptGoals>,
    pub abi_fp_opt_goals: Option<AbiFpOptGoals>,

    // Compat
    pub compat: Option<Compat<'a>>,
    pub also_compat_with: Option<AlsoCompatWith<'a>>,
    pub conform: Option<Conform<'a>>,

    // Misc
    pub no_defaults: bool,
}

impl<'a> AttributeScope<'a> {
    pub fn display(&self, indent: usize, show_defaults: bool) -> AttributeScopeDisplay {
        AttributeScopeDisplay {
            scope: self,
            indent,
            show_defaults,
        }
    }
}

pub struct AttributeScopeDisplay<'a> {
    scope: &'a AttributeScope<'a>,
    indent: usize,
    show_defaults: bool,
}

impl<'a> AttributeScopeDisplay<'a> {
    fn display_field<T: Display + Default>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        field: &str,
        value: &Option<T>,
    ) -> std::fmt::Result {
        if let Some(value) = value {
            writeln!(f, "{}{} : {}", format_args!("{: >1$}", "", self.indent), field, value)
        } else if self.show_defaults {
            let value = T::default();
            writeln!(
                f,
                "{}{} : [default] {}",
                format_args!("{: >1$}", "", self.indent),
                field,
                value
            )
        } else {
            Ok(())
        }
    }
}

impl<'a> Display for AttributeScopeDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = self.scope;
        self.display_field(f, "CPU raw name .........", &scope.cpu_raw_name)?;
        self.display_field(f, "CPU name .............", &scope.cpu_name)?;
        self.display_field(f, "CPU arch .............", &scope.cpu_arch)?;
        self.display_field(f, "CPU arch profile .....", &scope.cpu_arch_profile)?;
        self.display_field(f, "ARM ISA use ..........", &scope.arm_isa_use)?;
        self.display_field(f, "Thumb ISA use ........", &scope.thumb_isa_use)?;
        self.display_field(f, "FP arch ..............", &scope.fp_arch)?;
        self.display_field(f, "WMMX arch ............", &scope.wmmx_arch)?;
        self.display_field(f, "Advanced SIMD arch ...", &scope.asimd_arch)?;
        self.display_field(f, "MVE arch .............", &scope.mve_arch)?;
        self.display_field(f, "FP HP extension ......", &scope.fp_hp_ext)?;
        self.display_field(f, "Unaligned access .....", &scope.cpu_unaligned_access)?;
        self.display_field(f, "T2EE use .............", &scope.t2ee_use)?;
        self.display_field(f, "Virtualization use ...", &scope.virtual_use)?;
        self.display_field(f, "MP extension use .....", &scope.mp_ext_use)?;
        self.display_field(f, "DIV use ..............", &scope.div_use)?;
        self.display_field(f, "DSP use ..............", &scope.dsp_ext)?;
        self.display_field(f, "PAC extension ........", &scope.pac_ext)?;
        self.display_field(f, "BTI extension ........", &scope.bti_ext)?;
        self.display_field(f, "PCS config ...........", &scope.pcs_config)?;
        self.display_field(f, "PCS R9 use ...........", &scope.abi_pcs_r9_use)?;
        self.display_field(f, "PCS RW data ..........", &scope.abi_pcs_rw_data)?;
        self.display_field(f, "PCS RO data ..........", &scope.abi_pcs_ro_data)?;
        self.display_field(f, "PCS GOT use ..........", &scope.abi_pcs_got_use)?;
        self.display_field(f, "PCS wchar_t ..........", &scope.abi_pcs_wchar_t)?;
        self.display_field(f, "Enum size ............", &scope.abi_enum_size)?;
        self.display_field(f, "Align needed .........", &scope.abi_align_needed)?;
        self.display_field(f, "Align preserved ......", &scope.abi_align_preserved)?;
        self.display_field(f, "FP rounding ..........", &scope.abi_fp_rounding)?;
        self.display_field(f, "FP denormal ..........", &scope.abi_fp_denormal)?;
        self.display_field(f, "FP exceptions ........", &scope.abi_fp_exceptions)?;
        self.display_field(f, "FP user exceptions ...", &scope.abi_fp_user_exceptions)?;
        self.display_field(f, "FP number format .....", &scope.abi_fp_number_model)?;
        self.display_field(f, "FP 16-bit format .....", &scope.abi_fp_16bit_format)?;
        self.display_field(f, "FP hardware use ......", &scope.abi_hardfp_use)?;
        self.display_field(f, "VFP args .............", &scope.abi_vfp_args)?;
        self.display_field(f, "WMMX args ............", &scope.abi_wmmx_args)?;
        self.display_field(f, "Frame Pointer use ....", &scope.frame_pointer_use)?;
        self.display_field(f, "BTI use ..............", &scope.bti_use)?;
        self.display_field(f, "PACRET use ...........", &scope.pacret_use)?;
        self.display_field(f, "Optimization goals ...", &scope.abi_opt_goals)?;
        self.display_field(f, "FP optimization goals ", &scope.abi_fp_opt_goals)?;
        self.display_field(f, "Compatibility ........", &scope.compat)?;
        self.display_field(f, "Also compatible with .", &scope.also_compat_with)?;
        self.display_field(f, "Conformance ..........", &scope.conform)?;
        Ok(())
    }
}
