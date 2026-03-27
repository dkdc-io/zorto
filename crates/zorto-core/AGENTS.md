# zorto-core

Core library for the zorto static site generator. Pure library — no CLI, no server.

## Purpose

Site model, build pipeline, Markdown parsing, Tera template rendering, executable code blocks. All other zorto crates depend on this.

## Features

- `embed-python` (default) — embeds Python via PyO3 for executable code blocks
- `python` — PyO3 dependency without auto-initialize

## Key constraints

- No dkdc-* dependencies — zorto is an independent project
- No axum/tokio/clap — those belong in zorto-cli or zorto-webapp

## Testing

```bash
cargo test -p zorto-core
```
