use std::fs::{File, OpenOptions};
use std::io::Write;

use clap::Parser;
use rust_embed::Embed;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Error(String);
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error(e.to_string())
    }
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Embed)]
#[folder = "llama"]
struct Llama;

#[derive(Embed)]
#[folder = "gemini"]
struct Gemini;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 1337)]
    pub count: u64,
}
fn exaust_rust_embed_llama(target: &mut File) -> Result<()> {
    for filename in Llama::iter() {
        let file = Llama::get(&filename).unwrap();
        target.write_all(&file.data)?;
        target.flush()?;
    }
    Ok(())
}
fn exaust_rust_embed_gemini(target: &mut File) -> Result<()> {
    for filename in Gemini::iter() {
        let file = Gemini::get(&filename).unwrap();
        target.write_all(&file.data)?;
        target.flush()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut target = OpenOptions::new().append(true).open("/dev/random")?;
    for _ in 0..args.count {
        exaust_rust_embed_llama(&mut target)?;
        exaust_rust_embed_gemini(&mut target)?;
    }
    Ok(())
}
