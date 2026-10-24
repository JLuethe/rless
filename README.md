# rless

A planned Rust replacement for [less](https://github.com/gwsw/less), preserving
its command-line interface, interactive commands, and observable behavior.

The compatibility target is **less 712**, pinned to upstream commit
`7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e`.

The project is currently in requirements and design. The
[version 1 specification package](spec/README.md) defines the Linux/GNU less 712
target, with detailed contracts and executable reference acceptance cases.
No pager implementation exists yet.

- [Analysis and delivery plan](docs/less-analysis-plan.md)
- [Compatibility specification](docs/rless-compatibility-spec.md)
- [Upstream interface inventory](docs/less-interface-inventory.md)
- [Source audit and evidence](docs/less-source-audit.md)
- [Specification readiness review](docs/less-spec-review.md)
- [Plan to complete the specification](docs/spec-completion-plan.md)
- [Specification readiness and validation](spec/readiness.md)
