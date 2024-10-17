use clap::{Parser, Subcommand};
mod convert;

#[derive(Parser, Debug)]
#[clap(author = "Adi", version = "0.0.1", about = "Shitty image converter")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(
        about = "img <SRC> <DEST> [--quality <QUALITY>]",
        long_about = "Convert an image from png to jpg or viceversa."
    )]
    Img {
        #[clap(value_name = "SRC", required = true)]
        src: String,

        #[clap(value_name = "DEST", required = true)]
        dest: String,

        /// Ajusta la calidad de la imagen de salida (1-100)
        #[clap(short, long, default_value = "80", value_name = "QUALITY")]
        quality: u8,
    },
}

fn main() {
    let cli = Cli::parse(); // Procesa los argumentos de línea de comandos

    match &cli.command {
        Commands::Img { src, dest, quality } => {
            println!("Convirtiendo {} a {} con calidad {}%", src, dest, quality);
            convert::convert_img(src, dest);
        }
    }
}
