//! Build trigger and preview rendering.

use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;

use crate::{AppState, escape};

pub async fn trigger(State(state): State<Arc<AppState>>) -> Html<String> {
    match crate::rebuild_site(&state) {
        Ok(()) => Html(r#"<span style="color: #34d399;">Built successfully.</span>"#.to_string()),
        Err(e) => Html(format!(
            r#"<span style="color: #f87171;">Build error: {}</span>"#,
            escape(&e)
        )),
    }
}

pub async fn render_preview(body: String) -> Html<String> {
    // Simple markdown preview — strip frontmatter and render
    let content = strip_frontmatter(&body);
    let mut blocks = Vec::new();
    let md_config = zorto_core::config::MarkdownConfig::default();
    let html = zorto_core::markdown::render_markdown(&content, &md_config, &mut blocks, "");
    Html(html)
}

fn strip_frontmatter(content: &str) -> String {
    let trimmed = content.trim();
    if let Some(rest) = trimmed.strip_prefix("+++") {
        if let Some(end) = rest.find("\n+++") {
            return rest[end + 4..].to_string();
        }
    }
    content.to_string()
}
