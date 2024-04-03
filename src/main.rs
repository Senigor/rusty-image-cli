use rusty_image_cli::commands;

use clap::{Parser, Subcommand, ValueHint};

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
    /// Blur image
    Blur {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,

        /// Depth of the blur
        #[arg(short, long, value_name = "DEPTH")]
        depth: f32,
    },

    /// Create fractal image
    Fractal {
        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,
    },

    /// Brighten image
    Brighten {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,

        /// Value to brighten the image, positive numbers brighten the image, negative darken it
        #[arg(short, long, value_name = "VALUE")]
        value: i32,
    },

    /// Crop image
    Crop {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,

        /// X axis value
        #[arg(short)]
        x: u32,

        /// Y axis value
        #[arg(short)]
        y: u32,

        /// Width value
        #[arg(short, long)]
        width: u32,

        /// Height value
        #[arg(short, long)]
        height: u32,
    },

    /// Rotate image
    Rotate {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,

        /// Rotation degrees
        #[arg(short, long, value_name = "VALUE", value_parser=["90", "180", "270"])]
        value: String,
    },

    /// Invert image
    Invert {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
        outfile: String,
    },
    /// Grayscale image
    Grayscale {
        /// Path to the input file
        #[arg(short, long, value_name = "INFILE", value_hint = ValueHint::FilePath)]
        infile: String,

        /// Path to the output file
        #[arg(short, long, value_name = "OUTFILE", value_hint = ValueHint::FilePath)]
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
            commands::blur(infile, outfile, depth);
        }
        Some(Commands::Fractal { outfile }) => {
            println!("Executing fractal command with outfile: {}", outfile);
            commands::fractal(outfile);
        }
        Some(Commands::Brighten {
            infile,
            outfile,
            value,
        }) => {
            println!(
                "Executing brighten command with infile: {}, outfile: {}, value: {}",
                infile, outfile, value
            );
            commands::brighten(infile, outfile, value);
        }
        Some(Commands::Crop {
            infile,
            outfile,
            x,
            y,
            width,
            height,
        }) => {
            println!(
                "Executing crop command with infile: {}, outfile: {}, x: {}, y: {}, width: {}, height: {}",
                infile, outfile, x, y, width, height
            );
            commands::crop(infile, outfile, x, y, width, height);
        }
        Some(Commands::Rotate {
            infile,
            outfile,
            value,
        }) => {
            println!(
                "Executing rotate command with infile: {}, outfile: {}, value: {}",
                infile, outfile, value
            );
            commands::rotate(infile, outfile, &value.parse::<u32>().unwrap());
        }
        Some(Commands::Invert { infile, outfile }) => {
            println!(
                "Executing invert command with infile: {}, outfile: {}",
                infile, outfile
            );
            commands::invert(infile, outfile);
        }
        Some(Commands::Grayscale { infile, outfile }) => {
            println!(
                "Executing grayscale command with infile: {}, outfile: {}",
                infile, outfile
            );
            commands::grayscale(infile, outfile);
        }
        None => {
            println!("Provide correct subcommand. If you are lost, do not hesitate to use --help!")
        }
    }
}
