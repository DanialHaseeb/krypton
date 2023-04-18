use std::io;

use crate::{Γ, Σ};
use super::key::Key;

pub fn run(key: Key)
{
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    { print!("{}", encrypt(σ, &key)); }
    println!();
  }
}

pub fn encrypt(σ: char, key: &Key) -> char
{
  if !(σ.is_ascii_alphabetic())
  { return σ; }

  let σ = σ.to_ascii_uppercase();
  let mut γ = Γ[&σ];

  γ = (γ + key.shift) % Σ.len();

  Σ[γ]
}
