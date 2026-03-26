import sys

from zorto.core import run_cli as _run_cli

__all__ = ["run_cli", "main"]


def run_cli(argv: list[str] | None = None) -> None:
    """Run the zorto CLI with the given arguments."""
    if argv is None:
        argv = sys.argv
    try:
        _run_cli(argv)
    except KeyboardInterrupt:
        sys.exit(130)


def main() -> None:
    """CLI entry point."""
    run_cli()
