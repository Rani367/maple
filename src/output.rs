use crate::scan::ScanReport;

pub fn print_report(report: &ScanReport) {
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
