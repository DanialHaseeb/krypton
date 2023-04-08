use std::io;

use crate::Γ;
use crate::Σ;
use super::key::Key;

pub fn run(mut key: Key)
{
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    { print!("{}", encrypt(σ, &mut key)); }
    println!();
  }
}

pub fn encrypt(σ: char, key: &mut Key) -> char
{
  if !(σ.is_ascii_alphabetic())
  { return σ; }

  let σ = σ.to_ascii_uppercase();
  let mut γ = Γ[&σ];

  γ = key.plugboard[γ];

  key.rotate_rotors();

  for rotor in key.rotors.iter().rev()
  { γ = rotor.map(γ); }

  γ = key.reflector[γ];

  for rotor in key.rotors.iter()
  { γ = rotor.inverse_map(γ); }

  γ = key.plugboard[γ];

  Σ[γ]
}
