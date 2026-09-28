# focusdemo

**简体中文** | [English](#english)

BORUIX 上针对**终端焦点切换门禁**的**对抗性安全验收**程序。

这个程序的设计意图是**尝试突破权限**——它以普通用户身份运行，然后故意请求一个需要系统权限的
操作。**它期望被拒绝**。

---

## 背景：焦点切换为什么需要门禁

终端焦点决定了输入事件被送往哪个终端实例。如果任意进程都能改变焦点，它就可以把别的用户或
别的会话的输入劫持到自己的终端上——这是一条真实的攻击面。

因此焦点切换被限定为需要系统权限的操作。内核在执行前检查调用者身份。

## 对抗性验收的逻辑

普通的验收程序验证"正确的事情能成功"。这个程序验证的是**"错误的事情必须失败"**。

它以**默认用户身份**被启动（从系统初始化流程一路派生下来，不持有任何系统权限），然后请求切换
焦点：

| 结果 | 判定 | 含义 |
| --- | --- | --- |
| 被拒绝，错误码为"权限不足" | **PASS** | 门禁存在且真的在拦 |
| 调用**成功** | **FAIL** | 门禁被绕过 = 终端劫持面存在 |
| 被拒绝，但错误码不是"权限不足" | **FAIL** | 门禁没有先判身份就做别的检查 |

退出码：`0` 通过，`1` 门禁被绕过，`2` 错误码不对。

## 为什么"错误码不对"也算失败

第三种情况容易被忽略，但它是真实的信息泄露：如果内核**先解析参数、后检查权限**，那么一个无权
的调用者可以通过观察"是报权限错误还是报参数错误"来判断某个终端实例是否存在。

这是**实例存在性探测**。正确的顺序是**先判权限，再做其他检查**——这样无权限的调用者无论怎么
尝试都只能得到同一个回答。

## 输出

```
[focusdemo] adversarial FOCUS_SET(1) as unprivileged
[focusdemo] PASS: denied with EACCES (gate holds)
```

失败时会打印 FAIL 并说明是哪种失败形态。

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序后运行。

## 文件结构

```
focusdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 程序本体
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#focusdemo) | **English**

An **adversarial security acceptance** program for BORUIX's **terminal focus-switching gate**.

The program's design intent is to **attempt a privilege violation** — it runs as an ordinary user
and then deliberately requests an operation that requires system privilege. **It expects to be
denied.**

---

## Background: why focus switching needs a gate

Terminal focus determines which terminal instance input events are delivered to. If any process
could change focus at will, it could hijack another user's or another session's input into its own
terminal — a genuine attack surface.

Focus switching is therefore restricted to an operation requiring system privilege. The kernel checks
the caller's identity before performing it.

## The logic of adversarial acceptance

An ordinary acceptance program verifies that "the right things succeed". This one verifies that
**"the wrong things must fail"**.

It is started with **default user identity** (derived all the way down from the system init sequence,
holding no system privilege) and then requests a focus change:

| Outcome | Verdict | Meaning |
| --- | --- | --- |
| Denied with a "permission denied" code | **PASS** | the gate exists and really blocks |
| The call **succeeds** | **FAIL** | the gate was bypassed = a terminal-hijacking surface exists |
| Denied, but the code is not "permission denied" | **FAIL** | the gate did other checks before checking identity |

Exit codes: `0` pass, `1` gate bypassed, `2` wrong error code.

## Why "wrong error code" also counts as failure

The third case is easily overlooked but is a real information leak: if the kernel **parses arguments
before checking permission**, an unprivileged caller can tell whether a given terminal instance
exists by observing whether it gets a permission error or a parameter error.

That is **instance existence probing**. The correct order is to **check permission first, then do
anything else** — so that an unprivileged caller gets the same answer no matter how it probes.

## Output

```
[focusdemo] adversarial FOCUS_SET(1) as unprivileged
[focusdemo] PASS: denied with EACCES (gate holds)
```

On failure it prints FAIL and states which failure form occurred.

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system before running.

## Layout

```
focusdemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the program itself
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
