use std::fmt::Display;

use crate::tag::Tag;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum CpuName {
    /// ARM7EJ-S
    Arm7TejS,
    /// ARM7TM
    Arm7Tm,
    /// ARM7TDM
    Arm7Tdm,
    /// ARM7TDMI
    Arm7Tdmi,
    /// ARM710T
    Arm710T,
    /// ARM720T
    Arm720T,
    /// ARM740T
    Arm740T,
    /// ARM7TM-S
    Arm7TmS,
    /// ARM7TDMI-S
    Arm7TdmiS,
    /// ARM810
    Arm810,
    /// ARM9TDMI
    Arm9Tdmi,
    /// ARM920T
    Arm920T,
    /// ARM922T
    Arm922T,
    /// ARM940T
    Arm940T,
    /// ARM9E-S
    Arm9ES,
    /// ARM9EJ-S
    Arm9EjS,
    /// ARM926EJ-S
    Arm926EjS,
    /// ARM946E-S
    Arm946ES,
    /// ARM966E-S
    Arm966ES,
    /// ARM968E-S
    Arm968ES,
    /// ARM1020E
    Arm1020E,
    /// ARM1026EJ-S
    Arm1026EjS,
    /// ARM1136J-S
    Arm1136JS,
    /// ARM1136JF-S
    Arm1136JfS,
    /// ARM1156T2-S
    Arm1156T2S,
    /// ARM1156T2F-S
    Arm1156T2FS,
    /// ARM1176JZ-S
    Arm1176JzS,
    /// ARM1176JZF-S
    Arm1176JzfS,
    /// MPCore
    MpCore,
    /// Cortex-M0
    CortexM0,
    /// Cortex-M0plus
    CortexM0Plus,
    /// Cortex-M1
    CortexM1,
    /// Cortex-M3
    CortexM3,
    /// Cortex-M4
    CortexM4,
    /// SC000
    Sc000,
    /// SC300
    Sc300,
    /// Cortex-R4
    CortexR4,
    /// Cortex-R4F
    CortexR4F,
    /// Cortex-R5
    CortexR5,
    /// Cortex-R7
    CortexR7,
    /// Cortex-A5
    CortexA5,
    /// Cortex-A7
    CortexA7,
    /// Cortex-A8
    CortexA8,
    /// Cortex-A9
    CortexA9,
    /// Cortex-A16
    CortexA15,
    Other(String),
}

impl From<&str> for CpuName {
    fn from(value: &str) -> Self {
        match value {
            "ARM7EJ-S" => Self::Arm7TejS,
            "ARM7TM" => Self::Arm7Tm,
            "ARM7TDM" => Self::Arm7Tdm,
            "ARM7TDMI" => Self::Arm7Tdmi,
            "ARM710T" => Self::Arm710T,
            "ARM720T" => Self::Arm720T,
            "ARM740T" => Self::Arm740T,
            "ARM7TM-S" => Self::Arm7TmS,
            "ARM7TDMI-S" => Self::Arm7TdmiS,
            "ARM810" => Self::Arm810,
            "ARM9TDMI" => Self::Arm9Tdmi,
            "ARM920T" => Self::Arm920T,
            "ARM922T" => Self::Arm922T,
            "ARM940T" => Self::Arm940T,
            "ARM9E-S" => Self::Arm9ES,
            "ARM9EJ-S" => Self::Arm9EjS,
            "ARM926EJ-S" => Self::Arm926EjS,
            "ARM946E-S" => Self::Arm946ES,
            "ARM966E-S" => Self::Arm966ES,
            "ARM968E-S" => Self::Arm968ES,
            "ARM1020E" => Self::Arm1020E,
            "ARM1026EJ-S" => Self::Arm1026EjS,
            "ARM1136J-S" => Self::Arm1136JS,
            "ARM1136JF-S" => Self::Arm1136JfS,
            "ARM1156T2-S" => Self::Arm1156T2S,
            "ARM1156T2F-S" => Self::Arm1156T2FS,
            "ARM1176JZ-S" => Self::Arm1176JzS,
            "ARM1176JZF-S" => Self::Arm1176JzfS,
            "MPCore" => Self::MpCore,
            "Cortex-M0" => Self::CortexM0,
            "Cortex-M0plus" => Self::CortexM0Plus,
            "Cortex-M1" => Self::CortexM1,
            "Cortex-M3" => Self::CortexM3,
            "Cortex-M4" => Self::CortexM4,
            "SC000" => Self::Sc000,
            "SC300" => Self::Sc300,
            "Cortex-R4" => Self::CortexR4,
            "Cortex-R4F" => Self::CortexR4F,
            "Cortex-R5" => Self::CortexR5,
            "Cortex-R7" => Self::CortexR7,
            "Cortex-A5" => Self::CortexA5,
            "Cortex-A7" => Self::CortexA7,
            "Cortex-A8" => Self::CortexA8,
            "Cortex-A9" => Self::CortexA9,
            "Cortex-A16" => Self::CortexA15,
            _ => Self::Other(value.to_string()),
        }
    }
}

impl Display for CpuName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arm7TejS => write!(f, "ARM7EJ-S"),
            Self::Arm7Tm => write!(f, "ARM7TM"),
            Self::Arm7Tdm => write!(f, "ARM7TDM"),
            Self::Arm7Tdmi => write!(f, "ARM7TDMI"),
            Self::Arm710T => write!(f, "ARM710T"),
            Self::Arm720T => write!(f, "ARM720T"),
            Self::Arm740T => write!(f, "ARM740T"),
            Self::Arm7TmS => write!(f, "ARM7TM-S"),
            Self::Arm7TdmiS => write!(f, "ARM7TDMI-S"),
            Self::Arm810 => write!(f, "ARM810"),
            Self::Arm9Tdmi => write!(f, "ARM9TDMI"),
            Self::Arm920T => write!(f, "ARM920T"),
            Self::Arm922T => write!(f, "ARM922T"),
            Self::Arm940T => write!(f, "ARM940T"),
            Self::Arm9ES => write!(f, "ARM9E-S"),
            Self::Arm9EjS => write!(f, "ARM9EJ-S"),
            Self::Arm926EjS => write!(f, "ARM926EJ-S"),
            Self::Arm946ES => write!(f, "ARM946E-S"),
            Self::Arm966ES => write!(f, "ARM966E-S"),
            Self::Arm968ES => write!(f, "ARM968E-S"),
            Self::Arm1020E => write!(f, "ARM1020E"),
            Self::Arm1026EjS => write!(f, "ARM1026EJ-S"),
            Self::Arm1136JS => write!(f, "ARM1136J-S"),
            Self::Arm1136JfS => write!(f, "ARM1136JF-S"),
            Self::Arm1156T2S => write!(f, "ARM1156T2-S"),
            Self::Arm1156T2FS => write!(f, "ARM1156T2F-S"),
            Self::Arm1176JzS => write!(f, "ARM1176JZ-S"),
            Self::Arm1176JzfS => write!(f, "ARM1176JZF-S"),
            Self::MpCore => write!(f, "MPCore"),
            Self::CortexM0 => write!(f, "Cortex-M0"),
            Self::CortexM0Plus => write!(f, "Cortex-M0plus"),
            Self::CortexM1 => write!(f, "Cortex-M1"),
            Self::CortexM3 => write!(f, "Cortex-M3"),
            Self::CortexM4 => write!(f, "Cortex-M4"),
            Self::Sc000 => write!(f, "SC000"),
            Self::Sc300 => write!(f, "SC300"),
            Self::CortexR4 => write!(f, "Cortex-R4"),
            Self::CortexR4F => write!(f, "Cortex-R4F"),
            Self::CortexR5 => write!(f, "Cortex-R5"),
            Self::CortexR7 => write!(f, "Cortex-R7"),
            Self::CortexA5 => write!(f, "Cortex-A5"),
            Self::CortexA7 => write!(f, "Cortex-A7"),
            Self::CortexA8 => write!(f, "Cortex-A8"),
            Self::CortexA9 => write!(f, "Cortex-A9"),
            Self::CortexA15 => write!(f, "Cortex-A16"),
            Self::Other(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CpuArch {
    /// Pre-ARMv4
    PreV4,
    /// ARMv4
    V4,
    /// ARMv4T
    V4T,
    /// ARMv5T
    V5T,
    /// ARMv5TE
    V5TE,
    /// ARMv5TEJ
    V5TEJ,
    /// ARMv6
    V6,
    /// ARMv6KZ
    V6KZ,
    /// ARMv6T2
    V6T2,
    /// ARMv6K
    V6K,
    /// ARMv7
    V7,
    /// ARMv6-M
    V6M,
    /// ARMv6S-M
    V6SM,
    /// ARMv7E-M
    V7EM,
    /// ARMv8-A
    V8A,
    /// ARMv8-R
    V8R,
    /// ARMv8M.baseline
    V8MBaseline,
    /// ARMv8M.mainline
    V8MMainline,
    /// ARMv8.1-A
    V8_1A,
    /// ARMv8.2-A
    V8_2A,
    /// ARMv8.3-A
    V8_3A,
    /// ARMv8.1-M.mainline
    V8_1MMainline,
    /// ARMv9A
    V9A,
    Unknown(u8),
}

impl CpuArch {
    pub fn value(self) -> u8 {
        match self {
            Self::PreV4 => 0,
            Self::V4 => 1,
            Self::V4T => 2,
            Self::V5T => 3,
            Self::V5TE => 4,
            Self::V5TEJ => 5,
            Self::V6 => 6,
            Self::V6KZ => 7,
            Self::V6T2 => 8,
            Self::V6K => 9,
            Self::V7 => 10,
            Self::V6M => 11,
            Self::V6SM => 12,
            Self::V7EM => 13,
            Self::V8A => 14,
            Self::V8R => 15,
            Self::V8MBaseline => 16,
            Self::V8MMainline => 17,
            Self::V8_1A => 18,
            Self::V8_2A => 19,
            Self::V8_3A => 20,
            Self::V8_1MMainline => 21,
            Self::V9A => 22,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for CpuArch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::PreV4,
            1 => Self::V4,
            2 => Self::V4T,
            3 => Self::V5T,
            4 => Self::V5TE,
            5 => Self::V5TEJ,
            6 => Self::V6,
            7 => Self::V6KZ,
            8 => Self::V6T2,
            9 => Self::V6K,
            10 => Self::V7,
            11 => Self::V6M,
            12 => Self::V6SM,
            13 => Self::V7EM,
            14 => Self::V8A,
            15 => Self::V8R,
            16 => Self::V8MBaseline,
            17 => Self::V8MMainline,
            18 => Self::V8_1A,
            19 => Self::V8_2A,
            20 => Self::V8_3A,
            21 => Self::V8_1MMainline,
            22 => Self::V9A,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for CpuArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreV4 => write!(f, "Pre-v4"),
            Self::V4 => write!(f, "ARMv4"),
            Self::V4T => write!(f, "ARMv4T"),
            Self::V5T => write!(f, "ARMv5T"),
            Self::V5TE => write!(f, "ARMv5TE"),
            Self::V5TEJ => write!(f, "ARMv5TEJ"),
            Self::V6 => write!(f, "ARMv6"),
            Self::V6KZ => write!(f, "ARMv6KZ"),
            Self::V6T2 => write!(f, "ARMv6T2"),
            Self::V6K => write!(f, "ARMv6K"),
            Self::V7 => write!(f, "ARMv7"),
            Self::V6M => write!(f, "ARMv6-M"),
            Self::V6SM => write!(f, "ARMv6S-M"),
            Self::V7EM => write!(f, "ARMv7E-M"),
            Self::V8A => write!(f, "ARMv8-A"),
            Self::V8R => write!(f, "ARMv8-R"),
            Self::V8MBaseline => write!(f, "ARMv8-M.baseline"),
            Self::V8MMainline => write!(f, "ARMv8-M.mainline"),
            Self::V8_1A => write!(f, "ARMv8.1-A"),
            Self::V8_2A => write!(f, "ARMv8.2-A"),
            Self::V8_3A => write!(f, "ARMv8.3-A"),
            Self::V8_1MMainline => write!(f, "ARMv8.1-M.mainline"),
            Self::V9A => write!(f, "ARMv9-A"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CpuArchProfile {
    /// Architecture profile not applicable
    NotApplicable,
    /// Application profile (A)
    Application,
    /// Real-time profile (R)
    RealTime,
    /// Microcontroller profile (M)
    Microcontroller,
    /// Application or real-time profile (S)
    Classic,
    Unknown(u8),
}

impl CpuArchProfile {
    pub fn value(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::Application => b'A',
            Self::RealTime => b'R',
            Self::Microcontroller => b'M',
            Self::Classic => b'S',
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for CpuArchProfile {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotApplicable,
            b'A' => Self::Application,
            b'R' => Self::RealTime,
            b'M' => Self::Microcontroller,
            b'S' => Self::Classic,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for CpuArchProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotApplicable => Ok(()),
            Self::Application => write!(f, "A"),
            Self::RealTime => write!(f, "R"),
            Self::Microcontroller => write!(f, "M"),
            Self::Classic => write!(f, "S"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ArmIsaUse {
    /// ARM code is not allowed.
    None,
    /// ARM code is allowed.
    Allowed,
    Unknown(u8),
}

impl ArmIsaUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Allowed => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for ArmIsaUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for ArmIsaUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmIsaUse::None => write!(f, "None"),
            ArmIsaUse::Allowed => write!(f, "Allowed"),
            ArmIsaUse::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ThumbIsaUse {
    /// Thumb code is not allowed.
    None,
    /// Deprecated since ARMv8-M.baseline, use Allowed instead.
    ///
    /// 16-bit Thumb instructions are allowed, including 32-bit BL.
    Allowed16Bit,
    /// Deprecated since ARMv8-M.baseline, use Allowed instead.
    ///
    /// 32-bit and 16-bit Thumb instructions are allowed.
    Allowed32Bit,
    /// Thumb code is allowed. The set of permitted instructions can be inferred from CPU arch and CPU arch profile.
    Allowed,
    Unknown(u8),
}

impl ThumbIsaUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Allowed16Bit => 1,
            Self::Allowed32Bit => 2,
            Self::Allowed => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for ThumbIsaUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Allowed16Bit,
            2 => Self::Allowed32Bit,
            3 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for ThumbIsaUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Allowed16Bit => write!(f, "16-bit allowed, including BL"),
            Self::Allowed32Bit => write!(f, "32-bit allowed"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FpArch {
    /// No instructions requiring FP hardware.
    None,
    /// v1 FP ISA is allowed.
    V1,
    /// v2 FP ISA is allowed. Implies use of v1.
    V2,
    /// v3 FP ISA is allowed. Implies use of v2.
    V3,
    /// v3 FP ISA is allowed, but only registers D0-D15 and S0-S31
    V3Light,
    /// v4 FP ISA is allowed. Implies use of v3.
    V4,
    /// v4 FP ISA is allowed, but only registers D0-D15 and S0-S31
    V4Light,
    /// ARMv8-A FP ISA is allowed.
    V8A,
    /// ARMv8-A FP ISA is allowed, but only registers D0-D15 and S0-S31
    V8ALight,
    Unknown(u8),
}

impl FpArch {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::V1 => 1,
            Self::V2 => 2,
            Self::V3 => 3,
            Self::V3Light => 4,
            Self::V4 => 5,
            Self::V4Light => 6,
            Self::V8A => 7,
            Self::V8ALight => 8,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for FpArch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V3,
            4 => Self::V3Light,
            5 => Self::V4,
            6 => Self::V4Light,
            7 => Self::V8A,
            8 => Self::V8ALight,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for FpArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::V1 => write!(f, "v1"),
            Self::V2 => write!(f, "v2"),
            Self::V3 => write!(f, "v3"),
            Self::V3Light => write!(f, "v3 light"),
            Self::V4 => write!(f, "v4"),
            Self::V4Light => write!(f, "v4 light"),
            Self::V8A => write!(f, "ARMv8-A"),
            Self::V8ALight => write!(f, "ARMv8-A light"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum WmmxArch {
    /// Not allowed.
    None,
    /// WMMX v1 allowed.
    V1,
    /// WMMX v2 allowed.
    V2,
    Unknown(u8),
}

impl WmmxArch {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::V1 => 1,
            Self::V2 => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for WmmxArch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::V1,
            2 => Self::V2,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for WmmxArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::V1 => write!(f, "v1"),
            Self::V2 => write!(f, "v2"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AsimdArch {
    /// Not allowed.
    None,
    /// Advanced SIMDv1 (Neon) allowed.
    V1,
    /// Advanced SIMDv2 (Neon) allowed.
    V2,
    /// ARMv8-A Advanced SIMD (Neon) allowed.
    V8A,
    /// ARMv8.1-A Advanced SIMD (Neon) allowed.
    V8_1A,
    Unknown(u8),
}

impl AsimdArch {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::V1 => 1,
            Self::V2 => 2,
            Self::V8A => 3,
            Self::V8_1A => 4,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AsimdArch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V8A,
            4 => Self::V8_1A,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AsimdArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::V1 => write!(f, "SIMDv1"),
            Self::V2 => write!(f, "SIMDv2"),
            Self::V8A => write!(f, "ARMv8-A"),
            Self::V8_1A => write!(f, "ARMv8.1-A"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MveArch {
    /// Not allowed.
    None,
    /// Integer M-profile Vector Extension allowed.
    Int,
    /// Integer and Floating Point M-profile Vector Extension allowed.
    IntFloat,
    Unknown(u8),
}

impl MveArch {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Int => 1,
            Self::IntFloat => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for MveArch {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Int,
            2 => Self::IntFloat,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for MveArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Int => write!(f, "Integer"),
            Self::IntFloat => write!(f, "Integer and Float"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FpHpExt {
    /// Half-precision allowed if they exist in the instructions sets indicated by `FpArch` and `AsimdArch`.
    IfExists,
    /// Half-precision allowed as an extension to VFPv3 and ASIMDv1, in addition to those indicated by `FpArch` and
    /// `AsimdArch`.
    VfpV3,
    /// Half-precision allowed as an extension to ARMv8.2-A and ASIMD, in addition to those indicated by `FpArch` and
    /// `AsimdArch`.
    ArmV8_2A,
    Unknown(u8),
}

impl FpHpExt {
    pub fn value(self) -> u8 {
        match self {
            Self::IfExists => 0,
            Self::VfpV3 => 1,
            Self::ArmV8_2A => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for FpHpExt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::IfExists,
            1 => Self::VfpV3,
            2 => Self::ArmV8_2A,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for FpHpExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfExists => write!(f, "If exists"),
            Self::VfpV3 => write!(f, "VFPv3"),
            Self::ArmV8_2A => write!(f, "ARMv8.2-A"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CpuUnalignedAccess {
    /// Not allowed.
    None,
    /// v6-style unaligned data accesses allowed.
    Allowed,
    Unknown(u8),
}

impl CpuUnalignedAccess {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Allowed => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for CpuUnalignedAccess {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for CpuUnalignedAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum T2EeUse {
    /// Not allowed.
    None,
    /// The T2EE extension is allowed.
    Allowed,
    Unknown(u8),
}

impl T2EeUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Allowed => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for T2EeUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for T2EeUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum VirtualUse {
    /// Not allowed.
    None,
    /// The TrustZone extension is allowed.
    TrustZone,
    /// Virtualization extensions (HVC, ERET) are allowed.
    VExts,
    /// The TrustZone and virtualization extensions are allowed.
    TrustZoneVExts,
    Unknown(u8),
}

impl VirtualUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::TrustZone => 1,
            Self::VExts => 2,
            Self::TrustZoneVExts => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for VirtualUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::TrustZone,
            2 => Self::VExts,
            3 => Self::TrustZoneVExts,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for VirtualUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::TrustZone => write!(f, "TrustZone"),
            Self::VExts => write!(f, "Virtual exts"),
            Self::TrustZoneVExts => write!(f, "TrustZone and virtual exts"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MpExtUse {
    // Not allowed.
    None,
    // Use of the ARMv7 MP extension is allowed.
    Allowed,
    Unknown(u8),
}

impl MpExtUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Allowed => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for MpExtUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for MpExtUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DivUse {
    /// Divide instructions may be used if they exist in the architecture, as indicated by `CpuArch` and `CpuArchProfile`.
    IfExists,
    /// Not allowed.
    None,
    /// Allowed.
    Allowed,
    Unknown(u8),
}

impl DivUse {
    pub fn value(self) -> u8 {
        match self {
            Self::IfExists => 0,
            Self::None => 1,
            Self::Allowed => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for DivUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::IfExists,
            1 => Self::None,
            2 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for DivUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfExists => write!(f, "If exists"),
            Self::None => write!(f, "None"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum DspExt {
    /// DSP instructions may be used if they exist in the architecture, as indicated by `CpuArch`.
    IfExists,
    /// DSP instructions are allowed.
    Allowed,
    Unknown(u8),
}

impl DspExt {
    pub fn value(self) -> u8 {
        match self {
            Self::IfExists => 0,
            Self::Allowed => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for DspExt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::IfExists,
            1 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for DspExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IfExists => write!(f, "If exists"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PacExt {
    /// Not allowed.
    None,
    /// PAC/AUT instructions are allowed in the NOP space.
    OnlyNopSpace,
    /// PAC/AUT instructions are allowed.
    Allowed,
    Unknown(u8),
}

impl PacExt {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::OnlyNopSpace => 1,
            Self::Allowed => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for PacExt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::OnlyNopSpace,
            2 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for PacExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::OnlyNopSpace => write!(f, "Only NOP space"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BtiExt {
    /// Not allowed.
    None,
    /// BTI instructions are allowed in the NOP space.
    OnlyNopSpace,
    /// BTI instructions are allowed.
    Allowed,
    Unknown(u8),
}

impl BtiExt {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::OnlyNopSpace => 1,
            Self::Allowed => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for BtiExt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::OnlyNopSpace,
            2 => Self::Allowed,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for BtiExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::OnlyNopSpace => write!(f, "Only NOP space"),
            Self::Allowed => write!(f, "Allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

/// Procedure-call standard (PCS) configuration
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PcsConfig {
    /// No standard configuration used.
    None,
    BarePlatform,
    LinuxApplication,
    LinuxDso,
    PalmOs2004,
    /// Reserved for future Palm OS configuration.
    PalmOsFuture,
    SymbianOs2004,
    /// Reserved for future Symbian OS configuration.
    SymbianOsFuture,
    Unknown(u8),
}

impl PcsConfig {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::BarePlatform => 1,
            Self::LinuxApplication => 2,
            Self::LinuxDso => 3,
            Self::PalmOs2004 => 4,
            Self::PalmOsFuture => 5,
            Self::SymbianOs2004 => 6,
            Self::SymbianOsFuture => 7,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for PcsConfig {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::BarePlatform,
            2 => Self::LinuxApplication,
            3 => Self::LinuxDso,
            4 => Self::PalmOs2004,
            5 => Self::PalmOsFuture,
            6 => Self::SymbianOs2004,
            7 => Self::SymbianOsFuture,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for PcsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::BarePlatform => write!(f, "Bare platform"),
            Self::LinuxApplication => write!(f, "Linux application"),
            Self::LinuxDso => write!(f, "Linux DSO"),
            Self::PalmOs2004 => write!(f, "Palm OS 2004"),
            Self::PalmOsFuture => write!(f, "Future Palm OS"),
            Self::SymbianOs2004 => write!(f, "Symbian OS 2004"),
            Self::SymbianOsFuture => write!(f, "Future Symbian OS"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiPcsR9Use {
    /// R9 used as V6, a callee-saved register.
    V6,
    /// R9 used as SB, a global Static Base register.
    Sb,
    /// R9 used as a Thread Local Storage (TLS) pointer.
    TlsPointer,
    /// R9 not used.
    None,
    Unknown(u8),
}

impl AbiPcsR9Use {
    pub fn value(self) -> u8 {
        match self {
            Self::V6 => 0,
            Self::Sb => 1,
            Self::TlsPointer => 2,
            Self::None => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiPcsR9Use {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::V6,
            1 => Self::Sb,
            2 => Self::TlsPointer,
            3 => Self::None,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiPcsR9Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V6 => write!(f, "V6"),
            Self::Sb => write!(f, "SB"),
            Self::TlsPointer => write!(f, "TLS pointer"),
            Self::None => write!(f, "None"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiPcsRwData {
    /// RW static data is allowed to be addressed absolutely.
    Abs,
    /// RW static data is allowed to be addressed PC-relative.
    PcRel,
    /// RW static data is allowed to be addressed SB-relative.
    SbRel,
    /// Not allowed.
    None,
    Unknown(u8),
}

impl AbiPcsRwData {
    pub fn value(self) -> u8 {
        match self {
            Self::Abs => 0,
            Self::PcRel => 1,
            Self::SbRel => 2,
            Self::None => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiPcsRwData {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Abs,
            1 => Self::PcRel,
            2 => Self::SbRel,
            3 => Self::None,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiPcsRwData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abs => write!(f, "Absolute"),
            Self::PcRel => write!(f, "PC-relative"),
            Self::SbRel => write!(f, "SB-relative"),
            Self::None => write!(f, "Not allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiPcsRoData {
    /// RO static data is allowed to be addressed absolutely.
    Abs,
    /// RO static data is allowed to be addressed PC-relative.
    PcRel,
    /// Not allowed.
    None,
    Unknown(u8),
}

impl AbiPcsRoData {
    pub fn value(self) -> u8 {
        match self {
            Self::Abs => 0,
            Self::PcRel => 1,
            Self::None => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiPcsRoData {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Abs,
            1 => Self::PcRel,
            2 => Self::None,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiPcsRoData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Abs => write!(f, "Absolute"),
            Self::PcRel => write!(f, "PC-relative"),
            Self::None => write!(f, "Not allowed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiPcsGotUse {
    /// Not allowed.
    None,
    /// Imported data is allowed to be addressed directly.
    Direct,
    /// Imported data is allowed to be addressed indirectly.
    Indirect,
    Unknown(u8),
}

impl AbiPcsGotUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Direct => 1,
            Self::Indirect => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiPcsGotUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Direct,
            2 => Self::Indirect,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiPcsGotUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Not allowed"),
            Self::Direct => write!(f, "Direct"),
            Self::Indirect => write!(f, "Indirect"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiPcsWcharT {
    /// Not allowed.
    None,
    /// wchar_t of size 2 allowed.
    Size2,
    /// wchar_T of size 4 allowed.
    Size4,
    Unknown(u8),
}

impl AbiPcsWcharT {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Size2 => 2,
            Self::Size4 => 4,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiPcsWcharT {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            2 => Self::Size2,
            4 => Self::Size4,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiPcsWcharT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Not allowed"),
            Self::Size2 => write!(f, "16-bit"),
            Self::Size4 => write!(f, "32-bit"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiEnumSize {
    /// Not allowed.
    None,
    /// Enums are as small as possible while big enough to contain all values.
    SmallestSize,
    /// Enums are always 32-bits.
    Always32,
    /// Enums visible across an ABI-complying interface are 32-bits.
    Visible32,
    Unknown(u8),
}

impl AbiEnumSize {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::SmallestSize => 1,
            Self::Always32 => 2,
            Self::Visible32 => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiEnumSize {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::SmallestSize,
            2 => Self::Always32,
            3 => Self::Visible32,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiEnumSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Not allowed"),
            Self::SmallestSize => write!(f, "Smallest size"),
            Self::Always32 => write!(f, "32-bit"),
            Self::Visible32 => write!(f, "32-bit when exposed"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiAlignNeeded {
    /// Code does not depend on alignment of 8-byte data or data with extended alignment (> 8-byte).
    None,
    /// Code depends on 8-byte alignment of 8-byte data.
    Align8,
    /// Code depends on 4-byte alignment of 8-byte data.
    Align4,
    /// Reserved.
    Reserved,
    /// Code depends on 8-byte alignment of 8-byte data and alignment of data having up to 2n-byte extended alignment, where n
    /// is in \[4..12\].
    Align2n(u8),
    Unknown(u8),
}

impl AbiAlignNeeded {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Align8 => 1,
            Self::Align4 => 2,
            Self::Reserved => 3,
            Self::Align2n(value) => value,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiAlignNeeded {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Align8,
            2 => Self::Align4,
            3 => Self::Reserved,
            4..=12 => Self::Align2n(value),
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiAlignNeeded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Align8 => write!(f, "8-byte"),
            Self::Align4 => write!(f, "4-byte"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Align2n(value) => write!(f, "8-byte, {}-byte extended", 2 * value),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiAlignPreserved {
    /// Code does not preserve alignment of 8-byte data or data with extended alignment (> 8-byte).
    None,
    /// Code preserves 8-byte alignment of 8-byte data.
    Align8,
    /// Code preserves 4-byte alignment of 8-byte data.
    Align4,
    /// Reserved.
    Reserved,
    /// Code preserves 8-byte alignment of 8-byte data and alignmentt of data having up to 2n-byte extended alignment, where n
    /// is in \[4..12\].
    Align2n(u8),
    Unknown(u8),
}

impl AbiAlignPreserved {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Align8 => 1,
            Self::Align4 => 2,
            Self::Reserved => 3,
            Self::Align2n(value) => value,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiAlignPreserved {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Align8,
            2 => Self::Align4,
            3 => Self::Reserved,
            4..=12 => Self::Align2n(value),
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiAlignPreserved {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Align8 => write!(f, "8-byte"),
            Self::Align4 => write!(f, "4-byte"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Align2n(value) => write!(f, "8-byte, {}-byte extended", 2 * value),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpRounding {
    /// The code uses IEEE-754 round-to-nearest rounding mode.
    Nearest,
    /// The code decides IEEE-754 rounding modea at run-time.
    RunTime,
    Unknown(u8),
}

impl AbiFpRounding {
    pub fn value(self) -> u8 {
        match self {
            Self::Nearest => 0,
            Self::RunTime => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpRounding {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Nearest,
            1 => Self::RunTime,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpRounding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nearest => write!(f, "Nearest"),
            Self::RunTime => write!(f, "Decides at run-time"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpDenormal {
    /// Denormal numbers might be flushed to (+) zero.
    DontCare,
    /// The code depends on IEEE-754 denormal numbers.
    Ieee754,
    /// The code depends on the sign being preserved when flushed to zero.
    PreserveSign,
    Unknown(u8),
}

impl AbiFpDenormal {
    pub fn value(self) -> u8 {
        match self {
            Self::DontCare => 0,
            Self::Ieee754 => 1,
            Self::PreserveSign => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpDenormal {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DontCare,
            1 => Self::Ieee754,
            2 => Self::PreserveSign,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpDenormal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DontCare => write!(f, "Don't care"),
            Self::Ieee754 => write!(f, "Depends on IEEE-754 denormal numbers"),
            Self::PreserveSign => write!(f, "Preserve sign"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpExceptions {
    /// The code should not check for inexact results.
    None,
    /// The code checks for the IEEE-754 inexact exception.
    CheckInexact,
    Unknown(u8),
}

impl AbiFpExceptions {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::CheckInexact => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpExceptions {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::CheckInexact,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpExceptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::CheckInexact => write!(f, "IEEE-754 inexact exception"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpUserExceptions {
    /// The code should not enable or use IEEE-754 user exceptions.
    None,
    /// The code uses IEEE-754 user exceptions.
    Enabled,
    Unknown(u8),
}

impl AbiFpUserExceptions {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Enabled => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpUserExceptions {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Enabled,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpUserExceptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpNumberModel {
    /// The code does not use floating point numbers.
    None,
    /// The code allows the IEEE-754 normal numbers only.
    Normal,
    /// The code allows normal numbers, infinities, and one quiet NaN.
    InfNaN,
    /// The code allows all float encodings defined by IEEE-754.
    All,
    Unknown(u8),
}

impl AbiFpNumberModel {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Normal => 1,
            Self::InfNaN => 2,
            Self::All => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpNumberModel {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Normal,
            2 => Self::InfNaN,
            3 => Self::All,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpNumberModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Normal => write!(f, "Normal numbers"),
            Self::InfNaN => write!(f, "Normal numbers, infinities and NaN"),
            Self::All => write!(f, "All numbers"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFp16BitFormat {
    /// The code does not use 16-bit floats.
    None,
    /// The code uses the IEEE-754 format for 16-bit floats.
    Ieee754,
    /// The code uses the VFPv3/ASIMD alternative format for 16-bit floats.
    Alternative,
    Unknown(u8),
}

impl AbiFp16BitFormat {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Ieee754 => 1,
            Self::Alternative => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFp16BitFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Ieee754,
            2 => Self::Alternative,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFp16BitFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Ieee754 => write!(f, "IEEE-754 format"),
            Self::Alternative => write!(f, "Alternative format"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiHardFpUse {
    /// Floating point use is implied by `FpArch`.
    Implied,
    /// The code executes on the single-precision variant derived from `FpArch`.
    DerivedSingle,
    /// Reserved.
    Reserved,
    /// Deprecated: Use `Implied` instead.
    ImpliedOld,
    Unknown(u8),
}

impl AbiHardFpUse {
    pub fn value(self) -> u8 {
        match self {
            Self::Implied => 0,
            Self::DerivedSingle => 1,
            Self::Reserved => 2,
            Self::ImpliedOld => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiHardFpUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Implied,
            1 => Self::DerivedSingle,
            2 => Self::Reserved,
            3 => Self::ImpliedOld,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiHardFpUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Implied => write!(f, "Implied"),
            Self::DerivedSingle => write!(f, "Derived single-precision"),
            Self::Reserved => write!(f, "Reserved"),
            Self::ImpliedOld => write!(f, "Implied (deprecated)"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiVfpArgs {
    /// Float parameter/result passing conforms to AAPCS, base variant.
    Base,
    /// Float parameter/result passing conforms to AAPCS, VFP variant.
    Vfp,
    /// Float parameter/result passing conforms to toolchain-specific conventions.
    Toolchain,
    /// Code is compatible with both the base and VFP variants. Non-variadic functions are not allowed to pass float
    /// parameters/results.
    BaseVfp,
    Unknown(u8),
}

impl AbiVfpArgs {
    pub fn value(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::Vfp => 1,
            Self::Toolchain => 2,
            Self::BaseVfp => 3,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiVfpArgs {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Base,
            1 => Self::Vfp,
            2 => Self::Toolchain,
            3 => Self::BaseVfp,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiVfpArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "Base variant"),
            Self::Vfp => write!(f, "VFP variant"),
            Self::Toolchain => write!(f, "Toolchain-specific"),
            Self::BaseVfp => write!(f, "Base and VFP variants"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiWmmxArgs {
    /// WMMX parameter/result passing conforms to AAPCS, base variant.
    Base,
    /// WMMX parameter/result passing conforms to Intel's WMMX conventions.
    Intel,
    /// WMMX parameter/result passing conforms to toolchain-specific conventions.
    Toolchain,
    Unknown(u8),
}

impl AbiWmmxArgs {
    pub fn value(self) -> u8 {
        match self {
            Self::Base => 0,
            Self::Intel => 1,
            Self::Toolchain => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiWmmxArgs {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Base,
            1 => Self::Intel,
            2 => Self::Toolchain,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiWmmxArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base => write!(f, "Base variant"),
            Self::Intel => write!(f, "Intel conventions"),
            Self::Toolchain => write!(f, "Toolchain-specific"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum FramePointerUse {
    /// The code does not conform to the rules of using the frame pointer.
    None,
    /// The code creates a frame record for all functions that can modify the link register (LR).
    WithRecords,
    /// The codes does not create records, but preserves the frame pointer register (FP) value.
    WithoutRecords,
    Unknown(u8),
}

impl FramePointerUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::WithRecords => 1,
            Self::WithoutRecords => 2,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for FramePointerUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::WithRecords,
            2 => Self::WithoutRecords,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for FramePointerUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::WithRecords => write!(f, "With frame records"),
            Self::WithoutRecords => write!(f, "Without frame records"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum BtiUse {
    /// The code is compiled without branch target enforcement.
    None,
    /// The code is compiled with branch target enforcement.
    Enabled,
    Unknown(u8),
}

impl BtiUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Enabled => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for BtiUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Enabled,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for BtiUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum PacretUse {
    /// The code is compiled without return address signing and authentication.
    None,
    /// The code is compiled with return address signing and authentication.
    Enabled,
    Unknown(u8),
}

impl PacretUse {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Enabled => 1,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for PacretUse {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::Enabled,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for PacretUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Enabled => write!(f, "Enabled"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiOptGoals {
    /// The code has no optimization goals.
    None,
    /// Favor speed, preserve small size and debug experience.
    FavorSpeed,
    /// Optimize aggressively for speed, sacrifice small size and debug experience.
    OptimizeSpeed,
    /// Favor size, preserve for speed and debug experience.
    FavorSize,
    /// Optimize aggressively for size, sacrifice speed and debug experience.
    OptimizeSize,
    /// Favor debug experience, preserve speed and small size.
    FavorDebug,
    /// Optimize aggressively for debug experience, but sacrifice speed and small size.
    OptimizeDebug,
    Unknown(u8),
}

impl AbiOptGoals {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::FavorSpeed => 1,
            Self::OptimizeSpeed => 2,
            Self::FavorSize => 3,
            Self::OptimizeSize => 4,
            Self::FavorDebug => 5,
            Self::OptimizeDebug => 6,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiOptGoals {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::FavorSpeed,
            2 => Self::OptimizeSpeed,
            3 => Self::FavorSize,
            4 => Self::OptimizeSize,
            5 => Self::FavorDebug,
            6 => Self::OptimizeDebug,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiOptGoals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::FavorSpeed => write!(f, "Favor speed, preserve size and debug"),
            Self::OptimizeSpeed => write!(f, "Optimize speed, sacrifice size and debug"),
            Self::FavorSize => write!(f, "Favor size, preserve speed and debug"),
            Self::OptimizeSize => write!(f, "Optimize size, sacrifice size and debug"),
            Self::FavorDebug => write!(f, "Favor debug, preserve speed and size"),
            Self::OptimizeDebug => write!(f, "Optimize debug, sacrifice speed and size"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum AbiFpOptGoals {
    /// The code has no FP optimization goals.
    None,
    /// Favor speed, preserve small size and accuracy.
    FavorSpeed,
    /// Optimize aggressively for speed, sacrifice small size and accuracy.
    OptimizeSpeed,
    /// Favor size, preserve for speed and accuracy.
    FavorSize,
    /// Optimize aggressively for size, sacrifice speed and accuracy.
    OptimizeSize,
    /// Favor accuracy, preserve speed and small size.
    FavorAccuracy,
    /// Optimize aggressively for accuracy, but sacrifice speed and small size.
    OptimizeAccuracy,
    Unknown(u8),
}

impl AbiFpOptGoals {
    pub fn value(self) -> u8 {
        match self {
            Self::None => 0,
            Self::FavorSpeed => 1,
            Self::OptimizeSpeed => 2,
            Self::FavorSize => 3,
            Self::OptimizeSize => 4,
            Self::FavorAccuracy => 5,
            Self::OptimizeAccuracy => 6,
            Self::Unknown(value) => value,
        }
    }
}

impl From<u8> for AbiFpOptGoals {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::FavorSpeed,
            2 => Self::OptimizeSpeed,
            3 => Self::FavorSize,
            4 => Self::OptimizeSize,
            5 => Self::FavorAccuracy,
            6 => Self::OptimizeAccuracy,
            _ => Self::Unknown(value),
        }
    }
}

impl Display for AbiFpOptGoals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::FavorSpeed => write!(f, "Favor speed, preserve size and accuracy"),
            Self::OptimizeSpeed => write!(f, "Optimize speed, sacrifice size and accuracy"),
            Self::FavorSize => write!(f, "Favor size, preserve speed and accuracy"),
            Self::OptimizeSize => write!(f, "Optimize size, sacrifice size and accuracy"),
            Self::FavorAccuracy => write!(f, "Favor accuracy, preserve speed and size"),
            Self::OptimizeAccuracy => write!(f, "Optimize accuracy, sacrifice speed and size"),
            Self::Unknown(value) => write!(f, "<unknown: {:#x}>", value),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Compat {
    /// This entity has no toolchain-specific requirements.
    Always,
    /// This entity conforms to the ABI when built by the given toolchain.
    ByToolchain(String),
    /// This entity does not conform to the ABI, but can be used privately by given flag and vendor name.
    Private { flag: u8, vendor: String },
}

impl Compat {
    pub fn new(flag: u8, vendor: String) -> Self {
        match flag {
            0 => Self::Always,
            1 => Self::ByToolchain(vendor),
            _ => Self::Private { flag, vendor },
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Always => 0,
            Self::ByToolchain(_) => 1,
            Self::Private { flag, vendor: _ } => *flag,
        }
    }
}

impl Display for Compat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "Always"),
            Self::ByToolchain(toolchain) => write!(f, "By toolchain '{}'", toolchain),
            Self::Private { flag, vendor } => write!(f, "Private, flag = {}, vendor = '{}'", flag, vendor),
        }
    }
}

pub enum AlsoCompatWith {
    Arch(CpuArch),
    Unknown(Tag),
}

impl AlsoCompatWith {
    pub fn new(tag: Tag) -> Self {
        match tag {
            Tag::CpuArch(arch) => Self::Arch(CpuArch::from(arch)),
            _ => Self::Unknown(tag),
        }
    }
}

impl Display for AlsoCompatWith {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arch(arch) => write!(f, "{}", arch),
            Self::Unknown(tag) => write!(f, "<unknown: {:?}>", tag),
        }
    }
}

pub struct Conform(pub String);

impl Display for Conform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
