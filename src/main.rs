mod steg;
use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use steg::{decode, encode};

#[derive(Parser)]
#[command(name = "steg")]
#[command(about = "Encode and decode files inside images using LSB steganography")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        //LSB depth (between 1 and 8)
        #[arg(short, long)]
        depth: u8,

        //input image
        #[arg(short, long)]
        image: PathBuf,

        //input message file
        #[arg(short, long)]
        message: PathBuf,

        //output (encoded) image
        #[arg(short, long)]
        output: PathBuf,
    },

    Decode {
        //LSB depth (between 1 and 8)
        #[arg(short, long)]
        depth: u8,

        //input (encoded) image
        #[arg(short, long)]
        image: PathBuf,

        //output message file
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            depth,
            image,
            message,
            output,
        } => {
            let mut img = image::open(image)?.to_rgba8();
            let message = fs::read(message)?;

            encode(&mut img, &message, depth)?;

            img.save(output)?;

            println!("file successfully encoded");
        }

        Commands::Decode {depth, image, output } => {
            let img = image::open(image)?.to_rgba8();

            let message = decode(&img, depth)?;

            fs::write(output, message)?;

            println!("file successfully decoded");
        }
    }

    Ok(())
}
