# Qubit Error

面向 Qubit Rust 项目的通用错误处理辅助库。

`qubit-error` 当前提供 boxed error 类型别名和显式转换工具，用于那些刻意擦除具体错误类型的场景。

## 功能

- `DynError`：统一的动态错误 trait object 约束。
- `BoxError`：拥有所有权的 boxed 动态错误。
- `BoxResult<T>`：使用 `BoxError` 的 `Result` 类型别名。
- `IntoBoxError`：适用于 `map_err` 和手工构造错误的显式转换工具。

## 安装

```toml
[dependencies]
qubit-error = "0.1.0"
```

## 快速开始

```rust
use qubit_error::{BoxResult, IntoBoxError};

fn parse_port(text: &str) -> BoxResult<u16> {
    text.parse::<u16>()
        .map_err(|error| error.into_box_error())
}
```

## 模块

- `error`：boxed error 类型别名和转换工具。

当前所有公开项都已在 crate 顶层重新导出。
