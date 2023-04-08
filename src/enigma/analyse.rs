use std::io;

use crate::Γ;
use crate::Σ;
use super::key::Key;

pub fn run()
{
  eprintln!("Enigma analysis ran");

  let input = parse();


}

fn parse() -> String
{
  let mut input = String::new();
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    {
      if !(σ.is_ascii_alphabetic())
      { continue; }

      let σ = σ.to_ascii_uppercase();
      input.push(σ);
    }
  }
  input
}
