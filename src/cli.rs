use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Programming language extension [py, js, ts, cpp, rs]
    #[arg(short, long)]
    pub extension: String,

    /// Output folder path
    #[arg(short, long)]
    pub folder: String,

    /// Number of files to generate
    #[arg(short = 'n', long, default_value_t = 1)]
    pub count: usize,
}