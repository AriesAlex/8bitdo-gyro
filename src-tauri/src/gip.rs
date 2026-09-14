use std::{mem::size_of, ptr};
use windows_sys::Win32::{
    Foundation::{
        CloseHandle, GetLastError, ERROR_IO_PENDING, HANDLE, INVALID_HANDLE_VALUE, WAIT_OBJECT_0,
    },
    Storage::FileSystem::{
        CreateFileW, ReadFile, WriteFile, FILE_FLAG_OVERLAPPED, FILE_SHARE_READ, FILE_SHARE_WRITE,
        OPEN_EXISTING,
    },
    System::{
        Threading::{CreateEventW, WaitForSingleObject},
        IO::{CancelIoEx, DeviceIoControl, GetOverlappedResult, OVERLAPPED},
    },
};

struct Handle(HANDLE);

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

pub struct Gip {
    handle: Handle,
    controller_id: u64,
    configuration_mode: bool,
}

impl Gip {
    pub fn open() -> Result<Self, String> {
        let path: Vec<u16> = "\\\\.\\XboxGIP_Admin\0".encode_utf16().collect();
        let raw = unsafe {
            CreateFileW(
                path.as_ptr(),
                0xC0000000,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_OVERLAPPED,
                ptr::null_mut(),
            )
        };
        if raw == INVALID_HANDLE_VALUE {
            return Err(match unsafe { GetLastError() } {
                5 => "Xbox mode needs administrator access. Run the app as administrator.".into(),
                2 => "Connect your Ultimate 3 for Xbox using USB or 2.4G.".into(),
                code => format!("Cannot open Xbox transport (Windows {code})."),
            });
        }
        let mut gip = Self {
            handle: Handle(raw),
            controller_id: 0,
            configuration_mode: false,
        };
        gip.register(0x40001C10, Some(0x0F38))?;
        gip.register(0x40001CD0, None)?;
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while std::time::Instant::now() < deadline {
            let event = gip.read()?;
            if event.len() >= 24
                && event[8..12] == 0x2004u32.to_le_bytes()
                && event[20..22] == 0x2DC8u16.to_le_bytes()
                && event[22..24] == 0x2072u16.to_le_bytes()
            {
                gip.controller_id = u64::from_le_bytes(event[..8].try_into().unwrap());
                break;
            }
        }
        if gip.controller_id == 0 {
            return Err("Ultimate 3 for Xbox was not found. Reconnect the controller.".into());
        }
        let mut control = gip.outer_header(9, 9);
        control.extend_from_slice(&[0, 15, 0, 0, 0, 0, 255, 0, 235]);
        gip.write(&control)?;
        let mut start = [0; 64];
        start[..4].copy_from_slice(&[0x81, 4, 7, 0]);
        gip.send(&start)?;
        gip.configuration_mode = true;
        Ok(gip)
    }

    fn register(&self, code: u32, input: Option<u32>) -> Result<(), String> {
        self.transfer(|overlapped| unsafe {
            DeviceIoControl(
                self.handle.0,
                code,
                input
                    .as_ref()
                    .map_or(ptr::null(), |value| (value as *const u32).cast()),
                if input.is_some() {
                    size_of::<u32>() as u32
                } else {
                    0
                },
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                overlapped,
            )
        })
        .map(|_| ())
    }

    fn outer_header(&self, message: u32, length: u32) -> Vec<u8> {
        let mut bytes = vec![0; 20];
        bytes[..8].copy_from_slice(&self.controller_id.to_le_bytes());
        bytes[8..12].copy_from_slice(&message.to_le_bytes());
        bytes[12..16].copy_from_slice(&length.to_le_bytes());
        bytes
    }

    pub fn send(&self, report: &[u8; 64]) -> Result<(), String> {
        let mut packet = self.outer_header(0x13, 60);
        packet.extend_from_slice(&report[1..61]);
        self.write(&packet)
    }

    pub fn receive(&self) -> Result<Vec<u8>, String> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while std::time::Instant::now() < deadline {
            let packet = self.read()?;
            if packet.len() < 36
                || packet[..8] != self.controller_id.to_le_bytes()
                || packet[8..12] != 0x12u32.to_le_bytes()
                || packet[20] != 4
            {
                continue;
            }
            if packet[21] != 0 {
                return Err(format!(
                    "Controller rejected the request (0x{:02X}).",
                    packet[21]
                ));
            }
            let mut report = vec![0; 64];
            report[..4].copy_from_slice(&[2, 4, 4, 0]);
            let length = (packet.len() - 22).min(60);
            report[4..4 + length].copy_from_slice(&packet[22..22 + length]);
            return Ok(report);
        }
        Err("The controller did not answer.".into())
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let mut bytes = vec![0; 2048];
        let length = self.transfer(|overlapped| unsafe {
            ReadFile(
                self.handle.0,
                bytes.as_mut_ptr(),
                bytes.len() as u32,
                ptr::null_mut(),
                overlapped,
            )
        })?;
        bytes.truncate(length);
        Ok(bytes)
    }

    fn write(&self, bytes: &[u8]) -> Result<(), String> {
        let written = self.transfer(|overlapped| unsafe {
            WriteFile(
                self.handle.0,
                bytes.as_ptr(),
                bytes.len() as u32,
                ptr::null_mut(),
                overlapped,
            )
        })?;
        if written != bytes.len() {
            return Err("Incomplete Xbox request.".into());
        }
        Ok(())
    }

    fn transfer(&self, start: impl FnOnce(*mut OVERLAPPED) -> i32) -> Result<usize, String> {
        let event = Handle(unsafe { CreateEventW(ptr::null(), 1, 0, ptr::null()) });
        if event.0.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let mut overlapped: OVERLAPPED = unsafe { std::mem::zeroed() };
        overlapped.hEvent = event.0;
        let ok = start(&mut overlapped);
        if ok == 0 && unsafe { GetLastError() } != ERROR_IO_PENDING {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let mut transferred = 0;
        if ok == 0 && unsafe { WaitForSingleObject(event.0, 2500) } != WAIT_OBJECT_0 {
            // Drain cancellation before the stack-owned OVERLAPPED/buffer can be freed.
            unsafe {
                CancelIoEx(self.handle.0, &overlapped);
                GetOverlappedResult(self.handle.0, &overlapped, &mut transferred, 1);
            }
            return Err("The controller did not answer. Reconnect it and try again.".into());
        }
        if unsafe { GetOverlappedResult(self.handle.0, &overlapped, &mut transferred, 0) } == 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }
        Ok(transferred as usize)
    }
}

impl Drop for Gip {
    fn drop(&mut self) {
        if self.configuration_mode {
            let mut report = [0; 64];
            report[..6].copy_from_slice(&[0x81, 4, 7, 0, 1, 0]);
            if let Err(error) = self.send(&report) {
                eprintln!("Xbox configuration session could not close: {error}");
            }
        }
    }
}
