# 8bitdo-gyro

Windows desktop utility for the 8BitDo Ultimate 3 Controller for Xbox. Vue 3 + TypeScript + Vite in a Tauri 2 shell; Bun manages frontend dependencies. The public repository must contain independently written source only: never commit secrets, controller identifiers, firmware images, vendor binaries, decompiled vendor source or local research dumps.

## Ownership

- `src/components/` owns controls and presentation; `src/App.vue` composes the screen.
- `src/useController.ts` owns drafts, selection and pending/error states. `src/device.ts` is the typed Tauri boundary.
- `src-tauri/src/settings.rs` owns motion layout and validation. Do not infer device state from defaults or the current USB PID.
- `src-tauri/src/protocol.rs` owns HID discovery, framing and response validation; `gip.rs` owns the Windows Xbox transport.
- `src-tauri/src/lib.rs` serializes operations and verifies writes. Write only the requested block, preserve other profiles/settings, and verify against a fresh read. Never repeat a write automatically after a timeout.

## Workflow

Inspect the actual implementation and device traces before changing a protocol contract. Keep one source of truth per contract, clear layer boundaries and no speculative fallback branches. Preserve unrelated dirty work. Research artifacts and temporary backups belong outside this repository.

Use `bun install --frozen-lockfile`, `bun run check`, `bun run build`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, and `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings`. `bun run desktop` runs the app; `bun run package` builds the Windows installer. Hardware checks use `cargo run --manifest-path src-tauri/Cargo.toml --example device -- read`; `verify-write` changes profile 1 compensation by one raw step and restores it.

Do not add unit tests. Use builds, type checks, browser/native UI checks and narrowly scoped hardware read/write verification. A rendered screen or an ACK alone does not prove settings persist or affect motion. Report those gaps explicitly.

## Interface

Keep a compact, non-resizable desktop window. All states, including calibration and errors, must fit without scrollbars. Use minimal English copy and intuitive controls. No marketing sections, eyebrows, decorative badges, nested cards, invented live telemetry or placeholder success. Preserve the approved lavender palette; use Onest, non-Material switches and Reka UI primitives for custom popups. No global focus outlines, text selection or text dragging in the app shell. Use scoped CSS with readable layout rules; avoid styled UI frameworks and Tailwind. Preserve drafts across profile changes. A save completes only after device readback succeeds.

## Durable rules

Если пользователь в ходе работы даёт новые устойчивые правила по стилю кода, структуре или процессу, то их надо кратко и по делу сразу добавлять в этот файл `AGENTS.md`, если это реально полезно будущим агентам.

Правила в `AGENTS.md` добавлять только если пользователь явно просит сохранить что-то универсальное и долговременное; не заносить туда ситуативные договорённости текущей задачи.

"Работает" недостаточно. После того как довел до рабочего состояния, убедись, что решение встроено в код красиво и без временных подпорок. Если по пути пришлось оставить костыль или фоллбэк, потом обязательно добейся его удаления, даже если для этого надо явно попросить пользователя сделать связанное изменение.

After each change, review the diff for dead code, misplaced responsibilities and traces of rejected hypotheses. After the working result, remove added abstractions, state and wrappers that do not earn their place, then repeat affected checks. Never print secret values in logs or review output.
