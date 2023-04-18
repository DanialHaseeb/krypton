use std::io;

use super::{key::Key, encrypt::encrypt};

pub mod darwin;
pub mod fitness;

pub fn run()
{
  let ciphertext = parse();
  let mut key = Key::default();
  eprintln!("Ciphertext:\n{}\n", ciphertext);

  darwin::select_rotors(&ciphertext, &mut key);
  eprintln!("Best rotor config:\n{key:?}");
  darwin::select_ring_settings(&ciphertext, &mut key);
  eprintln!("Best ring settings:\n{key:?}");
  darwin::select_plugboard(&ciphertext, &mut key);

  let plaintext = decrypt(&ciphertext, &mut key);
  println!("\nPlaintext:\n{}", plaintext);
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
