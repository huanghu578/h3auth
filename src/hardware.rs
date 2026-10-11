//! hardware.rs
//! Hardware information collection module
//! Collects raw hardware identifiers (system UUID, baseboard serial, BIOS serial)
//! used as the basis for machine-bound authentication.
use super::error::{Result, AuthError};

#[cfg(windows)]
use wmi::{COMLibrary, WMIConnection};
#[cfg(windows)]
use serde::Deserialize;

/// Raw hardware identifiers gathered from the machine.
///
/// Each field falls back to `"[EMPTY]"` when the underlying query fails,
/// so a value of `"[EMPTY]"` indicates the identifier could not be read
/// (or was filtered out as a known placeholder value).
#[derive(Debug, Clone)]
pub struct HardwareRawInfo {
    /// UUID from `Win32_ComputerSystemProduct`.
    pub system_uuid: String,
    /// Serial number from `Win32_BaseBoard`.
    pub baseboard_serial: String,
    /// Serial number from `Win32_BIOS`.
    pub bios_serial: String,
}

impl HardwareRawInfo {
    /// Collects all hardware identifiers, substituting `"[EMPTY]"` on failure.
    ///
    /// This function never fails; errors from individual queries are swallowed
    /// and replaced with the placeholder string.
    pub fn collect() -> Self {
        let system_uuid = Self::get_system_uuid().unwrap_or_else(|_| "[EMPTY]".into());
        let baseboard_serial = Self::get_board_sn().unwrap_or_else(|_| "[EMPTY]".into());
        let bios_serial = Self::get_bios_sn().unwrap_or_else(|_| "[EMPTY]".into());
        Self { system_uuid, baseboard_serial, bios_serial }
    }

    /// Serializes the identifiers into a NUL-separated raw string.
    ///
    /// The NUL (`\0`) separator prevents ambiguity when individual fields
    /// could otherwise be concatenated in multiple ways.
    pub fn to_raw_string(&self) -> String {
        format!("{}\0{}\0{}", self.system_uuid, self.baseboard_serial, self.bios_serial)
    }

    /// Queries the system UUID via WMI (`Win32_ComputerSystemProduct.UUID`).
    ///
    /// Rejects empty strings and the all-zero UUID, which indicate an
    /// unpopulated or invalid value.
    #[cfg(windows)]
    fn get_system_uuid() -> Result<String> {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct Win32ComputerSystemProduct { uuid: Option<String> }

        let com = COMLibrary::new().map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let con = WMIConnection::new(com).map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let results: Vec<Win32ComputerSystemProduct> = con
            .raw_query("SELECT UUID FROM Win32_ComputerSystemProduct")
            .map_err(|e| AuthError::CmdFail(e.to_string()))?;
        results.into_iter().next()
            .and_then(|r| r.uuid)
            .filter(|s| !s.is_empty() && s != "00000000-0000-0000-0000-000000000000")
            .ok_or(AuthError::RegexMatch)
    }

    /// Non-Windows stub: system UUID collection is unsupported.
    #[cfg(not(windows))]
    fn get_system_uuid() -> Result<String> {
        Err(AuthError::CmdFail("non-windows not supported".into()))
    }

    /// Queries the baseboard serial via WMI (`Win32_BaseBoard.SerialNumber`).
    ///
    /// Rejects empty strings and the common OEM placeholder value.
    #[cfg(windows)]
    fn get_board_sn() -> Result<String> {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct Win32BaseBoard { serial_number: Option<String> }

        let com = COMLibrary::new().map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let con = WMIConnection::new(com).map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let results: Vec<Win32BaseBoard> = con
            .raw_query("SELECT SerialNumber FROM Win32_BaseBoard")
            .map_err(|e| AuthError::CmdFail(e.to_string()))?;
        results.into_iter().next()
            .and_then(|r| r.serial_number)
            .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.")
            .ok_or(AuthError::RegexMatch)
    }

    /// Non-Windows stub: baseboard serial collection is unsupported.
    #[cfg(not(windows))]
    fn get_board_sn() -> Result<String> { Err(AuthError::CmdFail("non-windows not supported".into())) }

    /// Queries the BIOS serial via WMI (`Win32_BIOS.SerialNumber`).
    ///
    /// Rejects empty strings and the common OEM placeholder value.
    #[cfg(windows)]
    fn get_bios_sn() -> Result<String> {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase")]
        struct Win32Bios { serial_number: Option<String> }

        let com = COMLibrary::new().map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let con = WMIConnection::new(com).map_err(|e| AuthError::CmdFail(e.to_string()))?;
        let results: Vec<Win32Bios> = con
            .raw_query("SELECT SerialNumber FROM Win32_BIOS")
            .map_err(|e| AuthError::CmdFail(e.to_string()))?;
        results.into_iter().next()
            .and_then(|r| r.serial_number)
            .filter(|s| !s.is_empty() && s != "To be filled by O.E.M.")
            .ok_or(AuthError::RegexMatch)
    }

    /// Non-Windows stub: BIOS serial collection is unsupported.
    #[cfg(not(windows))]
    fn get_bios_sn() -> Result<String> { Err(AuthError::CmdFail("non-windows not supported".into())) }
}