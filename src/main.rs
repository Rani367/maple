mod cli;
mod config;
mod output;
mod scan;

fn main() -> anyhow::Result<()> {
    cli::run()
}
