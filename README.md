# focusdemo

BORUIX 的对抗验收程序：验证无特权的进程不能切换终端焦点。

[English](README.en.md)

## 测什么

以普通用户身份运行，尝试切换音频流焦点。内核对此接口设有权限门禁，本程序**必须被拒绝**：

- 调用被拒绝且错误码是权限不足——通过
- 调用成功——失败，说明门禁被绕过，存在终端劫持面
- 被拒绝但错误码不对——失败，说明门禁没有先验证身份

输出：

```
[focusdemo] PASS: denied with EACCES (gate holds)
```

## 退出码

- `0`——调用被正确拒绝
- `1`——调用意外成功，门禁失效
- `2`——被拒绝但错误码不是权限不足

## 构建

```bash
cargo build --release
```

## 文件结构

```
focusdemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 尝试切换焦点并判定拒绝形态
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 焦点切换接口
- [`consoled`](https://github.com/BRX-Boruix/consoled) —— 控制台守护进程

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
