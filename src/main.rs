use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use ignore::WalkBuilder;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { target, json } => {
            let report = scan_local_path(&target)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                print_report(&report);
            }
        }
        Command::Serve { target } => {
            println!(
                "Local web explorer is not implemented yet. Target: {}",
                target.display()
            );
        }
        Command::Pack { target } => {
            println!(
                "Context pack export is not implemented yet. Target: {}",
                target.display()
            );
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "maple")]
#[command(about = "Map, explain, and package any repository.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan a local repository and print a structural summary.
    Scan {
        /// Local path to scan.
        #[arg(default_value = ".")]
        target: PathBuf,

        /// Print machine-readable JSON.
        #[arg(long)]
        json: bool,
    },

    /// Start the local web explorer.
    Serve {
        /// Local path to explore.
        #[arg(default_value = ".")]
        target: PathBuf,
    },

    /// Export an AI-ready context pack.
    Pack {
        /// Local path to package.
        #[arg(default_value = ".")]
        target: PathBuf,
    },
}

#[derive(Debug, Serialize)]
struct ScanReport {
    root: PathBuf,
    files: usize,
    bytes: u64,
    languages: BTreeMap<String, LanguageStats>,
}

#[derive(Debug, Default, Serialize)]
struct LanguageStats {
    files: usize,
    bytes: u64,
}

fn scan_local_path(root: &Path) -> Result<ScanReport> {
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

    let walker = WalkBuilder::new(&report.root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| should_descend(entry.path()))
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

fn should_descend(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return true;
    };

    !matches!(
        name,
        ".git"
            | ".hg"
            | ".svn"
            | ".next"
            | ".nuxt"
            | ".turbo"
            | ".vercel"
            | "build"
            | "coverage"
            | "dist"
            | "node_modules"
            | "target"
            | "vendor"
    )
}

fn print_report(report: &ScanReport) {
    println!("Maple scan");
    println!();
    println!("Root:  {}", report.root.display());
    println!("Files: {}", report.files);
    println!("Size:  {}", format_bytes(report.bytes));
    println!();
    println!("Languages:");

    for (language, stats) in &report.languages {
        println!(
            "  {language:<12} {:>5} files  {:>10}",
            stats.files,
            format_bytes(stats.bytes)
        );
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes = bytes as f64;

    if bytes >= GB {
        format!("{:.1} GB", bytes / GB)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes / MB)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes / KB)
    } else {
        format!("{bytes:.0} B")
    }
}
