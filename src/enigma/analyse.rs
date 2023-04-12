use std::io;

use super::key::Key;
use super::encrypt::encrypt;

pub mod darwin;

pub fn run()
{
  eprintln!("Enigma analysis ran");

  let ciphertext = parse();
  let mut key = Key::default();

  darwin::select_rotors(&ciphertext, &mut key);
  darwin::select_ring_settings(&ciphertext, &mut key);
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

fn score(plaintext: &String) -> f64
{
  let mut score = 0.0;
  for σ in plaintext.chars()
  {
    if σ.is_ascii_alphabetic()
    { score += 1.0; }
  }

  score
}
