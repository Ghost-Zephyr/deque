use std::error::Error;
use std::fs;

use clap::Parser;

pub mod jit;

const BANG: char = '!';

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, value_parser)]
  file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();
  let input = fs::read_to_string(args.file)?;

  match jit::Runner::new(input).run() {
    Ok(ok) => Ok(ok),
    Err(err) => {
      println!("{}", err);
      Ok(())
    },
  }
}
