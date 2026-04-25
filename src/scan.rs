use crate::config::ScanConfig;
use anyhow::{Context, Result, bail};
use ignore::WalkBuilder;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub root: PathBuf,
    pub files: usize,
    pub bytes: u64,
    pub languages: BTreeMap<String, LanguageStats>,
}

#[derive(Debug, Default, Serialize)]
pub struct LanguageStats {
    pub files: usize,
    pub bytes: u64,
}

pub fn scan_local_path(root: &Path, config: &ScanConfig) -> Result<ScanReport> {
    let root = root
        .canonicalize()
        .with_context(|| format!("failed to resolve {}", root.display()))?;

    if !root.is_dir() {
        bail!("{} is not a directory", root.display());
    }

    let mut report = ScanReport {
        root,
        files: 0,
        bytes: 0,
        languages: BTreeMap::new(),
    };

    let skipped_directory_names = config.skipped_directory_names.clone();
    let walker = WalkBuilder::new(&report.root)
        .hidden(!config.include_hidden)
        .git_ignore(config.respect_git_ignore)
        .git_global(config.respect_git_global)
        .git_exclude(config.respect_git_exclude)
        .filter_entry(move |entry| should_descend(entry.path(), &skipped_directory_names))
        .build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let metadata = path
            .metadata()
            .with_context(|| format!("failed to read metadata for {}", path.display()))?;
        let bytes = metadata.len();
        let language = language_for(path);
        let stats = report.languages.entry(language.to_string()).or_default();

        stats.files += 1;
        stats.bytes += bytes;
        report.files += 1;
        report.bytes += bytes;
    }

    Ok(report)
}

fn language_for(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("rs") => "Rust",
        Some("ts") | Some("tsx") => "TypeScript",
        Some("js") | Some("jsx") | Some("mjs") | Some("cjs") => "JavaScript",
        Some("py") => "Python",
        Some("go") => "Go",
        Some("java") => "Java",
        Some("kt") | Some("kts") => "Kotlin",
        Some("swift") => "Swift",
        Some("c") | Some("h") => "C",
        Some("cc") | Some("cpp") | Some("cxx") | Some("hpp") | Some("hh") => "C++",
        Some("cs") => "C#",
        Some("rb") => "Ruby",
        Some("php") => "PHP",
        Some("lua") => "Lua",
        Some("ex") | Some("exs") => "Elixir",
        Some("scala") | Some("sbt") => "Scala",
        Some("html") => "HTML",
        Some("css") | Some("scss") | Some("sass") => "CSS",
        Some("md") | Some("mdx") => "Markdown",
        Some("json") => "JSON",
        Some("toml") => "TOML",
        Some("yaml") | Some("yml") => "YAML",
        Some("sh") | Some("bash") | Some("zsh") | Some("fish") => "Shell",
        _ => "Other",
    }
}

fn should_descend(path: &Path, skipped_directory_names: &[String]) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return true;
    };

    !skipped_directory_names
        .iter()
        .any(|skipped| skipped == name)
}
