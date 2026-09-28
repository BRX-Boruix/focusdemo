# focusdemo

A BORUIX adversarial acceptance test: an unprivileged process must not be able to switch the terminal focus.

[简体中文](README.md)

## What it tests

Running as an ordinary user, it attempts to switch the audio stream focus. The kernel gates this
interface behind a privilege check, so this call **must be denied**:

- Denied with a permission error — pass
- The call succeeds — fail: the gate was bypassed and a terminal-hijack surface exists
- Denied with a different error — fail: the gate did not verify identity first

Output:

```
[focusdemo] PASS: denied with EACCES (gate holds)
```

## Exit codes

- `0` — the call was correctly denied
- `1` — the call unexpectedly succeeded; the gate is broken
- `2` — denied, but with the wrong error

## Building

```bash
cargo build --release
```

## Repository layout

```
focusdemo/
├── Cargo.toml    # package manifest
├── build.rs      # injects the linker script
├── linker.ld     # user-space segment layout
└── src/
    └── main.rs   # attempts the focus switch and judges the denial
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the focus switch interface
- [`consoled`](https://github.com/BRX-Boruix/consoled) — console daemon

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
