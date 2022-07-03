use clap::{Parser, Subcommand};
use console::style;
use t_rex::{fix::process_fix, insert::process_insert};

#[derive(Parser, Debug)]
#[clap(version, about, about)]
struct Cli {
    /// Path to the new file
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Insert a file, incrementing the index of all the following files
    Insert {
        /// Path to the new page
        page_path: String,
    },
    Fix {
        /// Path to the directory to fix
        directory_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Insert { page_path } => process_insert(page_path),
        Commands::Fix { directory_path } => process_fix(directory_path),
    };

    // result.unwrap();
    match result {
        Ok(()) => (),
        Err(err) => {
            println!("{} {}", style("Error:").red(), err);
            std::process::exit(1);
        }
    }
}
