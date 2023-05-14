# Pre-commit sort

[![CI](https://github.com/nim65s/pre-commit-sort/actions/workflows/ci.yml/badge.svg)](https://github.com/nim65s/pre-commit-sort/actions/workflows/ci.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/nim65s/pre-commit-sort/main.svg)](https://results.pre-commit.ci/latest/github/nim65s/pre-commit-sort/main)

Rust validation, sort, deduplication and formatting for `.pre-commit-config.yaml` and `.pre-commit-hooks.yaml` files:

```yaml
- repo: https://github.com/nim65s/pre-commit-sort
  rev: v0.0.1
  hooks:
  - id: pre-commit-sort
```

(or directly run the `pre-commit-sort` command if you already have it somewhere)

## Installation

pre-commit will install this project automatically for you, but if you need a standalone version:

```
cargo binstall pre-commit-sort
```
