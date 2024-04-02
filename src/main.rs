use rusty_image_cli::commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Igor Dudchenko <dudchenko.igor15@gmail.com>",
    about = "CLI Image processing"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Blur description
    Blur {
        /// Infile description
        #[arg(short, long, value_name = "INFILE")]
        infile: String,

        /// Outfile description
        #[arg(short, long, value_name = "OUTFILE")]
        outfile: String,

        /// Blur depth
        #[arg(short, long, value_name = "DEPTH")]
        depth: f32,
    },

    /// Fractal description
    Fractal {
        /// Outfile description
        #[arg(short, long, value_name = "OUTFILE")]
        outfile: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Blur {
            infile,
            outfile,
            depth,
        }) => {
            println!(
                "Executing blur command with infile: {}, outfile: {}, depth: {}",
                infile, outfile, depth
            );
            commands::blur(infile.to_string(), outfile.to_string());
        }
        Some(Commands::Fractal { outfile }) => {
            println!("Executing fractal command with outfile: {}", outfile);
            commands::fractal(outfile.to_string());
        }
        None => println!("Provide subcommand."),
    }
}
