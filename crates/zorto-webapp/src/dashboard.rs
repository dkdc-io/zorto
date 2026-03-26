//! Dashboard page.

use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;

use crate::AppState;
use crate::html;

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let site_title = state.site_title();
    let content_dir = state.root.join("content");

    let mut page_count = 0;
    let mut section_count = 0;
    let mut draft_count = 0;

    if content_dir.exists() {
        for entry in walkdir::WalkDir::new(&content_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name == "_index.md" {
                    section_count += 1;
                } else if name.ends_with(".md") {
                    page_count += 1;
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if content.contains("draft = true") {
                            draft_count += 1;
                        }
                    }
                }
            }
        }
    }

    let static_dir = state.root.join("static");
    let asset_count = if static_dir.exists() {
        walkdir::WalkDir::new(&static_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .count()
    } else {
        0
    };

    let body = format!(
        r#"<h2>Dashboard</h2>
<div class="card">
  <div class="stat">
    <div class="stat-num">{page_count}</div>
    <div class="stat-label">Pages</div>
  </div>
  <div class="stat">
    <div class="stat-num">{section_count}</div>
    <div class="stat-label">Sections</div>
  </div>
  <div class="stat">
    <div class="stat-num">{draft_count}</div>
    <div class="stat-label">Drafts</div>
  </div>
  <div class="stat">
    <div class="stat-num">{asset_count}</div>
    <div class="stat-label">Assets</div>
  </div>
</div>

<div class="card">
  <h3>Quick Actions</h3>
  <div style="display: flex; gap: 8px; margin-top: 12px;">
    <a href="/pages/new" class="btn btn-primary">New Page</a>
    <a href="/config" class="btn">Edit Config</a>
    <a href="/assets" class="btn">Manage Assets</a>
  </div>
</div>

<div class="card">
  <h3>Site Root</h3>
  <div style="font-size: 0.85rem; color: #666680; font-family: monospace; margin-top: 8px;">
    {root}
  </div>
</div>"#,
        root = crate::escape(&state.root.display().to_string()),
    );

    Html(html::page("Dashboard", &site_title, "dashboard", &body))
}
