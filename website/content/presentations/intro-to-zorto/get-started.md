+++
title = "Get started"
weight = 90
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", bottom="20px", right="20px", width="100px", opacity="0.6", class="logo-fade-in") }}

## Get started

```bash
# Install
uv tool install zorto

# Create a new site
zorto init mysite
cd mysite

# Preview with live reload
zorto preview --open
```

Or use as a Python library:

```python
import zorto
zorto.build(".")
```
