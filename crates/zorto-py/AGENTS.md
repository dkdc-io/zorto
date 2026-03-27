# zorto-py

PyO3 bindings for zorto. Own workspace (excluded from parent).

## Purpose

Exposes zorto functionality to Python via `zorto_core` cdylib. Built with maturin. Requires Python 3.11+.

## Dependencies

zorto-cli (no default features, python feature), pyo3.

## Building

```bash
cd projects/zorto/py && uv run maturin develop
```
