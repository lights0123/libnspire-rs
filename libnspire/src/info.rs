//! Information about the calculator

use libnspire_sys::{
    nspire_battery, nspire_devinfo, nspire_devinfo__bindgen_ty_3, nspire_devinfo__bindgen_ty_5,
    nspire_runlevel, nspire_type,
};
#[cfg(feature = "serde")]
use serde::Serialize;
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum HardwareType {
    Cas,
    NonCas,
    CasCx,
    NonCasCx,
    Unknown(u8),
}

impl HardwareType {
    /// Whether this model is a CAS model. This is the physical model, not the
    /// software model: if CAS software has been installed on a non-CAS device,
    /// this will still return `false`. Use the [name][crate::info::Info::name]
    /// field and search for "CAS" instead.
    pub fn is_cas(&self) -> bool {
        matches!(self, HardwareType::Cas | HardwareType::CasCx)
    }
    /// Whether this model is a CX or CX II model. Use
    /// [`Handle::is_cx_ii`][crate::Handle::is_cx_ii] to determine if the
    /// device is a CX II or not.
    pub fn is_cx(&self) -> bool {
        matches!(self, HardwareType::CasCx | HardwareType::NonCasCx)
    }
}

impl From<nspire_type> for HardwareType {
    fn from(hw_type: nspire_type) -> Self {
        use libnspire_sys::*;
        #[forbid(unreachable_patterns)]
        #[allow(non_upper_case_globals)]
        match hw_type {
            nspire_type_NSPIRE_CAS => HardwareType::Cas,
            nspire_type_NSPIRE_CASCX => HardwareType::CasCx,
            nspire_type_NSPIRE_NONCAS => HardwareType::NonCas,
            nspire_type_NSPIRE_NONCASCX => HardwareType::NonCasCx,
            v => HardwareType::Unknown(v as u8),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Battery {
    Powered,
    Low,
    Ok,
    Unknown(u8),
}

impl From<nspire_battery> for Battery {
    fn from(hw_type: nspire_battery) -> Self {
        use libnspire_sys::*;
        #[forbid(unreachable_patterns)]
        #[allow(non_upper_case_globals)]
        match hw_type {
            nspire_battery_NSPIRE_BATT_POWERED => Battery::Powered,
            nspire_battery_NSPIRE_BATT_OK => Battery::Ok,
            nspire_battery_NSPIRE_BATT_LOW => Battery::Low,
            v => Battery::Unknown(v as u8),
        }
    }
}

/// The current state of the calculator.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RunLevel {
    /// The calculator is in recovery mode.
    Recovery,
    /// The calculator is in the standard operating system.
    Os,
    Unknown(u8),
}

impl From<nspire_runlevel> for RunLevel {
    fn from(hw_type: nspire_runlevel) -> Self {
        use libnspire_sys::*;
        #[forbid(unreachable_patterns)]
        #[allow(non_upper_case_globals)]
        match hw_type {
            nspire_runlevel_NSPIRE_RUNLEVEL_RECOVERY => RunLevel::Recovery,
            nspire_runlevel_NSPIRE_RUNLEVEL_OS => RunLevel::Os,
            v => RunLevel::Unknown(v as u8),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl From<nspire_devinfo__bindgen_ty_3> for Version {
    fn from(
        nspire_devinfo__bindgen_ty_3 {
            major,
            minor,
            build,
        }: nspire_devinfo__bindgen_ty_3,
    ) -> Self {
        Version {
            major,
            minor: minor / 10,
            patch: minor % 10,
            build,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.major, self.minor, self.patch, self.build
        )
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Lcd {
    pub width: u16,
    pub height: u16,
    /// The number of bits per pixel. Either 8 for non-color calculators or 16
    /// for color calculators.
    pub bpp: u8,
    pub sample_mode: u8,
}

impl From<nspire_devinfo__bindgen_ty_5> for Lcd {
    fn from(
        nspire_devinfo__bindgen_ty_5 {
            width,
            height,
            bbp,
            sample_mode,
        }: nspire_devinfo__bindgen_ty_5,
    ) -> Self {
        Lcd {
            width,
            height,
            bpp: bbp,
            sample_mode,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Info {
    pub free_storage: u64,
    pub total_storage: u64,
    pub free_ram: u64,
    pub total_ram: u64,
    /// The operating system version.
    pub version: Version,
    pub boot1_version: Version,
    pub boot2_version: Version,
    pub hw_type: HardwareType,
    pub clock_speed: u8,
    pub lcd: Lcd,
    /// The accepted file extension for OS upgrades.
    pub os_extension: String,
    /// The accepted file extension for files.
    pub file_extension: String,
    /// The name of the calculator.
    pub name: String,
    /// The ID ("serial number") of the calculator.
    pub id: String,
    /// Whether the calculator is in maintenance mode or the standard operating
    /// system.
    pub run_level: RunLevel,
    pub battery: Battery,
    pub is_charging: bool,
}

impl From<nspire_devinfo> for Info {
    fn from(info: nspire_devinfo) -> Self {
        unsafe {
            Info {
                free_storage: info.storage.free,
                total_storage: info.storage.total,
                free_ram: info.ram.free,
                total_ram: info.ram.total,
                version: info.versions[0].into(),
                boot1_version: info.versions[1].into(),
                boot2_version: info.versions[2].into(),
                hw_type: info.hw_type.into(),
                clock_speed: info.clock_speed,
                lcd: info.lcd.into(),
                battery: info.batt.status.into(),
                file_extension: crate::c_str(&info.extensions.file),
                os_extension: crate::c_str(&info.extensions.os),
                name: crate::c_str(&info.device_name),
                id: crate::c_str(&info.electronic_id),
                run_level: info.runlevel.into(),
                is_charging: info.batt.is_charging > 0,
            }
        }
    }
}
