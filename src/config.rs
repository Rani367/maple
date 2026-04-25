use anyhow::Result;

const DEFAULT_SKIPPED_DIRECTORY_NAMES: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    ".next",
    ".nuxt",
    ".turbo",
    ".vercel",
    "build",
    "coverage",
    "dist",
    "node_modules",
    "target",
    "vendor",
];

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub scan: ScanConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub include_hidden: bool,
    pub respect_git_ignore: bool,
    pub respect_git_global: bool,
    pub respect_git_exclude: bool,
    pub skipped_directory_names: Vec<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            include_hidden: true,
            respect_git_ignore: true,
            respect_git_global: true,
            respect_git_exclude: true,
            skipped_directory_names: DEFAULT_SKIPPED_DIRECTORY_NAMES
                .iter()
                .map(|name| (*name).to_owned())
                .collect(),
        }
    }
}
