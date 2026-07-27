use anyhow::{bail, ensure, Result};
use std::path::Path;

#[cfg(windows)]
pub fn verify_msix_signature(path: &Path) -> Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Security::WinTrust::{
        WinVerifyTrust, WINTRUST_ACTION_GENERIC_VERIFY_V2, WINTRUST_DATA, WINTRUST_DATA_0,
        WINTRUST_FILE_INFO, WTD_CHOICE_FILE, WTD_REVOKE_NONE, WTD_STATEACTION_CLOSE,
        WTD_STATEACTION_VERIFY, WTD_UICONTEXT_INSTALL, WTD_UI_NONE,
    };

    ensure!(path.is_file(), "package does not exist: {}", path.display());
    let wide_path: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut file_info = WINTRUST_FILE_INFO {
        cbStruct: std::mem::size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: PCWSTR(wide_path.as_ptr()),
        ..Default::default()
    };
    let mut trust_data = WINTRUST_DATA {
        cbStruct: std::mem::size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,
        fdwRevocationChecks: WTD_REVOKE_NONE,
        dwUnionChoice: WTD_CHOICE_FILE,
        Anonymous: WINTRUST_DATA_0 {
            pFile: &mut file_info,
        },
        dwStateAction: WTD_STATEACTION_VERIFY,
        dwUIContext: WTD_UICONTEXT_INSTALL,
        ..Default::default()
    };
    let mut action = WINTRUST_ACTION_GENERIC_VERIFY_V2;

    let status = unsafe {
        WinVerifyTrust(
            HWND::default(),
            &mut action,
            &mut trust_data as *mut WINTRUST_DATA as *mut core::ffi::c_void,
        )
    };
    trust_data.dwStateAction = WTD_STATEACTION_CLOSE;
    unsafe {
        let _ = WinVerifyTrust(
            HWND::default(),
            &mut action,
            &mut trust_data as *mut WINTRUST_DATA as *mut core::ffi::c_void,
        );
    }

    if status != 0 {
        bail!(
            "package signature verification failed for {} (WinVerifyTrust 0x{:08X})",
            path.display(),
            status as u32
        );
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn verify_msix_signature(_path: &Path) -> Result<()> {
    bail!("package signature verification is supported only on Windows")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(windows)]
    #[test]
    fn unsigned_file_is_rejected() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "storeless-updater-unsigned-{}-{unique}.msix",
            std::process::id()
        ));
        fs::write(&path, b"not a signed package").unwrap();

        let error = verify_msix_signature(&path).unwrap_err();

        let _ = fs::remove_file(path);
        assert!(error.to_string().contains("signature verification failed"));
    }
}
