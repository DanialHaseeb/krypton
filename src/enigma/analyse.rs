use std::io;

use super::key::rotor::Rotor;
use super::key::rotor::kind::Kind::*;
use super::key::reflector::Reflector;
use super::key::plugboard::Plugboard;
use crate::Γ;
use crate::Σ;
use super::key::Key;

pub fn run()
{
  eprintln!("Enigma analysis ran");

  let ciphertext = parse();

  for left in [I, II, III, IV, V].iter()
  {
    for middle in [I, II, III, IV, V].iter()
    {
      if middle == left
      { continue; }

      for right in [I, II, III, IV, V].iter()
      {
        if (right == left) || (right == middle)
        { continue; }

        let rotor1 = Rotor::new(*right, 0, 0);
        let rotor2 = Rotor::new(*middle, 0, 0);
        let rotor3 = Rotor::new(*left, 0, 0);
        let rotors = [rotor1, rotor2, rotor3];
        let reflector = Reflector::A.wiring();
        let plugboard = Plugboard::new().wiring;

        let key = Key{ rotors, reflector, plugboard };

        let combo = [left, middle, right];
        eprintln!("{:?}", combo);

        let plaintext = decrypt(&ciphertext, &key);


      }
    }
  }
}

fn parse() -> String
{
  let mut ciphertext = String::new();
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    {
      if !(σ.is_ascii_alphabetic())
      { continue; }

      let σ = σ.to_ascii_uppercase();
      ciphertext.push(σ);
    }
  }

  ciphertext
}

fn decrypt(ciphertext: &String, key: &Key) -> String
{
  let mut plaintext = String::new();
  for σ in ciphertext.chars()
  {
    if !(σ.is_ascii_alphabetic())
    { continue; }

    let σ = σ.to_ascii_uppercase();
    let mut γ = Γ[&σ];

    γ = key.plugboard[γ];

    for rotor in key.rotors.iter().rev()
    { γ = rotor.map(γ); }

    γ = key.reflector[γ];

    for rotor in key.rotors.iter()
    { γ = rotor.inverse_map(γ); }

    γ = key.plugboard[γ];

    plaintext.push(Σ[γ]);
  }

  plaintext
}
