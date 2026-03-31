# zorto.dev

Project site for [Zorto](https://github.com/dkdc-io/zorto) — the AI-native static site generator (SSG) with executable code blocks. Built with zorto.

## Commands

- `bin/preview`: Start dev server with auto-open browser
- `zorto build`: Build site to `public/`

## Styling

Dark mode default with light mode toggle. Color scheme: violet/purple neon + cyan accents. Same theme as dkdc.io / dkdc.sh.

## Content style

- Use sentence casing for headings and nav items (capitalize first word only)
- Use "Zorto" (capitalized) in prose; lowercase `zorto` for commands/code/URLs

## Docs structure

Documentation lives as plain markdown at the repo root (`docs/`), readable on GitHub without zorto-specific syntax. Website content pages in `website/content/docs/` are thin wrappers with frontmatter + `{{ include(path="../docs/...", rewrite_links="true") }}`. Two exceptions stay as full website content: `reference/cli.md` (executable code blocks) and `reference/shortcodes.md` (live shortcode demos).

Docs at repo root must NOT use `+++` frontmatter, `@/` links, or zorto shortcodes. Use relative `.md` links for cross-references (the `rewrite_links` parameter converts them to clean URLs on the website).
