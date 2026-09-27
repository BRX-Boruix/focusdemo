//! BORUIX focusdemo：焦点切换门禁的**用户态对抗验收**程序（ADR-048 T3）。
//!
//! owner 裁决 α：焦点切换 = `SYS_STREAM_FOCUS_SET`，内核 `current_has_cap`
//! 门禁（audio attach 同款）。本程序以**默认用户身份**（init→shell→exec，
//! 无 CAP_SYSTEM）被拉起，必须被内核 EACCES 拒绝——这是 α 门禁「存在且
//! 真的拦」的真机证据；反之若调用意外成功，即终端劫持面存在（S17 红灯）。
#![no_std]
#![no_main]

use libsys::{event::focus_set, write, STDOUT};

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    let _ = write(STDOUT, b"[focusdemo] adversarial FOCUS_SET(1) as unprivileged (ADR-048 T3)\n");
    match focus_set(1) {
        Ok(()) => {
            // 门禁被绕过 = 终端劫持面 = 最严重形态的失败（S20 红灯）。
            let _ = write(STDOUT, b"[focusdemo] FAIL: FOCUS_SET succeeded without CAP_SYSTEM (gate bypassed!)\n");
            1
        }
        Err(e) => {
            // 期望 EACCES。其他错误（EINVAL 等）说明门禁没先判身份——
            // audio attach 纪律：先权限后解析，防实例存在性探测。如实报告。
            if e == libsys::Error::PermissionDenied {
                let _ = write(STDOUT, b"[focusdemo] PASS: denied with EACCES (gate holds)\n");
                0
            } else {
                let _ = write(STDOUT, b"[focusdemo] FAIL: denied but with wrong errno (expected EACCES)\n");
                2
            }
        }
    }
}
