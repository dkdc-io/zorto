# zorto-cli

CLI binary and preview server for zorto. Published as `zorto` on crates.io.

## Binary

`zorto` — build static sites, serve with live reload.

## Purpose

Main entry point for the static site generator. Includes clap CLI, dev server with file watching, and build commands.

## Key constraints

- No dkdc-* dependencies — zorto is independent
- Published to crates.io — maintain semver

## Testing

```bash
cargo test -p zorto
```
