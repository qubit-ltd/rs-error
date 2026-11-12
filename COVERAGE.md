# Code Coverage Guide

This repository uses the shared `rs-infra-coverage` tool and `cargo-llvm-cov` to collect and check code coverage.

## Collect coverage

From the repository root, run:

```bash
./.infra/bin/coverage.sh
```

The script runs the configured coverage collection and then creates reports. It uses the latest `rs-infra-coverage` tool selected by `.infra/bootstrap.sh`; it does not accept report-format names such as `html`, `text`, `lcov`, `json`, or `cobertura` as positional arguments. The tool's `collect` command accepts `--output <PATH>` and `--ignore-thresholds`; the project wrapper also runs the shared report script.

For one-off reports, run `cargo llvm-cov` directly from the repository root:

```bash
cargo llvm-cov clean
cargo llvm-cov --html                 # HTML: target/llvm-cov/html/index.html
cargo llvm-cov --html --open          # HTML and open it in a browser
cargo llvm-cov                        # text summary in the terminal
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --json --output-path coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

The shared report script writes `coverage.json`, `lcov.info`, `coverage.txt`, `target/llvm-cov/cobertura.xml`, and `target/llvm-cov/html/index.html` in the repository root. Its CI workflow publishes these under the `coverage-reports` artifact. The direct `cargo llvm-cov` commands shown above use the output paths supplied on each command; HTML defaults to `target/llvm-cov/html`.

## Scope and configuration

The coverage tool reads `.infra/coverage/coverage.json` when present; without it, the tool's defaults apply. This repository currently has no such project-specific file. The checked-in `.llvm-cov.toml` was not read by the configured tool or by `cargo-llvm-cov 0.9.1`, and has been removed. `cargo-llvm-cov` excludes common test code locations by default; inspect its own documentation for its current behavior.

The CI coverage scope, exclusions, and thresholds are controlled by the shared coverage configuration and workflow. The local entry point and workflow are [`.infra/bin/coverage.sh`](.infra/bin/coverage.sh) and [`.github/workflows/ci.yml`](.github/workflows/ci.yml); this repository delegates CI to `qubit-ltd/rs-infra-ci/.github/workflows/github-ci.yml@main`.

## Troubleshooting

If `cargo llvm-cov` is unavailable, install it with `cargo install cargo-llvm-cov`. Run `cargo llvm-cov clean` before collecting again if stale coverage artifacts affect a local run.

## References

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
