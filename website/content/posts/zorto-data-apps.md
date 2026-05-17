+++
title = "Zorto as a data app builder"
date = "2026-05-16"
author = "Cody"
description = "Search, analytics, and the first DuckDB-backed data app pattern on zorto.dev."
tags = ["zorto", "duckdb", "data"]
template = "data-app-post.html"
+++

[Zorto](/) now ships a public DuckDB database with the site.

<!-- more -->

[`/data/site.ddb`](/data/site.ddb) is generated from local repo and build metadata. [Search](/?q=zorto) reads from it. The [analytics dashboard](/analytics/) reads from it. Static hosting serves it as a file; the browser attaches it read-only when a page needs data.

No visitor analytics. No tracking, cookies, tokens, or third-party event stream. The database contains site metadata: commits, packages, content files, links, build outputs, search rows, and pipeline receipts.

{{ flow(steps="Build:Rust CLI renders the site|Generate:uv script writes site.ddb|Ship:static hosting serves HTML and data|Query:DuckDB-Wasm runs in the browser", caption="Local pipeline. Static artifact. Browser query.") }}

## Current slice

{% tree(caption="The zorto.dev data-app files today.") %}
website/
  bin/build-meta  [uv script]
  data/meta.toml  [pipeline config]
  data/analytics.toml  [dashboard config]
  static/data/site.ddb  [public DuckDB file]
  static/data/analytics-dashboard.json  [runtime manifest]
  static/js/analytics-dashboard.js  [browser app]
  content/analytics/_index.md  [page]
{% end %}

The dashboard page is a thin shell. On click it lazy-loads [DuckDB-Wasm](https://duckdb.org/docs/current/clients/wasm/overview) and [Plotly.js](https://plotly.com/javascript/), fetches `/data/site.ddb`, attaches it read-only, runs configured SQL, then renders charts and tables.

Search uses the same database. That matters more than the dashboard itself: Zorto can ship one public data artifact and let different browser surfaces query it.

One panel is just SQL over the attached database:

```sql
SELECT kind, count(*) AS files, sum(bytes) AS bytes
FROM site.main.build_outputs
GROUP BY kind
ORDER BY bytes DESC;
```

## Layers

{{ layers(items="Content:Markdown owns pages and explanations:website/content|Config:TOML owns sources, limits, panels, queries:website/data|Code:Rust, Python, SQL, HTML, CSS, and JS do the work:crates + pipelines + static/js", caption="The editing contract stays small.") }}

This follows the separation I want for Zorto:

- Content: Markdown owns the page.
- Config: TOML owns the data app shape.
- Code: Rust, Python, SQL, HTML, CSS, and JavaScript handle execution.

Humans get stable files to edit. Agents get boundaries. The repo stays legible.

## Links

<div class="data-post-link-grid" aria-label="Implementation links">
  <a href="/analytics/">
    <span>Dashboard</span>
    <strong>/analytics/</strong>
  </a>
  <a href="/data/site.ddb">
    <span>Database</span>
    <strong>/data/site.ddb</strong>
  </a>
  <a href="/data/analytics-dashboard.json">
    <span>Manifest</span>
    <strong>analytics-dashboard.json</strong>
  </a>
  <a href="https://github.com/dkdc-io/zorto/blob/main/website/bin/build-meta">
    <span>Pipeline</span>
    <strong>website/bin/build-meta</strong>
  </a>
  <a href="https://github.com/dkdc-io/zorto/blob/main/website/data/meta.toml">
    <span>Metadata config</span>
    <strong>website/data/meta.toml</strong>
  </a>
  <a href="https://github.com/dkdc-io/zorto/blob/main/website/data/analytics.toml">
    <span>Dashboard config</span>
    <strong>website/data/analytics.toml</strong>
  </a>
</div>

## Implementation

`website/bin/build-meta` is a self-contained `uv` script. It uses DuckDB, runs a timed current-code Zorto build through the Rust CLI, performs privacy checks, writes a temporary database, validates it, then atomically replaces `website/static/data/site.ddb`.

`website/data/meta.toml` owns the pipeline settings: output path, build output directory, collection limits, content include and exclude rules, privacy checks, and the build command.

`website/data/analytics.toml` owns the dashboard: views, panels, KPIs, SQL queries, table columns, and runtime assets.

The database includes `pipeline_steps`, so generation leaves receipts: step name, kind, status, duration, output count, command, and details.

## The app surface

A dashboard is data plus frontend.

Python is good for data work: pulls, transforms, validation, orchestration. Many Python dashboard tools eventually emit JavaScript anyway. Zorto keeps the split direct:

- Build data locally with `uv`, Python, and DuckDB.
- Ship `.ddb` files beside static HTML.
- Render the interface with HTML, CSS, and JavaScript.
- Query in the browser with DuckDB-Wasm when embedded data is enough.
- Use remote DuckDB when live data is worth it.

That can stay static or grow into dynamic browser interfaces. The default remains standards first: files, SQL, HTML, CSS, JavaScript, and Rust where the generator needs to be solid.

## Boundaries

This is a zorto.dev implementation, not a public Zorto data API.

There is no `[data]` config in Zorto core yet. There is no automatic pipeline hook in `zorto build`. DuckDB-Wasm and Plotly are pinned CDN-loaded runtime assets for now.

## Next

- Promote the stable pieces into Zorto after zorto.dev proves them.
- Keep `uv` as the local Python orchestration layer.
- Keep DuckDB as the local database layer.
- Use DuckDB's beta [Quack protocol](https://duckdb.org/quack/) for remote DuckDB when live access is useful.
- Use [DuckLake](https://ducklake.select/) when the data wants lakehouse-style partitioning instead of a single DuckDB database file.

Zorto remains an AI-native static site generator with executable code blocks. The `& more` is now visible: data apps built from small files, local pipelines, and browser-native interfaces.
