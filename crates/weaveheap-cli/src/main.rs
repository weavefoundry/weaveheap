use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "weaveheap")]
#[command(about = "WeaveHeap CLI", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check tooling and environment
    Doctor,
    /// Print placeholder counters
    Stats,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Doctor => cmd_doctor(),
        Commands::Stats => cmd_stats(),
    }
}

fn cmd_doctor() {
    println!("weaveheap doctor: OK (scaffold)");
    println!(
        "version {}.{}.{}",
        weaveheap_capi::weaveheap_version_major(),
        weaveheap_capi::weaveheap_version_minor(),
        weaveheap_capi::weaveheap_version_patch()
    );
}

fn cmd_stats() {
    let counters = weaveheap_core::Counters::default();
    println!("allocations: {}", counters.allocations);
    println!("copies: {}", counters.copies);
}
