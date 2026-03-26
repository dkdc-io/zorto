//! Page shell, CSS, and navigation for the webapp.

use crate::escape;

const CSS: &str = r#"
* { margin: 0; padding: 0; box-sizing: border-box; }
html { background: #111118; }
body { font-family: system-ui, -apple-system, sans-serif; background: #111118; color: #c8c8d8; min-height: 100vh; display: flex; }
a { color: #60a5fa; text-decoration: none; }
a:hover { text-decoration: underline; }
.sidebar { width: 220px; background: #16161f; border-right: 1px solid #2a2a3a; padding: 24px 16px; flex-shrink: 0; min-height: 100vh; }
.sidebar h1 { font-size: 1.1rem; color: #60a5fa; margin-bottom: 4px; font-weight: 600; }
.sidebar .site-title { font-size: 0.8rem; color: #666680; margin-bottom: 24px; }
.sidebar nav a { display: block; padding: 8px 12px; border-radius: 6px; color: #8c8ca6; font-size: 0.9rem; margin-bottom: 2px; }
.sidebar nav a:hover { background: #1e1e2e; color: #c8c8d8; text-decoration: none; }
.sidebar nav a.active { background: #1e1e2e; color: #60a5fa; }
.main { flex: 1; padding: 32px 40px; max-width: 960px; }
.main h2 { font-size: 1.3rem; color: #e0e0f0; margin-bottom: 20px; font-weight: 500; }
.card { background: #16161f; border: 1px solid #2a2a3a; border-radius: 8px; padding: 20px; margin-bottom: 16px; }
.card h3 { font-size: 1rem; color: #c8c8d8; margin-bottom: 8px; }
.stat { display: inline-block; margin-right: 24px; }
.stat-num { font-size: 1.5rem; color: #60a5fa; font-weight: 600; }
.stat-label { font-size: 0.8rem; color: #666680; text-transform: uppercase; letter-spacing: 0.05em; }
table { width: 100%; border-collapse: collapse; }
th { text-align: left; font-size: 0.75rem; color: #666680; text-transform: uppercase; letter-spacing: 0.05em; padding: 8px 12px; border-bottom: 1px solid #2a2a3a; }
td { padding: 8px 12px; border-bottom: 1px solid #1e1e2e; font-size: 0.9rem; }
tr:hover { background: #1a1a26; }
.btn { display: inline-block; background: #1e1e2e; border: 1px solid #2a2a3a; color: #c8c8d8; padding: 8px 16px; border-radius: 6px; cursor: pointer; font-size: 0.85rem; }
.btn:hover { border-color: #60a5fa; color: #60a5fa; text-decoration: none; }
.btn-primary { background: #1e3a5f; border-color: #60a5fa; color: #60a5fa; }
.btn-primary:hover { background: #264d7a; }
.btn-success { background: #1a3a2a; border-color: #34d399; color: #34d399; }
.btn-success:hover { background: #1f4a35; }
.btn-sm { padding: 4px 10px; font-size: 0.8rem; }
textarea { width: 100%; background: #111118; border: 1px solid #2a2a3a; border-radius: 6px; color: #c8c8d8; padding: 12px; font-family: 'SF Mono', 'Fira Code', monospace; font-size: 0.85rem; resize: vertical; }
textarea:focus { outline: none; border-color: #60a5fa; }
input[type="text"], input[type="date"], select { background: #111118; border: 1px solid #2a2a3a; border-radius: 6px; color: #c8c8d8; padding: 8px 12px; font-size: 0.85rem; width: 100%; }
input:focus, select:focus { outline: none; border-color: #60a5fa; }
label { display: block; font-size: 0.8rem; color: #8c8ca6; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 0.05em; }
.form-group { margin-bottom: 16px; }
.form-row { display: flex; gap: 16px; }
.form-row > * { flex: 1; }
.toolbar { display: flex; gap: 8px; align-items: center; margin-bottom: 20px; }
.toolbar-right { margin-left: auto; }
.badge { display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 0.75rem; }
.badge-draft { background: #3a2a1a; color: #fbbf24; }
.badge-section { background: #1a2a3a; color: #60a5fa; }
.flash { padding: 12px 16px; border-radius: 6px; margin-bottom: 16px; font-size: 0.85rem; }
.flash-success { background: #1a3a2a; border: 1px solid #34d399; color: #34d399; }
.flash-error { background: #3a1a1a; border: 1px solid #f87171; color: #f87171; }
.preview-panel { background: #1a1a26; border: 1px solid #2a2a3a; border-radius: 6px; padding: 16px; min-height: 200px; }
.preview-panel h1, .preview-panel h2, .preview-panel h3, .preview-panel h4 { color: #e0e0f0; margin: 1em 0 0.5em; }
.preview-panel h1 { font-size: 1.5rem; }
.preview-panel h2 { font-size: 1.25rem; }
.preview-panel h3 { font-size: 1.1rem; }
.preview-panel p { margin: 0.5em 0; line-height: 1.6; }
.preview-panel ul, .preview-panel ol { margin: 0.5em 0 0.5em 1.5em; }
.preview-panel li { margin: 0.25em 0; }
.preview-panel code { background: #242438; padding: 2px 6px; border-radius: 3px; font-size: 0.85em; }
.preview-panel pre { background: #0d0d14; border: 1px solid #2a2a3a; border-radius: 6px; padding: 12px; overflow-x: auto; margin: 0.5em 0; }
.preview-panel pre code { background: none; padding: 0; }
.preview-panel blockquote { border-left: 3px solid #60a5fa; padding-left: 12px; color: #8c8ca6; margin: 0.5em 0; }
.preview-panel img { max-width: 100%; border-radius: 6px; }
.preview-panel a { color: #60a5fa; }
.preview-panel table { width: 100%; border-collapse: collapse; margin: 0.5em 0; }
.preview-panel th, .preview-panel td { border: 1px solid #2a2a3a; padding: 6px 10px; text-align: left; }
.preview-panel th { background: #1e1e2e; }
.editor-layout { display: flex; gap: 20px; }
.editor-layout > .editor-pane { flex: 1; min-width: 0; }
.file-list { list-style: none; }
.file-list li { padding: 6px 12px; border-bottom: 1px solid #1e1e2e; font-size: 0.85rem; display: flex; align-items: center; gap: 8px; }
.file-list li:hover { background: #1a1a26; }
.file-icon { color: #666680; }
"#;

pub fn page(title: &str, site_title: &str, active: &str, body: &str) -> String {
    let nav_items = [
        ("dashboard", "/", "Dashboard"),
        ("pages", "/pages", "Pages"),
        ("sections", "/sections", "Sections"),
        ("config", "/config", "Config"),
        ("assets", "/assets", "Assets"),
    ];

    let nav_html: String = nav_items
        .iter()
        .map(|(id, href, label)| {
            let class = if *id == active { " active" } else { "" };
            format!(r#"<a href="{href}" class="{class}">{label}</a>"#)
        })
        .collect::<Vec<_>>()
        .join("\n      ");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title} — zorto</title>
  <script src="https://unpkg.com/htmx.org@2.0.4"></script>
  <style>{CSS}</style>
</head>
<body>
  <div class="sidebar">
    <h1>zorto</h1>
    <div class="site-title">{site_title}</div>
    <nav>
      {nav_html}
    </nav>
    <div style="margin-top: auto; padding-top: 24px;">
      <form method="POST" action="/build">
        <button type="submit" class="btn btn-success" style="width: 100%;"
                hx-post="/build" hx-target="#build-status" hx-swap="innerHTML">
          Build Site
        </button>
      </form>
      <div id="build-status" style="margin-top: 8px; font-size: 0.8rem; color: #666680;"></div>
    </div>
  </div>
  <div class="main">
    {body}
  </div>
</body>
</html>"##,
        title = escape(title),
        site_title = escape(site_title),
    )
}
