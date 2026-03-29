+++
title = "Deploy to Netlify"
+++

## 1. Build your site

Make sure your site builds locally:

```bash
zorto build
```

The output goes to `public/`.

## 2. Add netlify.toml

Create `netlify.toml` in your repository root:

```toml
[build]
command = "pip install zorto && zorto build"
publish = "public"

[build.environment]
PYTHON_VERSION = "3.12"
```

Alternatively, install via Rust:

```toml
[build]
command = "cargo install zorto && zorto build"
publish = "public"
```

> [!TIP]
> The Python install is faster on Netlify since it downloads a prebuilt wheel. Cargo install compiles from source.

## 3. Connect your repository

1. Log into [Netlify](https://app.netlify.com)
2. Click **Add new site** > **Import an existing project**
3. Select your Git provider and repository
4. Netlify detects `netlify.toml` automatically
5. Click **Deploy site**

## 4. Custom domain

In **Site settings** > **Domain management**, add your custom domain and configure DNS as directed.

Every push to your main branch triggers a new deploy.
