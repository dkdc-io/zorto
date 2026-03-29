+++
title = "Quick start"
template = "docs.html"
+++

## Create a new site

```bash
zorto init mysite
```

## Preview locally

```bash
cd mysite
zorto preview
```

Open `http://127.0.0.1:1111` in your browser, or use `zorto preview --open` to open it automatically. Edit any file in `content/` and the page reloads instantly.

## Build for production

```bash
zorto build
```

The output lands in `public/`. Deploy that directory to any static host.
