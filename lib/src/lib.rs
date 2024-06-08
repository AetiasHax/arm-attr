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
use tag::Tag;

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
        let end = pos + length as usize - name_size - 4;
        if end > self.data.len() {
            return Some(Err(ReadError::OutOfBounds));
        }
        let data = &self.data[pos..end];
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

    pub fn into_public_attributes(self) -> Result<File<'a>, PublicAttrsError> {
        let data_len = self.data.len();
        let mut tags = self.into_public_attr_iter()?.map(|a| a.map_err(PublicAttrsError::Tag));

        let first_tag = tags.next().unwrap_or(Err(PublicAttrsError::NoTags))?;
        if let (_, Tag::File { end_offset }) = first_tag {
            if end_offset as usize != data_len {
                return Err(PublicAttrsError::ScopeEndsBeforeParent);
            }
        } else {
            return Err(PublicAttrsError::NoFileTag);
        }

        let mut file = File::default();
        let mut attrs = &mut file.attributes;
        let mut curr_section = None;
        let mut curr_symbol = None;

        for tag in tags {
            let (offset, tag) = tag?;

            if let Some((end_offset, _)) = curr_symbol {
                if offset >= end_offset {
                    curr_symbol = None;
                    attrs = if let Some((_, sections)) = curr_section {
                        &mut file.sections.entry(sections).or_default().attributes
                    } else {
                        &mut file.attributes
                    };
                }
            }

            if let Some((end_offset, _)) = curr_section {
                if offset >= end_offset {
                    curr_section = None;
                    attrs = &mut file.attributes;
                }
            }

            match tag {
                Tag::File { end_offset: _ } => return Err(PublicAttrsError::DuplicateFileTag),
                Tag::Section { end_offset, sections } => {
                    if curr_section.is_none() && curr_symbol.is_none() {
                        let section = file.sections.entry(sections).or_default();
                        attrs = &mut section.attributes;
                        curr_section = Some((end_offset, sections));
                    } else {
                        return Err(PublicAttrsError::NotFileScope);
                    }
                }
                Tag::Symbol { end_offset, symbols } => {
                    if let Some((section_end, sections)) = &curr_section {
                        if end_offset > *section_end {
                            return Err(PublicAttrsError::ScopeEndsBeforeParent);
                        }
                        let symbol = file.sections.entry(sections).or_default().symbols.entry(symbols).or_default();
                        attrs = &mut symbol.attributes;
                        curr_symbol = Some((end_offset, symbols));
                    } else {
                        return Err(PublicAttrsError::NotSectionScope);
                    }
                }
                Tag::CpuRawName(x) => attrs.cpu_raw_name = Some(x),
                Tag::CpuName(x) => attrs.cpu_name = Some(x),
                Tag::CpuArch(x) => attrs.cpu_arch = Some(x),
                Tag::CpuArchProfile(x) => attrs.cpu_arch_profile = Some(x),
                Tag::ArmIsaUse(x) => attrs.arm_isa_use = Some(x),
                Tag::ThumbIsaUse(x) => attrs.thumb_isa_use = Some(x),
                Tag::FpArch(x) => attrs.fp_arch = Some(x),
                Tag::WmmxArch(x) => attrs.wmmx_arch = Some(x),
                Tag::AsimdArch(x) => attrs.asimd_arch = Some(x),
                Tag::PcsConfig(x) => attrs.pcs_config = Some(x),
                Tag::AbiPcsR9Use(x) => attrs.abi_pcs_r9_use = Some(x),
                Tag::AbiPcsRwData(x) => attrs.abi_pcs_rw_data = Some(x),
                Tag::AbiPcsRoData(x) => attrs.abi_pcs_ro_data = Some(x),
                Tag::AbiPcsGotUse(x) => attrs.abi_pcs_got_use = Some(x),
                Tag::AbiPcsWcharT(x) => attrs.abi_pcs_wchar_t = Some(x),
                Tag::AbiFpRounding(x) => attrs.abi_fp_rounding = Some(x),
                Tag::AbiFpDenormal(x) => attrs.abi_fp_denormal = Some(x),
                Tag::AbiFpExceptions(x) => attrs.abi_fp_exceptions = Some(x),
                Tag::AbiFpUserExceptions(x) => attrs.abi_fp_user_exceptions = Some(x),
                Tag::AbiFpNumberModel(x) => attrs.abi_fp_number_model = Some(x),
                Tag::AbiAlignNeeded(x) => attrs.abi_align_needed = Some(x),
                Tag::AbiAlignPreserved(x) => attrs.abi_align_preserved = Some(x),
                Tag::AbiEnumSize(x) => attrs.abi_enum_size = Some(x),
                Tag::AbiHardFpUse(x) => attrs.abi_hardfp_use = Some(x),
                Tag::AbiVfpArgs(x) => attrs.abi_vfp_args = Some(x),
                Tag::AbiWmmxArgs(x) => attrs.abi_wmmx_args = Some(x),
                Tag::AbiOptGoals(x) => attrs.abi_opt_goals = Some(x),
                Tag::AbiFpOptGoals(x) => attrs.abi_fp_opt_goals = Some(x),
                Tag::Compat(x) => attrs.compat = Some(x),
                Tag::CpuUnalignedAccess(x) => attrs.cpu_unaligned_access = Some(x),
                Tag::FpHpExt(x) => attrs.fp_hp_ext = Some(x),
                Tag::AbiFp16BitFormat(x) => attrs.abi_fp_16bit_format = Some(x),
                Tag::MpExtUse(x) => attrs.mp_ext_use = Some(x),
                Tag::DivUse(x) => attrs.div_use = Some(x),
                Tag::DspExt(x) => attrs.dsp_ext = Some(x),
                Tag::MveArch(x) => attrs.mve_arch = Some(x),
                Tag::PacExt(x) => attrs.pac_ext = Some(x),
                Tag::BtiExt(x) => attrs.bti_ext = Some(x),
                Tag::AlsoCompatWith(x) => attrs.also_compat_with = Some(x),
                Tag::Conform(x) => attrs.conform = Some(x),
                Tag::T2EeUse(x) => attrs.t2ee_use = Some(x),
                Tag::VirtualUse(x) => attrs.virtual_use = Some(x),
                Tag::FramePointerUse(x) => attrs.frame_pointer_use = Some(x),
                Tag::BtiUse(x) => attrs.bti_use = Some(x),
                Tag::PacretUse(x) => attrs.pacret_use = Some(x),
                Tag::NoDefaults => attrs.no_defaults = true,
            }
        }

        if !file.attributes.no_defaults {
            for section in file.sections.values_mut() {
                if section.attributes.empty() {
                    section.attributes.inherit(&file.attributes);
                }
                if !section.attributes.no_defaults {
                    for symbol in section.symbols.values_mut() {
                        if symbol.attributes.empty() {
                            symbol.attributes.inherit(&section.attributes);
                        }
                    }
                }
            }
        }

        Ok(file)
    }
}

pub struct PublicAttrIter<'a> {
    cursor: Cursor<&'a [u8]>,
    endian: Endian,
}

impl<'a> Iterator for PublicAttrIter<'a> {
    type Item = Result<(u32, Tag<'a>), TagError>;

    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.cursor.position() as u32;
        match Tag::read(&mut self.cursor, self.endian) {
            Ok(tag) => Some(Ok((offset, tag))),
            Err(TagError::Read(ReadError::Eof)) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[derive(Default)]
pub struct File<'a> {
    pub attributes: Attributes<'a>,
    /// Maps list of section indices to a section group
    pub sections: HashMap<&'a [u8], SectionGroup<'a>>,
}

#[derive(Default)]
pub struct SectionGroup<'a> {
    pub attributes: Attributes<'a>,
    /// Maps list of symbol values to a symbol group
    pub symbols: HashMap<&'a [u8], SymbolGroup<'a>>,
}

#[derive(Default)]
pub struct SymbolGroup<'a> {
    pub attributes: Attributes<'a>,
}

#[derive(Default)]
pub struct Attributes<'a> {
    // Target-related attributes
    pub cpu_raw_name: Option<&'a str>,
    pub cpu_name: Option<CpuName<'a>>,
    pub cpu_arch: Option<CpuArch>,
    pub cpu_arch_profile: Option<CpuArchProfile>,
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

    // Procedure call-related attributes
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

    // Miscellaneous attributes
    pub pacret_use: Option<PacretUse>,
    pub abi_opt_goals: Option<AbiOptGoals>,
    pub abi_fp_opt_goals: Option<AbiFpOptGoals>,
    pub compat: Option<Compat<'a>>,
    pub also_compat_with: Option<AlsoCompatWith<'a>>,
    pub conform: Option<Conform<'a>>,
    pub no_defaults: bool,
}

impl<'a> Attributes<'a> {
    pub fn empty(&self) -> bool {
        self.cpu_raw_name.is_some()
            || self.cpu_name.is_some()
            || self.cpu_arch.is_some()
            || self.cpu_arch_profile.is_some()
            || self.arm_isa_use.is_some()
            || self.thumb_isa_use.is_some()
            || self.fp_arch.is_some()
            || self.wmmx_arch.is_some()
            || self.asimd_arch.is_some()
            || self.mve_arch.is_some()
            || self.fp_hp_ext.is_some()
            || self.cpu_unaligned_access.is_some()
            || self.t2ee_use.is_some()
            || self.virtual_use.is_some()
            || self.mp_ext_use.is_some()
            || self.div_use.is_some()
            || self.dsp_ext.is_some()
            || self.pac_ext.is_some()
            || self.bti_ext.is_some()
            || self.pcs_config.is_some()
            || self.abi_pcs_r9_use.is_some()
            || self.abi_pcs_rw_data.is_some()
            || self.abi_pcs_ro_data.is_some()
            || self.abi_pcs_got_use.is_some()
            || self.abi_pcs_wchar_t.is_some()
            || self.abi_enum_size.is_some()
            || self.abi_align_needed.is_some()
            || self.abi_align_preserved.is_some()
            || self.abi_fp_rounding.is_some()
            || self.abi_fp_denormal.is_some()
            || self.abi_fp_exceptions.is_some()
            || self.abi_fp_user_exceptions.is_some()
            || self.abi_fp_number_model.is_some()
            || self.abi_fp_16bit_format.is_some()
            || self.abi_hardfp_use.is_some()
            || self.abi_vfp_args.is_some()
            || self.abi_wmmx_args.is_some()
            || self.frame_pointer_use.is_some()
            || self.bti_use.is_some()
            || self.pacret_use.is_some()
            || self.abi_opt_goals.is_some()
            || self.abi_fp_opt_goals.is_some()
            || self.compat.is_some()
            || self.also_compat_with.is_some()
            || self.conform.is_some()
    }

    fn inherit(&mut self, from: &Attributes<'a>) {
        macro_rules! inherit {
            ($to:ident, $from:ident, $tag:ident) => {
                $to.$tag = $to.$tag.or($from.$tag)
            };
        }
        inherit!(self, from, cpu_raw_name);
        inherit!(self, from, cpu_name);
        inherit!(self, from, cpu_arch);
        inherit!(self, from, cpu_arch_profile);
        inherit!(self, from, arm_isa_use);
        inherit!(self, from, thumb_isa_use);
        inherit!(self, from, fp_arch);
        inherit!(self, from, wmmx_arch);
        inherit!(self, from, asimd_arch);
        inherit!(self, from, mve_arch);
        inherit!(self, from, fp_hp_ext);
        inherit!(self, from, cpu_unaligned_access);
        inherit!(self, from, t2ee_use);
        inherit!(self, from, virtual_use);
        inherit!(self, from, mp_ext_use);
        inherit!(self, from, div_use);
        inherit!(self, from, dsp_ext);
        inherit!(self, from, pac_ext);
        inherit!(self, from, bti_ext);
        inherit!(self, from, pcs_config);
        inherit!(self, from, abi_pcs_r9_use);
        inherit!(self, from, abi_pcs_rw_data);
        inherit!(self, from, abi_pcs_ro_data);
        inherit!(self, from, abi_pcs_got_use);
        inherit!(self, from, abi_pcs_wchar_t);
        inherit!(self, from, abi_enum_size);
        inherit!(self, from, abi_align_needed);
        inherit!(self, from, abi_align_preserved);
        inherit!(self, from, abi_fp_rounding);
        inherit!(self, from, abi_fp_denormal);
        inherit!(self, from, abi_fp_exceptions);
        inherit!(self, from, abi_fp_user_exceptions);
        inherit!(self, from, abi_fp_number_model);
        inherit!(self, from, abi_fp_16bit_format);
        inherit!(self, from, abi_hardfp_use);
        inherit!(self, from, abi_vfp_args);
        inherit!(self, from, abi_wmmx_args);
        inherit!(self, from, frame_pointer_use);
        inherit!(self, from, bti_use);
        inherit!(self, from, pacret_use);
        inherit!(self, from, abi_opt_goals);
        inherit!(self, from, abi_fp_opt_goals);
        inherit!(self, from, compat);
        if self.also_compat_with.is_none() {
            self.also_compat_with.clone_from(&from.also_compat_with);
        }
        inherit!(self, from, conform);
    }

    pub fn display(&self, options: AttributeDisplayOptions) -> AttributeScopeDisplay {
        AttributeScopeDisplay { scope: self, options }
    }
}

pub struct AttributeScopeDisplay<'a> {
    scope: &'a Attributes<'a>,
    options: AttributeDisplayOptions,
}

pub struct AttributeDisplayOptions {
    pub indent: usize,
    pub show_defaults: bool,
    pub show_target: bool,
    pub show_pcs: bool,
    pub show_misc: bool,
}

impl<'a> AttributeScopeDisplay<'a> {
    fn display_field<T: Display + Default>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        field: &str,
        value: &Option<T>,
    ) -> std::fmt::Result {
        if let Some(value) = value {
            writeln!(f, "{}{} : {}", format_args!("{: >1$}", "", self.options.indent), field, value)
        } else if self.options.show_defaults {
            let value = T::default();
            writeln!(
                f,
                "{}{} : [default] {}",
                format_args!("{: >1$}", "", self.options.indent),
                field,
                value
            )
        } else {
            Ok(())
        }
    }

    fn display_quote(&self, f: &mut std::fmt::Formatter<'_>, field: &str, value: &Option<&str>) -> std::fmt::Result {
        if let Some(value) = value {
            writeln!(
                f,
                "{}{} : \"{}\"",
                format_args!("{: >1$}", "", self.options.indent),
                field,
                value
            )
        } else if self.options.show_defaults {
            writeln!(
                f,
                "{}{} : [default] \"\"",
                format_args!("{: >1$}", "", self.options.indent),
                field
            )
        } else {
            Ok(())
        }
    }
}

impl<'a> Display for AttributeScopeDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope = self.scope;
        if self.options.show_target {
            self.display_quote(f, "CPU raw name .........", &scope.cpu_raw_name)?;
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
        }
        if self.options.show_pcs {
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
        }
        if self.options.show_misc {
            self.display_field(f, "PACRET use ...........", &scope.pacret_use)?;
            self.display_field(f, "Optimization goals ...", &scope.abi_opt_goals)?;
            self.display_field(f, "FP optimization goals ", &scope.abi_fp_opt_goals)?;
            self.display_field(f, "Compatibility ........", &scope.compat)?;
            self.display_field(f, "Also compatible with .", &scope.also_compat_with)?;
            self.display_field(f, "Conformance ..........", &scope.conform)?;
        }
        Ok(())
    }
}
