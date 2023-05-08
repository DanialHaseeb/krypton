use crate::Σ;
use super::{decrypt, super::key::Key, dictionary};

pub fn assassinate(caesar: &String, key: &mut Key)
{
  let mut best = usize::MIN;
  for factor in 1..Σ.len()
  {
    if gcd(factor, Σ.len()) != 1
    { continue; }
    for shift in 0..Σ.len()
    {
      let current = Key{ factor, shift };
      let plaintext = decrypt(caesar, &current);
      let score = score(&plaintext);

      if score > best
      {
        best = score;
        key.shift = shift;
        key.factor = factor;
      }
    }
  }
}

fn score(text: &String) -> usize
{
  let mut score = usize::MIN;

  for word in text.split_whitespace()
  {
    if dictionary::contains(word)
    { score += 1; }
  }

  score
}

fn gcd(a: usize, b: usize) -> usize
{
  if b == 0
  { a }
  else
  { gcd(b, a % b) }
}
