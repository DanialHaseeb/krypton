use crate::Σ;
use super::{decrypt, super::key::Key, dictionary};

pub fn assassinate(caesar: &String, key: &mut Key)
{
  let mut best = usize::MIN;
  for shift in 0..Σ.len()
  {
    let current = Key{ shift };
    let plaintext = decrypt(caesar, &current);
    let score = score(&plaintext);

    if score > best
    {
      best = score;
      key.shift = shift;
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
