# 代码覆盖率指南

本仓库使用共享的 `rs-infra-coverage` 工具和 `cargo-llvm-cov` 收集并检查代码覆盖率。

## 收集覆盖率

在仓库根目录运行：

```bash
./.infra/bin/coverage.sh
```

该脚本会按配置收集覆盖率，随后生成报告。它使用 `.infra/bootstrap.sh` 解析到的最新 `rs-infra-coverage` 工具；不接受 `html`、`text`、`lcov`、`json` 或 `cobertura` 这类报告格式名称作为位置参数。工具的 `collect` 命令接受 `--output <PATH>` 和 `--ignore-thresholds`；项目包装脚本还会运行共享报告脚本。

需要单独生成报告时，可在仓库根目录直接运行 `cargo llvm-cov`：

```bash
cargo llvm-cov clean
cargo llvm-cov --html                 # HTML：target/llvm-cov/html/index.html
cargo llvm-cov --html --open          # 生成 HTML 并在浏览器中打开
cargo llvm-cov                        # 在终端显示文本摘要
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --json --output-path coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

共享报告脚本会在仓库根目录生成 `coverage.json`、`lcov.info`、`coverage.txt`、`target/llvm-cov/cobertura.xml` 和 `target/llvm-cov/html/index.html`。CI 工作流会将它们发布到 `coverage-reports` artifact。上面的直接 `cargo llvm-cov` 命令使用各命令指定的路径；HTML 默认写入 `target/llvm-cov/html`。

## 范围与配置

覆盖率工具会读取 `.infra/coverage/coverage.json`（如果存在）；否则使用工具默认值。本仓库目前没有该项目配置文件。已提交的 `.llvm-cov.toml` 不会被当前覆盖率工具或 `cargo-llvm-cov 0.9.1` 读取，因此已删除。`cargo-llvm-cov` 默认排除常见测试代码位置；当前行为请查看其自身文档。

CI 覆盖范围、排除项和阈值由共享覆盖率配置及工作流控制。本仓库的本地入口和工作流分别是 [`.infra/bin/coverage.sh`](.infra/bin/coverage.sh) 与 [`.github/workflows/ci.yml`](.github/workflows/ci.yml)；本仓库将 CI 委托给 `qubit-ltd/rs-infra-ci/.github/workflows/github-ci.yml@main`。

## 常见问题

如果找不到 `cargo llvm-cov` 命令，请运行 `cargo install cargo-llvm-cov` 安装。若旧覆盖率数据影响本地运行，可先执行 `cargo llvm-cov clean` 清理，再重新收集。

## 参考资料

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
