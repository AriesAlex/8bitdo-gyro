use serde::{Deserialize, Serialize};

pub const CONFIG_SIZE: usize = 0x340;
pub const PROFILE_COUNT: usize = 3;
pub const MOTION_SIZE: usize = 12;
const MOTION_FLAG: u32 = 0x20200911;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MotionSettings {
    pub enabled: bool,
    pub activation_key: u32,
    pub activation_mode: u8,
    pub sensitivity: u8,
    pub deadzone: u8,
    pub mapping: u8,
}

impl MotionSettings {
    pub fn decode(data: &[u8]) -> Result<Self, String> {
        if data.len() != MOTION_SIZE {
            return Err("Invalid motion block length.".into());
        }
        let flag = u32::from_le_bytes(data[0..4].try_into().unwrap());
        // The stock app uses these defaults for an uninitialized profile.
        if data.iter().all(|byte| *byte == 0) {
            return Ok(Self {
                enabled: false,
                activation_key: 0,
                activation_mode: 1,
                sensitivity: 5,
                deadzone: 30,
                mapping: 1,
            });
        }
        if flag != 0 && flag != MOTION_FLAG {
            return Err(format!("Unknown motion format: 0x{flag:08X}."));
        }
        let settings = Self {
            enabled: flag == MOTION_FLAG,
            activation_key: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            activation_mode: data[8],
            sensitivity: data[9],
            deadzone: data[10],
            mapping: data[11],
        };
        settings.validate()?;
        Ok(settings)
    }

    pub fn encode(&self) -> Result<[u8; MOTION_SIZE], String> {
        self.validate()?;
        if self.enabled && self.activation_key == 0 {
            return Err("Choose an activation button.".into());
        }
        let mut data = [0; MOTION_SIZE];
        let flag = if self.enabled { MOTION_FLAG } else { 0 };
        data[0..4].copy_from_slice(&flag.to_le_bytes());
        data[4..8].copy_from_slice(&self.activation_key.to_le_bytes());
        data[8] = self.activation_mode;
        data[9] = self.sensitivity;
        data[10] = self.deadzone;
        data[11] = self.mapping;
        Ok(data)
    }

    fn validate(&self) -> Result<(), String> {
        if !matches!(self.activation_mode, 1 | 2)
            || !(1..=6).contains(&self.sensitivity)
            || self.mapping > 2
        {
            return Err("Unsupported motion settings. Refresh the controller.".into());
        }
        Ok(())
    }
}

pub fn motion_offset(profile: usize) -> Result<usize, String> {
    if profile >= PROFILE_COUNT {
        return Err("Choose profile 1, 2 or 3.".into());
    }
    Ok(0x10C + profile * 0x114)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerSettings {
    pub tournament: bool,
    pub reconnect_required: bool,
    pub profiles: Vec<MotionSettings>,
}

impl ControllerSettings {
    pub fn decode(data: &[u8], hid_transport: bool) -> Result<Self, String> {
        if data.len() != CONFIG_SIZE || data[1] > 1 {
            return Err("Unsupported controller configuration.".into());
        }
        let profiles = (0..PROFILE_COUNT)
            .map(|profile| {
                let start = motion_offset(profile)?;
                MotionSettings::decode(&data[start..start + MOTION_SIZE])
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            tournament: data[1] == 1,
            reconnect_required: data[1] == 0 && hid_transport,
            profiles,
        })
    }
}
