//! Zorto webapp — HTMX-based local CMS for managing zorto sites.

use axum::Router;
use axum::routing::{get, post};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::broadcast;

mod assets;
mod build;
mod config;
mod dashboard;
mod html;
mod pages;
mod sections;

pub(crate) struct AppState {
    pub root: PathBuf,
    pub output_dir: PathBuf,
    pub sandbox: Option<PathBuf>,
    pub reload_tx: broadcast::Sender<()>,
}

impl AppState {
    fn site_title(&self) -> String {
        let config_path = self.root.join("config.toml");
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str::<toml::Value>(&content) {
                if let Some(title) = config.get("title").and_then(|v| v.as_str()) {
                    return title.to_string();
                }
            }
        }
        "Zorto Site".to_string()
    }
}

/// Run the zorto webapp server.
///
/// Starts an HTMX-based CMS webapp for managing the site at the given root directory.
/// The optional `sandbox` path allows file operations (like include shortcodes) to
/// access files outside the site root within the sandbox boundary.
pub fn run_webapp(root: &Path, output_dir: &Path, sandbox: Option<&Path>) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let port: u16 = 1112;
        let (reload_tx, _) = broadcast::channel::<()>(16);

        let state = Arc::new(AppState {
            root: root.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
            sandbox: sandbox.map(|p| p.to_path_buf()),
            reload_tx,
        });

        let app = Router::new()
            .route("/", get(dashboard::index))
            .route("/pages", get(pages::list))
            .route("/pages/new", get(pages::new_form).post(pages::create))
            .route("/pages/{*path}", get(pages::edit).post(pages::save))
            .route("/pages/delete/{*path}", post(pages::delete))
            .route("/sections", get(sections::list))
            .route(
                "/sections/{*path}",
                get(sections::edit).post(sections::save),
            )
            .route("/config", get(config::edit).post(config::save))
            .route("/assets", get(assets::list))
            .route("/assets/upload", post(assets::upload))
            .route("/build", post(build::trigger))
            .route("/preview/render", post(build::render_preview))
            .with_state(state);

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                eprintln!("Port {port} is in use, using a random available port...");
                let fallback = SocketAddr::from(([127, 0, 0, 1], 0));
                tokio::net::TcpListener::bind(fallback).await?
            }
            Err(e) => return Err(e.into()),
        };
        let actual_addr = listener.local_addr()?;

        println!("zorto webapp: http://localhost:{}", actual_addr.port());
        let _ = open::that(format!("http://localhost:{}", actual_addr.port()));

        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c().await.ok();
                println!("\nshutting down...");
            })
            .await?;

        Ok(())
    })
}

pub(crate) fn rebuild_site(state: &AppState) -> Result<(), String> {
    match zorto_core::site::Site::load(&state.root, &state.output_dir, true) {
        Ok(mut site) => {
            site.sandbox = state.sandbox.clone();
            site.build().map_err(|e| e.to_string())?;
            let _ = state.reload_tx.send(());
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub(crate) fn escape(s: &str) -> String {
    zorto_core::content::escape_html(s)
}

/// Validate that a user-supplied path, when joined to a base directory, stays
/// within that directory. Returns the canonical path on success, or an error
/// message suitable for display.
pub(crate) fn validate_path(base: &Path, user_path: &str) -> Result<PathBuf, String> {
    let joined = base.join(user_path);

    // Canonicalize base (must exist)
    let canonical_base = base
        .canonicalize()
        .map_err(|e| format!("Base directory does not exist: {e}"))?;

    // For existence-checking operations, canonicalize the joined path.
    // For creation, canonicalize the parent and verify.
    let canonical = if joined.exists() {
        joined
            .canonicalize()
            .map_err(|e| format!("Cannot resolve path: {e}"))?
    } else {
        // File doesn't exist yet (creation). Canonicalize the parent dir.
        let parent = joined.parent().ok_or("Invalid path")?;
        let canonical_parent = parent
            .canonicalize()
            .map_err(|e| format!("Parent directory does not exist: {e}"))?;
        canonical_parent.join(joined.file_name().ok_or("Invalid filename")?)
    };

    if !canonical.starts_with(&canonical_base) {
        return Err("Path traversal detected".to_string());
    }

    Ok(canonical)
}
