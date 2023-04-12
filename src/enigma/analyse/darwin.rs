use super::super::key::
{
  Key,
  rotor::{Rotor, kind::Kind::*},
};
use super::*;

pub fn select_rotors(ciphertext: &String, key: &mut Key)
{
  let mut best = f64::NEG_INFINITY;
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

        let mut enigma = key.clone();
        enigma.rotors[0] = Rotor::new(*left, 0, 0);
        enigma.rotors[1] = Rotor::new(*middle, 0, 0);
        enigma.rotors[2] = Rotor::new(*right, 0, 0);

        let score = select_positions(ciphertext, &mut enigma);

        if score > best
        {
          best = score;
          key.rotors[0].kind = *left;
          key.rotors[1].kind = *middle;
          key.rotors[2].kind = *right;
        }
      }
    }
  }
}

fn select_positions(ciphertext: &String, key: &mut Key) -> f64
{
  let mut best = f64::NEG_INFINITY;

  for left in 0..26
  {
    for middle in 0..26
    {
      for right in 0..26
      {
        let mut enigma = key.clone();
        enigma.rotors[0].position = left;
        enigma.rotors[1].position = middle;
        enigma.rotors[2].position = right;

        let plaintext = decrypt(ciphertext, &mut enigma);
        let score = score(&plaintext);

        if score > best
        {
          best = score;
          key.rotors[0].position = left;
          key.rotors[1].position = middle;
          key.rotors[2].position = right;
        }
      }
    }
  }

  best
}

pub fn select_ring_settings(ciphertext: &String, key: &mut Key)
{
  unimplemented!();
}
