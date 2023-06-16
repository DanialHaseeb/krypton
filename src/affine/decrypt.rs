use std::io;

use crate::{Γ, Σ};
use super::key::Key;

//Run
pub fn run(key: Key)
{
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    { print!("{}", decrypt(σ, &key)); }
    println!();
  }
}

//Decrypt
pub fn decrypt(σ: char, key: &Key) -> char
{
  let factor = inverse(key.factor);
  let shift = key.shift;

  if !(σ.is_ascii_alphabetic())
  { return σ; }

  let σ = σ.to_ascii_uppercase();
  let mut γ = Γ[&σ];

  γ = (26 + γ - shift) % Σ.len();
  γ = γ * factor % Σ.len();

  Σ[γ]
}

//Inverse
fn inverse(factor: usize) -> usize
{
  for i in 1..Σ.len()
  {
    if (factor * i) % Σ.len() == 1
    { return i; }
  }
  panic!("No inverse found for {}", factor);
}
