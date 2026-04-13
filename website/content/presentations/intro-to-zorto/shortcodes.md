+++
title = "Shortcodes"
weight = 85

[extra]
reveal_theme = "league"
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", top="20px", right="20px", width="72px") }}

## Shortcodes: rich content without HTML

{% columns(widths="52%|48%") %}

### Built-in

- **`slide_image`** -- positioned or inline images
- **`fragment`** -- progressive reveal
- **`columns`** -- side-by-side layout
- **`speaker_notes`** -- reveal.js speaker view

<!-- column -->

### Custom

- Drop a `.tera` template in `shortcodes/`
- Call it from any markdown page
- Fully sandboxed, deterministic
- Tested via `zorto check`

{% end %}

{% fragment(style="highlight-green") %}
**The whole presentation you're watching** is rendered from these shortcodes.
{% end %}
