//! Template linting — opinionated checks for Zorto templates.
//!
//! Inspired by clippy and rustfmt: warns about patterns that make themes
//! harder to maintain, like hardcoded user-facing strings in templates.
//! Strings should live in `config.toml` (`[extra]`) or content markdown
//! files, not in HTML templates.

use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;

/// A lint warning produced by the template linter.
#[derive(Debug)]
pub struct LintWarning {
    /// Relative path to the template file.
    pub file: String,
    /// 1-based line number.
    pub line: usize,
    /// The offending text snippet.
    pub text: String,
    /// Human-readable message.
    pub message: String,
}

impl std::fmt::Display for LintWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "warning[hardcoded-string]: {}:{}: \"{}\" -- {}",
            self.file, self.line, self.text, self.message
        )
    }
}

/// Regex to match Tera expressions and tags: {{ ... }}, {% ... %}, {# ... #}
static TERA_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)\{\{.*?\}\}|\{%.*?%\}|\{#.*?#\}").unwrap());

/// Regex to match HTML tags (including self-closing)
static HTML_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)<[^>]+>").unwrap());

/// Regex to detect user-facing text: 2+ word characters in a row
static TEXT_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[A-Za-z][A-Za-z]{1,}\b").unwrap());

/// Common structural strings that are acceptable in templates.
const ALLOWLIST: &[&str] = &[
    // HTML/structural
    "DOCTYPE", "html", "head", "body", "main", "nav", "footer", "div", "span", "ul", "li", "button",
    "label", "input", "meta", "link", "script", "style", "svg", "path", "circle", "line", "rect",
    "polyline", "xmlns", "viewBox", "fill", "stroke", "width", "height", "cx", "cy",
    // Accessibility
    "aria", // CSS class names
    "class", "id", "href", "src", "alt", "rel", "type", "name", "content", "charset", "viewport",
    "robots", "noodp", // Common template text
    "if", "else", "endif", "for", "endfor", "block", "endblock", "extends", "macro", "import",
    "set", "true", "false", // HTML entities / symbols
    "larr", "rarr", "copy", "amp", "nbsp", "lt", "gt", "xFE",
];

/// Lint all HTML template files in the given directory.
///
/// Returns warnings for lines that appear to contain hardcoded user-facing
/// strings. Skips files in `shortcodes/` subdirectory and content inside
/// `<script>` and `<style>` blocks.
pub fn lint_templates(templates_dir: &Path) -> Vec<LintWarning> {
    let mut warnings = Vec::new();

    if !templates_dir.exists() {
        return warnings;
    }

    for entry in walkdir::WalkDir::new(templates_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip non-HTML files
        if path.extension().and_then(|e| e.to_str()) != Some("html") {
            continue;
        }

        // Skip shortcodes directory
        let rel = path
            .strip_prefix(templates_dir)
            .unwrap_or(path)
            .to_string_lossy();
        if rel.starts_with("shortcodes") {
            continue;
        }

        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };

        lint_template_content(&rel, &content, &mut warnings);
    }

    warnings
}

/// Lint a single template's content.
fn lint_template_content(file: &str, content: &str, warnings: &mut Vec<LintWarning>) {
    // Remove <script>...</script> and <style>...</style> blocks
    let no_script = remove_blocks(content, "script");
    let cleaned = remove_blocks(&no_script, "style");

    for (line_idx, line) in cleaned.lines().enumerate() {
        let line_num = line_idx + 1;

        // Strip Tera expressions/tags from the line
        let no_tera = TERA_RE.replace_all(line, " ");

        // Strip HTML tags
        let no_html = HTML_TAG_RE.replace_all(&no_tera, " ");

        // Look for remaining text that looks like user-facing content
        for m in TEXT_RE.find_iter(&no_html) {
            let word = m.as_str();

            // Skip allowlisted words
            if ALLOWLIST.iter().any(|&a| word.eq_ignore_ascii_case(a)) {
                continue;
            }

            // Skip single short words (likely CSS classes or HTML attributes)
            if word.len() <= 3 {
                continue;
            }

            warnings.push(LintWarning {
                file: file.to_string(),
                line: line_num,
                text: word.to_string(),
                message: "consider moving to config.extra or content".to_string(),
            });
        }
    }
}

/// Remove all `<tag>...</tag>` blocks from content.
fn remove_blocks(content: &str, tag: &str) -> String {
    let re = Regex::new(&format!(r"(?si)<{tag}[\s>].*?</{tag}>")).unwrap();
    re.replace_all(content, " ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_lint_detects_hardcoded_string() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("templates");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test.html"), "<h1>Welcome to my site</h1>").unwrap();
        let warnings = lint_templates(&dir);
        assert!(
            warnings.iter().any(|w| w.text == "Welcome"),
            "Should flag 'Welcome': {warnings:?}"
        );
    }

    #[test]
    fn test_lint_allows_tera_expressions() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("templates");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("test.html"), "<h1>{{ config.title }}</h1>").unwrap();
        let warnings = lint_templates(&dir);
        assert!(
            warnings.is_empty(),
            "Should not flag Tera expressions: {warnings:?}"
        );
    }

    #[test]
    fn test_lint_skips_script_blocks() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("templates");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("test.html"),
            "<script>var message = 'Hello World';</script>",
        )
        .unwrap();
        let warnings = lint_templates(&dir);
        assert!(
            warnings.is_empty(),
            "Should not flag content in script tags: {warnings:?}"
        );
    }

    #[test]
    fn test_lint_skips_style_blocks() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("templates");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("test.html"),
            "<style>.greeting { color: red; }</style>",
        )
        .unwrap();
        let warnings = lint_templates(&dir);
        assert!(
            warnings.is_empty(),
            "Should not flag content in style tags: {warnings:?}"
        );
    }

    #[test]
    fn test_lint_skips_shortcodes_dir() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("templates");
        let sc_dir = dir.join("shortcodes");
        std::fs::create_dir_all(&sc_dir).unwrap();
        std::fs::write(
            sc_dir.join("note.html"),
            "<div>Warning: important notice</div>",
        )
        .unwrap();
        let warnings = lint_templates(&dir);
        assert!(
            warnings.is_empty(),
            "Should not lint shortcode templates: {warnings:?}"
        );
    }

    #[test]
    fn test_lint_nonexistent_dir() {
        let warnings = lint_templates(Path::new("/nonexistent"));
        assert!(warnings.is_empty());
    }
}
