+++
title = "Progressive reveal"
weight = 65
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", top="20px", right="20px", width="72px") }}

## Progressive reveal

Press → or space to reveal one at a time:

{% fragment(style="fade-in") %}
- **fade-in** -- the default, gentle entrance
{% end %}

{% fragment(style="fade-up") %}
- **fade-up** -- slides in from below
{% end %}

{% fragment(style="grow") %}
- **grow** -- scales up to emphasize
{% end %}

{% fragment(style="highlight-blue") %}
- **highlight-blue** -- flashes a color highlight
{% end %}

{% fragment(style="fade-right") %}
- **fade-right** -- slides in from the left
{% end %}

{% speaker_notes() %}
Fragments are just reveal.js classes, driven by the `fragment()` shortcode.
All standard styles work: fade-in/out/up/down/left/right, grow, shrink,
strike, highlight-red/blue/green.
{% end %}
