use std::io;

use crate::{Γ, Σ};
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

pub fn encrypt(σ: char, enigma: &mut Key) -> char
{
  if !(σ.is_ascii_alphabetic())
  { return σ; }

  let σ = σ.to_ascii_uppercase();
  let mut γ = Γ[&σ];

  γ = enigma.plugboard.wiring[γ];

  enigma.rotate_rotors();

  for rotor in enigma.rotors.iter().rev()
  { γ = rotor.map(γ); }

  γ = enigma.reflector.wiring[γ];

  for rotor in enigma.rotors.iter()
  { γ = rotor.inverse_map(γ); }

  γ = enigma.plugboard.wiring[γ];

  Σ[γ]
}
