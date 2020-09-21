use libnspire_sys::{
    nspire_battery, nspire_devinfo, nspire_devinfo__bindgen_ty_3, nspire_devinfo__bindgen_ty_5,
    nspire_runlevel, nspire_type,
};
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum HardwareType {
    Cas,
    NonCas,
    CasCx,
    NonCasCx,
    Unknown(u8),
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum RunLevel {
    Recovery,
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
        write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, self.build)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Lcd {
    pub width: u16,
    pub height: u16,
    pub bbp: u8,
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
            bbp,
            sample_mode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub free_storage: u64,
    pub total_storage: u64,
    pub free_ram: u64,
    pub total_ram: u64,
    pub version: Version,
    pub boot1_version: Version,
    pub boot2_version: Version,
    pub hw_type: HardwareType,
    pub clock_speed: u8,
    pub lcd: Lcd,
    pub os_extension: String,
    pub file_extension: String,
    pub name: String,
    pub id: String,
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
