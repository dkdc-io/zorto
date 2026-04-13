+++
title = "One file per slide"
weight = 80

[extra]
layout = "image-left"
+++

{{ slide_image(src="/zorto-logo-dark.png", alt="Zorto", width="340px", class="logo-sway") }}

## Presentations: one file per slide

- Each slide is a markdown file with its own frontmatter
- `weight` field controls slide order
- `[extra]` controls background, transitions, and layout
- Shortcodes for columns, fragments, speaker notes, and positioned images
- Powered by **reveal.js** -- keyboard navigation, fullscreen, speaker view
