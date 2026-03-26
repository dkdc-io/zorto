//! Config editor routes.

use axum::extract::State;
use axum::response::Html;
use std::sync::Arc;

use crate::html;
use crate::{AppState, escape};

pub async fn edit(State(state): State<Arc<AppState>>) -> Html<String> {
    let site_title = state.site_title();
    let config_path = state.root.join("config.toml");
    let content = std::fs::read_to_string(&config_path).unwrap_or_default();

    let body = format!(
        r#"<h2>Site Configuration</h2>
<form method="POST" action="/config">
  <div class="card">
    <div class="form-group">
      <label>config.toml</label>
      <textarea name="content" rows="30">{escaped_content}</textarea>
    </div>
    <button type="submit" class="btn btn-primary">Save Config</button>
  </div>
</form>"#,
        escaped_content = escape(&content),
    );

    Html(html::page("Config", &site_title, "config", &body))
}

pub async fn save(
    State(state): State<Arc<AppState>>,
    axum::Form(form): axum::Form<SaveForm>,
) -> Html<String> {
    let config_path = state.root.join("config.toml");

    // Validate TOML before saving
    let flash = match toml::from_str::<toml::Value>(&form.content) {
        Ok(_) => match std::fs::write(&config_path, &form.content) {
            Ok(()) => {
                let _ = rebuild_site(&state);
                r#"<div class="flash flash-success">Config saved and site rebuilt.</div>"#
                    .to_string()
            }
            Err(e) => format!(
                r#"<div class="flash flash-error">Error writing: {}</div>"#,
                escape(&e.to_string())
            ),
        },
        Err(e) => format!(
            r#"<div class="flash flash-error">Invalid TOML: {}</div>"#,
            escape(&e.to_string())
        ),
    };

    let site_title = state.site_title();
    let content = std::fs::read_to_string(&config_path).unwrap_or_default();

    let body = format!(
        r#"{flash}
<h2>Site Configuration</h2>
<form method="POST" action="/config">
  <div class="card">
    <div class="form-group">
      <label>config.toml</label>
      <textarea name="content" rows="30">{escaped_content}</textarea>
    </div>
    <button type="submit" class="btn btn-primary">Save Config</button>
  </div>
</form>"#,
        escaped_content = escape(&content),
    );

    Html(html::page("Config", &site_title, "config", &body))
}

#[derive(serde::Deserialize)]
pub struct SaveForm {
    content: String,
}

use crate::rebuild_site;
