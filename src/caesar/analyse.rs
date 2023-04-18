use std::io;

use super::{key::Key, decrypt};

pub mod brutus;
pub mod dictionary;

pub fn run()
{
  let caesar = parse();
  eprintln!("Ciphertext:\n{}", caesar)
  let mut key = Key::default();

  brutus::assassinate(&caesar, &mut key);

  let plaintext = decrypt(&caesar, &mut key);
  println!("\nPlaintext:\n{}", plaintext);
}

fn parse() -> String
{
  let mut ciphertext = String::new();
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    { ciphertext.push(σ.to_ascii_uppercase()); }
  }
  ciphertext
}

fn decrypt(ciphertext: &String, key: &Key) -> String
{
  let mut plaintext = String::new();
  for σ in ciphertext.chars()
  {
    let mut σ = σ;

    if σ.is_ascii_alphabetic()
    { σ = decrypt::decrypt(σ, key); }

    plaintext.push(σ);
  }

  plaintext
}
