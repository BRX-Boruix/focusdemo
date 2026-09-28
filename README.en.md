# focusdemo

An **adversarial acceptance program** for BORUIX: it verifies that the focus-switching operation is stopped by the privilege gate.

[简体中文](README.md)

## What it tests

Switching terminal focus is a privilege-controlled operation. This program is started under an **ordinary user identity** (not a system identity), then **deliberately attempts** the operation and **expects the kernel to refuse it**.

| Outcome observed | Verdict |
| --- | --- |
| Refused, with "permission denied" | **PASS** — the gate exists and genuinely stops it |
| Refused, but with some other error (e.g. "invalid argument") | **FAIL** — the gate did not check identity first, it parsed arguments first |
| **It succeeded** | **FAIL (the most severe)** — the gate was bypassed and any program could hijack the terminal |

## Note: this program expects to fail

**Its "success" is being refused.** Seeing it return `0` and print `PASS: denied with EACCES` is exactly the expected outcome.

Conversely, if it prints `FAIL: FOCUS_SET succeeded`, that is the genuinely serious problem.

## Why the error code is checked too

Judging "it was refused" is not enough. **Checking privilege before parsing arguments** is a deliberate order: if an implementation parsed arguments first instead, an unprivileged caller could **probe whether a particular object exists** through the differing error codes. So this program requires "permission denied" and counts any other error as a failure.

## Exit codes

| Exit code | Meaning |
| --- | --- |
| `0` | Refused with "permission denied"; the gate holds |
| `1` | **The operation unexpectedly succeeded**; the gate was bypassed |
| `2` | Refused, but with the wrong error code |

## Building

```bash
cargo build --release
```

## Layout

```
focusdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the adversarial call and verdict
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — provides the focus switching interface
- [`consoled`](https://github.com/BRX-Boruix/consoled) — the console daemon

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
