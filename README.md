# 8bitdo-gyro

A small Windows app for the **8BitDo Ultimate 3 Controller for Xbox**. Built with Tauri and Vue.

- Three motion profiles, left/right stick or mouse output.
- Hold/toggle activation and button selection.
- Sensitivity, deadzone compensation and gyroscope calibration.
- Tournament mode switching and verified writes to the controller.

Connect using USB or the 2.4G receiver and close Ultimate Software X before opening the app. Xbox mode may require running as administrator. Select the corresponding hardware profile on the controller to use its settings.

Turning Tournament mode off may require restarting the controller and receiver. The app reports when a reconnect is needed; reading a saved mode does not prove the current USB mode has changed.

This is an independent community project, not affiliated with 8BitDo. Other Ultimate controller models are not supported.

## Download

Get **8bitdo-gyro.exe** from [GitHub Releases](https://github.com/AriesAlex/8bitdo-gyro/releases). One portable Windows x64 executable, no installer or companion files. It uses the Microsoft WebView2 Runtime installed on the system.

Releases are built locally and uploaded manually. There are no GitHub Actions or CI/CD builds.

## Development

Requires Windows, Bun, Rust's MSVC toolchain, Visual Studio C++ Build Tools and WebView2.

```sh
bun install --frozen-lockfile
bun run desktop
```

```sh
bun run check
bun run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
bun run package
```

The portable executable is written to `src-tauri/target/release/eightbitdo-gyro.exe`.

Hardware diagnostics (the stock app must be closed):

```sh
cargo run --manifest-path src-tauri/Cargo.toml --example device -- read
```

`verify-write` temporarily changes profile 1 compensation by one raw step, checks the result, and restores it. `tournament on` and `tournament off` change the device mode.

## Device access

Configuration uses the controller's vendor HID interface in Tournament mode and Windows Xbox GIP in Xbox mode. The app writes only the requested motion block or Tournament byte, sends the apply command to the input processor, then reads back the complete configuration to detect unintended changes. It does not flash firmware or modify the official app.

After upgrading from v0.1.0, select each configured profile and press Save once. That release staged HID settings without applying every motion parameter. Save can also reapply unchanged settings.

Hardware checks on Ultimate 3 over 2.4G in Tournament mode cover configuration readback, acknowledged writes and apply commands, and calibration. They do not validate every output mode, motion response or persistence across a receiver power cycle. The Rust Xbox GIP path and the final USB-mode transition after switching Tournament off have not yet been hardware-verified.

Known limitations in the tested setup: Mouse output produces no cursor movement, and horizontal gyro response still needs investigation. The HID apply fix improves right-stick motion but is not a complete motion-quality fix.

## License

MIT. The 8BitDo name identifies the supported hardware and is not a claim of endorsement.
