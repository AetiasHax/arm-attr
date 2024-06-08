#![allow(non_upper_case_globals)]

pub const Tag_File: u8 = 1;
/// Deprecated
pub const Tag_Section: u8 = 2;
/// Deprecated
pub const Tag_Symbol: u8 = 3;
pub const Tag_CPU_raw_name: u8 = 4;
pub const Tag_CPU_name: u8 = 5;
pub const Tag_CPU_arch: u8 = 6;
pub const Tag_CPU_arch_profile: u8 = 7;
pub const Tag_ARM_ISA_use: u8 = 8;
pub const Tag_THUMB_ISA_use: u8 = 9;
pub const Tag_FP_arch: u8 = 10;
/// Deprecated: Use Tag_FP_arch instead
pub const Tag_VFP_arch: u8 = 10;
pub const Tag_WMMX_arch: u8 = 11;
pub const Tag_Advanced_SIMD_arch: u8 = 12;
pub const Tag_PCS_config: u8 = 13;
pub const Tag_ABI_PCS_R9_use: u8 = 14;
pub const Tag_ABI_PCS_RW_data: u8 = 15;
pub const Tag_ABI_PCS_RO_data: u8 = 16;
pub const Tag_ABI_PCS_GOT_use: u8 = 17;
pub const Tag_ABI_PCS_wchar_t: u8 = 18;
pub const Tag_ABI_FP_rounding: u8 = 19;
pub const Tag_ABI_FP_denormal: u8 = 20;
pub const Tag_ABI_FP_exceptions: u8 = 21;
pub const Tag_ABI_FP_user_exceptions: u8 = 22;
pub const Tag_ABI_FP_number_model: u8 = 23;
pub const Tag_ABI_align_needed: u8 = 24;
/// Deprecated: Use Tag_ABI_align_needed instead
pub const Tag_ABI_align8_needed: u8 = 24;
pub const Tag_ABI_align_preserved: u8 = 25;
/// Deprecated: Use Tag_ABI_align_preserved instead
pub const Tag_ABI_align8_preserved: u8 = 25;
pub const Tag_ABI_enum_size: u8 = 26;
pub const Tag_ABI_HardFP_use: u8 = 27;
pub const Tag_ABI_VFP_args: u8 = 28;
pub const Tag_ABI_WMMX_args: u8 = 29;
pub const Tag_ABI_optimization_goals: u8 = 30;
pub const Tag_ABI_FP_optimization_goals: u8 = 31;
pub const Tag_compatibility: u8 = 32;
pub const Tag_CPU_unaligned_access: u8 = 34;
pub const Tag_FP_HP_extension: u8 = 36;
/// Deprecated: Use Tag_FP_HP_extension instead
pub const Tag_VFP_HP_extension: u8 = 36;
pub const Tag_ABI_FP_16bit_format: u8 = 38;
pub const Tag_MPextension_use: u8 = 42;
pub const Tag_DIV_use: u8 = 44;
pub const Tag_DSP_extension: u8 = 46;
pub const Tag_MVE_arch: u8 = 48;
pub const Tag_PAC_extension: u8 = 50;
pub const Tag_BTI_extension: u8 = 52;
/// Deprecated
pub const Tag_nodefaults: u8 = 64;
pub const Tag_also_compatible_with: u8 = 65;
pub const Tag_conformance: u8 = 67;
/// Deprecated
pub const Tag_T2EE_use: u8 = 66;
pub const Tag_Virtualization_use: u8 = 68;
/// Deprecated: Use Tag_MPextension_use instead
pub const Tag_MPextension_use_OLD: u8 = 70;
pub const Tag_FramePointer_use: u8 = 72;
pub const Tag_BTI_use: u8 = 74;
pub const Tag_PACRET_use: u8 = 76;
