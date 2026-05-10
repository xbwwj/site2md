use clap::{Parser, Subcommand};
use site2md_secrss::{Secrss, fetch_index_html};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Secrss(secrss) => match secrss.command {
            site2md_secrss::Commands::RawIndexHtml => {
                let out = fetch_index_html().await;
                println!("{}", out);
            }
            site2md_secrss::Commands::RawIndexJson => unimplemented!(),
        },
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Secrss(Secrss),
}
