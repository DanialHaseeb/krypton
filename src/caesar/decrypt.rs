use std::io;

use super::{key::Key, encrypt::encrypt};

pub fn run(key: Key)
{
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    for σ in line.chars()
    { print!("{}", decrypt(σ, &key)); }
    println!();
  }
}

pub fn decrypt(σ: char, key: &Key) -> char
{
  let shift = 26 - key.shift;
  let key = Key{ shift };
  encrypt(σ, &key)
}
