
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ProgramArgs {
    /// File to convert
    #[arg(short, long)]
    pub file: String,

    // Output file
    #[arg(short, long)]
    pub output: String,

    /// Bit depth to use for the conversion
    #[arg(short, long, default_value_t = 24)]
    pub depth: u8,
}
