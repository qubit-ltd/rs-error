# Qubit Error

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-error.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-error)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-error/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-error?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-error.svg?color=blue)](https://crates.io/crates/qubit-error)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的通用错误处理工具库。

## 概述

Qubit Error 提供小型、可复用的错误处理基础工具，适用于有意擦除具体错误类型的场景。当前重点是 boxed error 类型别名和显式转换 trait，未来可以继续扩展其他通用错误处理工具。

## 设计目标

- **极小依赖面**：只提供基于标准库的工具。
- **显式类型擦除**：让 boxed error 转换在调用点清晰可见。
- **便于集成**：支持回调、可执行入口和保存 source error 的场景。
- **具体错误优先**：鼓励领域 crate 保持结构化公开错误类型。
- **面向未来扩展**：可以增加更多通用错误工具，而不耦合到领域 crate。

## 特性

### Boxed Error 类型别名

- **`DynError`**：统一的 `dyn Error + Send + Sync + 'static` 对象约束。
- **`BoxError`**：拥有所有权的 boxed 动态错误。
- **`BoxResult<T>`**：使用 `BoxError` 的结果别名。

### 显式转换

- **`IntoBoxError`**：将具体标准错误转换为 `BoxError`。
- 适用于 `map_err`、泛型回调边界和手工构造 `Err`。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-error = "0.2.0"
```

## 快速开始

```rust
use qubit_error::{
    BoxResult,
    IntoBoxError,
};

fn parse_port(text: &str) -> BoxResult<u16> {
    text.parse::<u16>()
        .map_err(|error| error.into_box_error())
}

let port = parse_port("8080").expect("valid port should parse");
assert_eq!(port, 8080);
assert!(parse_port("not-a-port").is_err());
```

## 何时使用 Boxed Error

当调用方只需要传播、记录或保存原始 source error 时，适合使用 boxed error 工具：

- 小型二进制入口或示例。
- 泛型回调返回类型。
- 不相关错误类型之间的集成胶水代码。
- 在上层具体领域错误中保存 source error。

如果公共 API 的调用方需要按错误类型或变体进行恢复，应优先返回具体错误 enum 或 struct。

## API 参考

- [`DynError`](https://docs.rs/qubit-error/latest/qubit_error/type.DynError.html) - 统一的动态错误对象约束。
- [`BoxError`](https://docs.rs/qubit-error/latest/qubit_error/type.BoxError.html) - 拥有所有权的 boxed 动态错误。
- [`BoxResult`](https://docs.rs/qubit-error/latest/qubit_error/type.BoxResult.html) - 使用 `BoxError` 的结果别名。
- [`IntoBoxError`](https://docs.rs/qubit-error/latest/qubit_error/trait.IntoBoxError.html) - 显式转换 trait。

## 测试与代码覆盖率

本项目的测试聚焦于类型约束、转换行为和 doctest 示例。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率、审计）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

本 crate 除 Rust 标准库外没有运行时依赖。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 保持工具的通用性，避免依赖领域 crate。
- 在文档能帮助理解时，为公共 API 提供示例。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-error](https://github.com/qubit-ltd/rs-error)
