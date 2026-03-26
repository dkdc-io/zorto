//! Asset management routes.

use axum::extract::{Multipart, State};
use axum::response::Html;
use std::sync::Arc;

use crate::html;
use crate::{AppState, escape};

pub async fn list(State(state): State<Arc<AppState>>) -> Html<String> {
    let site_title = state.site_title();
    let static_dir = state.root.join("static");

    let mut file_items = Vec::new();
    if static_dir.exists() {
        for entry in walkdir::WalkDir::new(&static_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let relative = path
                .strip_prefix(&static_dir)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            let size = std::fs::metadata(path)
                .map(|m| format_size(m.len()))
                .unwrap_or_default();

            let ext = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            let icon = match ext.as_str() {
                "png" | "jpg" | "jpeg" | "gif" | "svg" | "ico" | "webp" => "🖼",
                "css" | "scss" => "🎨",
                "js" => "📜",
                "pdf" => "📄",
                "woff" | "woff2" | "ttf" | "otf" => "🔤",
                _ => "📁",
            };

            file_items.push(format!(
                r#"<li><span class="file-icon">{icon}</span> <span>{path}</span> <span style="margin-left: auto; color: #666680; font-size: 0.8rem;">{size}</span></li>"#,
                path = escape(&relative),
            ));
        }
    }

    file_items.sort();
    let file_list = file_items.join("\n");

    let body = format!(
        r#"<h2>Assets</h2>
<div class="card">
  <h3>Upload</h3>
  <form method="POST" action="/assets/upload" enctype="multipart/form-data" style="margin-top: 12px;">
    <div style="display: flex; gap: 8px; align-items: center;">
      <input type="file" name="file" style="flex: 1;" required>
      <input type="text" name="subdir" placeholder="subdirectory (optional)" style="width: 200px;">
      <button type="submit" class="btn btn-primary">Upload</button>
    </div>
  </form>
</div>

<div class="card">
  <h3>Static Files</h3>
  <ul class="file-list" style="margin-top: 12px;">
    {file_list}
  </ul>
</div>"#
    );

    Html(html::page("Assets", &site_title, "assets", &body))
}

pub async fn upload(State(state): State<Arc<AppState>>, mut multipart: Multipart) -> Html<String> {
    let static_dir = state.root.join("static");
    let mut subdir = String::new();
    let mut file_saved = false;
    let mut error_msg = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();

        if name == "subdir" {
            subdir = field.text().await.unwrap_or_default();
        } else if name == "file" {
            let filename = field.file_name().unwrap_or("upload").to_string();

            // Reject suspicious filenames
            if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
                error_msg = Some("Invalid filename".to_string());
                continue;
            }

            let dest_dir = if subdir.is_empty() {
                static_dir.clone()
            } else {
                static_dir.join(&subdir)
            };
            let _ = std::fs::create_dir_all(&dest_dir);

            match field.bytes().await {
                Ok(bytes) => {
                    let dest = dest_dir.join(&filename);
                    match std::fs::write(&dest, &bytes) {
                        Ok(()) => file_saved = true,
                        Err(e) => error_msg = Some(e.to_string()),
                    }
                }
                Err(e) => error_msg = Some(e.to_string()),
            }
        }
    }

    let flash = if file_saved {
        r#"<div class="flash flash-success">File uploaded.</div>"#.to_string()
    } else if let Some(err) = error_msg {
        format!(
            r#"<div class="flash flash-error">Upload error: {}</div>"#,
            escape(&err)
        )
    } else {
        r#"<div class="flash flash-error">No file received.</div>"#.to_string()
    };

    // Re-render the full asset list with flash
    let full = list(State(state))
        .await
        .0
        .replace("<h2>Assets</h2>", &format!("{flash}<h2>Assets</h2>"));

    Html(full)
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes} B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
