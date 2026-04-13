+++
title = "Images, anywhere"
weight = 45

[extra]
background_color = "#0f172a"
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="", top="16px",    left="16px",   width="48px", opacity="0.7", class="logo-float") }}
{{ slide_image(src="/zorto-mark-transparent.png", alt="", top="16px",    right="16px",  width="48px", opacity="0.7", class="logo-sway")  }}
{{ slide_image(src="/zorto-mark-transparent.png", alt="", bottom="16px", left="16px",   width="48px", opacity="0.7", class="logo-pulse") }}
{{ slide_image(src="/zorto-mark-transparent.png", alt="", bottom="16px", right="16px",  width="48px", opacity="0.7", class="logo-glow")  }}

## Images, anywhere

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", width="130px", class="logo-pulse") }}

- **`slide_image`** shortcode -- absolute or inline
- Position with `top` / `left` / `right` / `bottom`
- Animate with `class="logo-float logo-pulse logo-sway logo-glow"`
- Opacity, width, height -- all optional
