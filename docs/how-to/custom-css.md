# Customize styles with SCSS and CSS

Override colors, fonts, spacing, and layout without forking a theme.

All built-in themes use CSS custom properties (variables) for colors and layout. Override them in a custom stylesheet to change the look of your site without replacing any templates.

## CSS variables available in all themes

Every built-in theme defines these variables on `:root` (dark mode) and `[data-theme="light"]` (light mode):

| Variable | Purpose | Example value |
|----------|---------|---------------|
| `--accent` | Primary accent color (links, highlights) | `#3b82f6` |
| `--accent-alpha-70` | Accent at 70% opacity | `rgba(59, 130, 246, .7)` |
| `--accent-alpha-20` | Accent at 20% opacity | `rgba(59, 130, 246, .2)` |
| `--accent-secondary` | Secondary accent (hover states) | `#22d3ee` |
| `--accent-secondary-alpha-20` | Secondary accent at 20% opacity | `rgba(34, 211, 238, .2)` |
| `--background` | Page background | `#0f172a` |
| `--background-raised` | Raised surface (cards, nav) | `#1e293b` |
| `--color` | Primary text color | `#e2e8f0` |
| `--color-muted` | Secondary text (captions, metadata) | `rgba(226, 232, 240, .6)` |
| `--border-color` | Border and divider lines | `rgba(59, 130, 246, .15)` |
| `--code-bg` | Code block background | `rgba(0, 0, 0, 0.3)` |
| `--max-width` | Maximum content width | `1200px` |

## Override with a custom stylesheet

Create `sass/custom.scss` in your project root:

```scss
// sass/custom.scss
:root {
  --accent: #e74c3c;
  --background: #1a1a2e;
  --color: #eaeaea;
  --max-width: 900px;
}
```

Zorto compiles SCSS to CSS at build time — `sass/custom.scss` becomes `/custom.css` in the output. Load it via the `extra_head` template block:

```html
{% extends "base.html" %}

{% block extra_head %}
  <link rel="stylesheet" href="/custom.css">
{% endblock %}
```

This approach layers your styles on top of the theme's defaults. You only override what you need.

## Light and dark mode patterns

Themes default to dark mode on `:root` and define light mode overrides on `[data-theme="light"]`. To customize both modes:

```scss
// sass/custom.scss

// Dark mode (default)
:root {
  --accent: #ff6b6b;
  --background: #1e1e2e;
  --background-raised: #2a2a3e;
  --color: #cdd6f4;
  --color-muted: rgba(205, 214, 244, .6);
  --border-color: rgba(255, 107, 107, .15);
  --code-bg: rgba(0, 0, 0, 0.3);
}

// Light mode
[data-theme="light"] {
  --background: #ffffff;
  --background-raised: #f5f5f5;
  --color: #1e1e2e;
  --color-muted: rgba(30, 30, 46, .6);
  --border-color: #e0e0e0;
  --code-bg: #f1f5f9;
}
```

The theme's JavaScript toggles the `data-theme` attribute on the `<html>` element. Your CSS variables respond automatically — no JavaScript changes needed.

## Replace the theme stylesheet entirely

If you want full control, create `sass/style.scss` in your project. This replaces the theme's `style.scss` entirely (local files overlay theme files by filename). You will need to provide all the styles yourself, including layout, typography, and responsive breakpoints.

> [!TIP]
> Start by copying a theme's `style.scss` as a starting point and modify from there. Theme stylesheets import shared partials from `_structure.scss` and `_components.scss`.

## SCSS features

Zorto compiles SCSS, so you can use:

- **Variables**: `$phone-max-width: 683px;`
- **Nesting**: `.navbar { &__inner { ... } }`
- **Partials and imports**: `@import 'custom-components';`
- **Mixins and functions**: standard Sass features

Create additional partials in `sass/` and import them from your main stylesheet.

## Add custom fonts

Load fonts via the `extra_head` block or import them in your SCSS:

```scss
// sass/custom.scss
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap');

body {
  font-family: "Inter", sans-serif;
}
```

Or use locally hosted fonts from the `static/` directory:

```scss
@font-face {
  font-family: "CustomFont";
  src: url("/fonts/custom.woff2") format("woff2");
  font-display: swap;
}

body {
  font-family: "CustomFont", sans-serif;
}
```

## Related guides

- [Customize your theme](customize-theme.md) — template overrides, shortcodes, and the `extra_head` block
- [Themes](../concepts/themes.md) — how the theme system works
- [Asset management](assets.md) — serving fonts and other static files
