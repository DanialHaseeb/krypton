use std::io;

use super::key::rotor::Rotor;
use super::key::rotor::kind::Kind::*;
use super::key::reflector::Reflector;
use super::key::reflector::Kind::*;
use super::key::plugboard::Plugboard;
use super::key::Key;
use super::encrypt::encrypt;
// use crate::Γ;
// use crate::Σ;

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

        let key = Key
        {
          rotors:
          [
            Rotor::new(*left, 0, 0),
            Rotor::new(*middle, 0, 0),
            Rotor::new(*right, 0, 0)
          ],
          reflector: Reflector::new(A),
          plugboard: Plugboard::default()
        };

        let _plaintext = decrypt(&ciphertext, key.clone());
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

fn decrypt(ciphertext: &String, mut key: Key) -> String
{
  let mut plaintext = String::new();
  for σ in ciphertext.chars()
  {
    if !(σ.is_ascii_alphabetic())
    { continue; }

    plaintext.push(encrypt(σ, &mut key));
  }

  plaintext
}
