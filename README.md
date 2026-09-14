# 8bitdo-gyro

A small Windows app for the **8BitDo Ultimate 3 Controller for Xbox**. Built with Tauri and Vue.

- Three motion profiles, left/right stick or mouse output.
- Hold/toggle activation and button selection.
- Sensitivity, deadzone compensation and gyroscope calibration.
- Tournament mode switching and verified writes to the controller.

Connect using USB or the 2.4G receiver and close Ultimate Software X before opening the app. Xbox mode may require running as administrator. Select the corresponding hardware profile on the controller to use its settings.

Turning Tournament mode off may require restarting the controller and receiver. The app reports when a reconnect is needed; reading a saved mode does not prove the current USB mode has changed.

This is an independent community project, not affiliated with 8BitDo. Other Ultimate controller models are not supported.

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

Hardware diagnostics (the stock app must be closed):

```sh
cargo run --manifest-path src-tauri/Cargo.toml --example device -- read
```

`verify-write` temporarily changes profile 1 compensation by one raw step, checks the result, and restores it. `tournament on` and `tournament off` change the device mode.

## Device access

Configuration uses the controller's vendor HID interface in Tournament mode and Windows Xbox GIP in Xbox mode. The app writes only the requested motion block or Tournament byte, then reads back the complete configuration to detect unintended changes. It does not flash firmware or modify the official app.

Hardware-verified on Ultimate 3 over 2.4G in Tournament mode: profile read/write, calibration, and profile persistence after controller restart. The Rust Xbox GIP path and the final USB-mode transition after switching Tournament off have not yet been hardware-verified.

## License

MIT. The 8BitDo name identifies the supported hardware and is not a claim of endorsement.
