use std::io;

use super::key::rotor::kind::Kind::*;
// use super::key::rotor::Rotor;
// use super::key::reflector::Kind;
// use super::key::plugboard::Plugboard;
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

        let mut key =

        let _plaintext = decrypt(&ciphertext, &key);
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

fn decrypt(ciphertext: &String, key: &mut Key) -> String
{
  let mut plaintext = String::new();
  for σ in ciphertext.chars()
  {
    if !(σ.is_ascii_alphabetic())
    { continue; }

    plaintext.push(encrypt(σ, key));
  }

  plaintext
}
