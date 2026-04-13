+++
title = "Backgrounds"
weight = 75

[extra]
background_image = "/zorto-mark-transparent.png"
background_size = "420px"
background_opacity = "0.14"
background_color = "#0b1020"
reveal_theme = "moon"
+++

## Full-slide backgrounds

- **`background_image`** -- any URL, sized and positioned by reveal.js
- **`background_color`** -- solid fill with hex or CSS color names
- **`background_size`** / **`background_opacity`** -- tune the watermark
- **`reveal_theme`** -- swap the slide's reveal.js theme (this one is `moon`)

Everything lives in the slide's `[extra]` table -- no template edits required.

{% speaker_notes() %}
This slide demonstrates per-slide theme switching via a custom JS hook in
presentation.html. Set `reveal_theme = "..."` and the theme stylesheet is
swapped on slidechanged. Works with any reveal.js 5.x theme.
{% end %}
