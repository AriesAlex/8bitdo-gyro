use crate::{gip::Gip, settings::CONFIG_SIZE};
use hidapi::{HidApi, HidDevice};
use std::time::{Duration, Instant};

enum Transport {
    Hid(HidDevice),
    Gip(Gip),
}

pub struct Controller {
    transport: Transport,
}

impl Controller {
    pub fn uses_hid(&self) -> bool {
        matches!(self.transport, Transport::Hid(_))
    }

    pub fn open() -> Result<Self, String> {
        let api = HidApi::new().map_err(|error| error.to_string())?;
        let devices: Vec<_> = api
            .device_list()
            .filter(|device| {
                device.vendor_id() == 0x2DC8
                    && matches!(device.product_id(), 0x2051..=0x2053)
                    && device.usage_page() == 0x8C
                    && device.usage() == 1
            })
            .collect();
        if devices.len() > 1 {
            return Err("Leave one Ultimate 3 controller connected.".into());
        }
        let transport = if let Some(device) = devices.first() {
            Transport::Hid(
                device
                    .open_device(&api)
                    .map_err(|error| error.to_string())?,
            )
        } else {
            Transport::Gip(Gip::open()?)
        };
        Ok(Self { transport })
    }

    pub fn read_config(&self) -> Result<Vec<u8>, String> {
        let mut config = Vec::with_capacity(CONFIG_SIZE);
        while config.len() < CONFIG_SIZE {
            let length = (CONFIG_SIZE - config.len()).min(32);
            config.extend_from_slice(&self.read_block(config.len(), length)?);
        }
        Ok(config)
    }

    pub fn read_block(&self, offset: usize, length: usize) -> Result<Vec<u8>, String> {
        let response = self.request(2, 0, offset, length, &[])?;
        Ok(response[18..18 + length].to_vec())
    }

    pub fn write_block(&self, offset: usize, data: &[u8]) -> Result<(), String> {
        self.request(1, 0, offset, data.len(), data)?;
        // Command 1 stages bytes; command 6 applies them to the input processor.
        self.request(6, 0, 0, 0, &[])?;
        Ok(())
    }

    pub fn calibrate(&self) -> Result<(), String> {
        self.request(0x60, 3, 0, 0, &[])?;
        std::thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    fn request(
        &self,
        command: u16,
        sub: u16,
        offset: usize,
        length: usize,
        data: &[u8],
    ) -> Result<Vec<u8>, String> {
        if length > 32 || data.len() > 32 || offset + length > CONFIG_SIZE {
            return Err("Invalid controller request.".into());
        }
        // HID describes the partial block; Xbox GIP retains the full image size.
        let total = if matches!(command, 1 | 2) {
            (if self.uses_hid() { length } else { CONFIG_SIZE }) as u32
        } else {
            0
        };
        let mut report = [0; 64];
        report[..2].copy_from_slice(&[0x81, 4]);
        report[2..4].copy_from_slice(&command.to_le_bytes());
        report[4..6].copy_from_slice(&sub.to_le_bytes());
        report[6..8].copy_from_slice(&(length as u16).to_le_bytes());
        report[10..14].copy_from_slice(&total.to_le_bytes());
        report[14..18].copy_from_slice(&(offset as u32).to_le_bytes());
        report[18..18 + data.len()].copy_from_slice(data);
        match &self.transport {
            Transport::Hid(device) => {
                if device.write(&report).map_err(|error| error.to_string())? != report.len() {
                    return Err("Incomplete HID request.".into());
                }
            }
            Transport::Gip(device) => device.send(&report)?,
        }
        let deadline = Instant::now() + Duration::from_secs(3);
        while Instant::now() < deadline {
            let reply = match &self.transport {
                Transport::Hid(device) => {
                    let mut reply = vec![0; 64];
                    let count = device
                        .read_timeout(&mut reply, 500)
                        .map_err(|error| error.to_string())?;
                    reply.truncate(count);
                    reply
                }
                Transport::Gip(device) => device.receive()?,
            };
            if reply.len() != 64
                || reply[..4] != [2, 4, 4, 0]
                || reply[4..6] != command.to_le_bytes()
            {
                continue;
            }
            if matches!(command, 1 | 2)
                && (reply[6..8] != (length as u16).to_le_bytes()
                    || reply[10..14] != total.to_le_bytes()
                    || reply[14..18] != (offset as u32).to_le_bytes())
            {
                continue;
            }
            return Ok(reply);
        }
        Err(format!(
            "The controller did not acknowledge command 0x{command:02X}. Refresh and try again."
        ))
    }
}
