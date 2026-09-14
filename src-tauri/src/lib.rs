mod gip;
mod protocol;
pub mod settings;

use protocol::Controller;
use settings::{motion_offset, ControllerSettings, MotionSettings, MOTION_SIZE};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

static DEVICE_ACCESS: Mutex<()> = Mutex::new(());

fn ensure_stock_app_closed() -> Result<(), String> {
    use windows_sys::Win32::{
        Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
        System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
            TH32CS_SNAPPROCESS,
        },
    };
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        let mut found = Process32FirstW(snapshot, &mut entry);
        let mut stock_open = false;
        while found != 0 {
            let length = entry
                .szExeFile
                .iter()
                .position(|character| *character == 0)
                .unwrap_or(entry.szExeFile.len());
            if String::from_utf16_lossy(&entry.szExeFile[..length])
                .eq_ignore_ascii_case("UWPAdvance.exe")
            {
                stock_open = true;
                break;
            }
            found = Process32NextW(snapshot, &mut entry);
        }
        CloseHandle(snapshot);
        if stock_open {
            return Err("Close 8BitDo Ultimate Software X, then refresh.".into());
        }
    }
    Ok(())
}

pub fn read_controller() -> Result<ControllerSettings, String> {
    let _access = DEVICE_ACCESS
        .try_lock()
        .map_err(|_| "Another controller operation is running.")?;
    ensure_stock_app_closed()?;
    let controller = Controller::open()?;
    ControllerSettings::decode(&controller.read_config()?, controller.uses_hid())
}

pub fn write_motion(
    profile: usize,
    settings: MotionSettings,
) -> Result<ControllerSettings, String> {
    let _access = DEVICE_ACCESS
        .try_lock()
        .map_err(|_| "Another controller operation is running.")?;
    ensure_stock_app_closed()?;
    let offset = motion_offset(profile)?;
    let bytes = settings.encode()?;
    let controller = Controller::open()?;
    let mut expected = controller.read_config()?;
    if expected[offset..offset + MOTION_SIZE] != bytes {
        controller.write_block(offset, &bytes)?;
        expected[offset..offset + MOTION_SIZE].copy_from_slice(&bytes);
    }
    let actual = controller.read_config()?;
    if actual != expected {
        return Err("Save could not be verified. Refresh before making more changes.".into());
    }
    ControllerSettings::decode(&actual, controller.uses_hid())
}

pub fn change_tournament(enabled: bool) -> Result<ControllerSettings, String> {
    let _access = DEVICE_ACCESS
        .try_lock()
        .map_err(|_| "Another controller operation is running.")?;
    ensure_stock_app_closed()?;
    let controller = Controller::open()?;
    let mut expected = controller.read_config()?;
    if expected[1] == u8::from(enabled) {
        return ControllerSettings::decode(&expected, controller.uses_hid());
    }
    expected[1] = u8::from(enabled);
    controller.write_block(1, &[expected[1]])?;
    drop(controller);

    // A mode change can replace the USB device. Reopen for readback, never resend a write.
    let deadline = Instant::now() + Duration::from_secs(12);
    let mut last_error = String::new();
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(400));
        match Controller::open().and_then(|device| Ok((device.read_config()?, device.uses_hid()))) {
            Ok((actual, hid)) if actual == expected => {
                return ControllerSettings::decode(&actual, hid)
            }
            Ok(_) => last_error = "The controller has not confirmed the new mode.".into(),
            Err(error) => last_error = error,
        }
    }
    Err(format!(
        "Mode change could not be verified. Refresh the controller. {last_error}"
    ))
}

pub fn calibrate_controller() -> Result<(), String> {
    let _access = DEVICE_ACCESS
        .try_lock()
        .map_err(|_| "Another controller operation is running.")?;
    ensure_stock_app_closed()?;
    Controller::open()?.calibrate()
}

#[tauri::command]
async fn read_settings() -> Result<ControllerSettings, String> {
    tauri::async_runtime::spawn_blocking(read_controller)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn save_motion(
    profile: usize,
    settings: MotionSettings,
) -> Result<ControllerSettings, String> {
    tauri::async_runtime::spawn_blocking(move || write_motion(profile, settings))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn set_tournament(enabled: bool) -> Result<ControllerSettings, String> {
    tauri::async_runtime::spawn_blocking(move || change_tournament(enabled))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
async fn calibrate() -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(calibrate_controller)
        .await
        .map_err(|error| error.to_string())?
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_settings,
            save_motion,
            set_tournament,
            calibrate
        ])
        .run(tauri::generate_context!())
        .expect("Could not start 8bitdo gyro");
}
