//! Built-in site templates for `zorto init`.

use std::path::Path;

/// A file to be written during site initialization.
struct TemplateFile {
    /// Relative path from the site root.
    path: &'static str,
    /// File contents.
    content: &'static str,
    /// Whether the file should be executable (bin/ scripts).
    executable: bool,
}

// ── Default (blog) template ──────────────────────────────────────────────

const DEFAULT_FILES: &[TemplateFile] = &[
    TemplateFile {
        path: "config.toml",
        content: include_str!("../templates/default/config.toml"),
        executable: false,
    },
    TemplateFile {
        path: "content/_index.md",
        content: include_str!("../templates/default/content/_index.md"),
        executable: false,
    },
    TemplateFile {
        path: "content/posts/_index.md",
        content: include_str!("../templates/default/content/posts/_index.md"),
        executable: false,
    },
    TemplateFile {
        path: "content/posts/hello.md",
        content: include_str!("../templates/default/content/posts/hello.md"),
        executable: false,
    },
    TemplateFile {
        path: "templates/base.html",
        content: include_str!("../templates/default/templates/base.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/index.html",
        content: include_str!("../templates/default/templates/index.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/section.html",
        content: include_str!("../templates/default/templates/section.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/page.html",
        content: include_str!("../templates/default/templates/page.html"),
        executable: false,
    },
];

// ── Business template ────────────────────────────────────────────────────

const BUSINESS_FILES: &[TemplateFile] = &[
    TemplateFile {
        path: "config.toml",
        content: include_str!("../templates/business/config.toml"),
        executable: false,
    },
    TemplateFile {
        path: "content/_index.md",
        content: include_str!("../templates/business/content/_index.md"),
        executable: false,
    },
    TemplateFile {
        path: "templates/base.html",
        content: include_str!("../templates/business/templates/base.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/index.html",
        content: include_str!("../templates/business/templates/index.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/page.html",
        content: include_str!("../templates/business/templates/page.html"),
        executable: false,
    },
    TemplateFile {
        path: "templates/404.html",
        content: include_str!("../templates/business/templates/404.html"),
        executable: false,
    },
    TemplateFile {
        path: "sass/style.scss",
        content: include_str!("../templates/business/sass/style.scss"),
        executable: false,
    },
    TemplateFile {
        path: "bin/build",
        content: include_str!("../templates/business/bin/build"),
        executable: true,
    },
    TemplateFile {
        path: "bin/preview",
        content: include_str!("../templates/business/bin/preview"),
        executable: true,
    },
];

/// Available template names.
pub const TEMPLATE_NAMES: &[&str] = &["default", "business"];

/// Write all files for the given template into `target`.
pub fn write_template(target: &Path, template: &str) -> anyhow::Result<()> {
    let files = match template {
        "default" => DEFAULT_FILES,
        "business" => BUSINESS_FILES,
        _ => anyhow::bail!(
            "unknown template \"{template}\". Available templates: {}",
            TEMPLATE_NAMES.join(", ")
        ),
    };

    for file in files {
        let dest = target.join(file.path);
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&dest, file.content)?;

        #[cfg(unix)]
        if file.executable {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
        }
    }

    // Ensure static/ directory exists even if no static files are in the template.
    std::fs::create_dir_all(target.join("static"))?;

    Ok(())
}
