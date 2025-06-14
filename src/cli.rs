use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(long)]
        exchange: String,
        #[arg(long)]
        symbol: String,
        #[arg(long, default_value = "L1")]
        level: String,
    },
    Export {
        #[arg(long)]
        exchange: String,
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        output: String,
    },
}