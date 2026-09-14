use eightbitdo_gyro::{change_tournament, read_controller, write_motion};

fn main() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let settings = match args.first().map(String::as_str) {
        None | Some("read") => read_controller()?,
        Some("tournament") => match args.get(1).map(String::as_str) {
            Some("on") => change_tournament(true)?,
            Some("off") => change_tournament(false)?,
            _ => return Err("Usage: device tournament on|off".into()),
        },
        Some("verify-write") => {
            let original = read_controller()?;
            let mut changed = original.profiles[0].clone();
            changed.deadzone = if changed.deadzone == 255 {
                254
            } else {
                changed.deadzone + 1
            };
            // Always attempt restoration, including when write verification fails.
            let result = write_motion(0, changed);
            let restored = write_motion(0, original.profiles[0].clone())?;
            result?;
            if serde_json::to_value(&original).unwrap() != serde_json::to_value(&restored).unwrap()
            {
                return Err("Restored settings differ from the original.".into());
            }
            restored
        }
        _ => return Err("Usage: device [read|verify-write|tournament on|off]".into()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&settings).map_err(|error| error.to_string())?
    );
    Ok(())
}
