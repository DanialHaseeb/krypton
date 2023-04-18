use crate::Σ;
use super::{*, super::key::{Key, rotor::{Rotor, kind::Kind::*}}};
use super::fitness;

pub fn select_rotors(ciphertext: &String, key: &mut Key)
{
  eprintln!("Selecting rotor configuration:");
  let mut enigma = key.clone();
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


        enigma.rotors[0] = Rotor::new(*left, 0, 0);
        enigma.rotors[1] = Rotor::new(*middle, 0, 0);
        enigma.rotors[2] = Rotor::new(*right, 0, 0);

        let score = select_positions(ciphertext, &mut enigma);

        eprint!("{enigma:?}: {score} ");

        if score > best
        {
          eprint!("UPDATED");
          best = score;
          key.rotors = enigma.rotors;
        }

        eprintln!();
      }
    }
  }
  eprintln!();
}

fn select_positions(ciphertext: &String, key: &mut Key) -> f64
{
  let mut best = f64::NEG_INFINITY;
  let mut enigma = key.clone();

  for left in 0..26
  {
    for middle in 0..26
    {
      for right in 0..26
      {
        enigma.rotors[0].position = left;
        enigma.rotors[1].position = middle;
        enigma.rotors[2].position = right;

        let plaintext = decrypt(ciphertext, &mut enigma);
        let score = fitness::score(&plaintext);

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
  eprintln!("Selecting ring settings:");
  eprintln!("Right Rotor:");
  let mut enigma = key.clone();
  let mut best = f64::NEG_INFINITY;
  for ring_setting in 0..26
  {
    let current_position = enigma.rotors[2].position;
    let new_position = (current_position + ring_setting) % 26;
    enigma.rotors[2].position = new_position;
    enigma.rotors[2].ring_setting = ring_setting;

    let plaintext = decrypt(ciphertext, &mut enigma);
    let score = fitness::score(&plaintext);
    eprint!("{}: {score} ", Σ[ring_setting]);

    if score > best
    {
      eprint!("UPDATED");
      best = score;
      key.rotors[2].ring_setting = ring_setting;
      key.rotors[2].position = new_position;
    }

    eprintln!();
  }

  eprintln!("Middle Rotor:");
  let mut enigma = key.clone();
  let mut best = f64::NEG_INFINITY;
  for ring_setting in 0..26
  {
    let current_position = enigma.rotors[1].position;
    let new_position = (current_position + ring_setting) % 26;
    enigma.rotors[1].position = new_position;
    enigma.rotors[1].ring_setting = ring_setting;

    let plaintext = decrypt(ciphertext, &mut enigma);
    let score = fitness::score(&plaintext);
    eprint!("{}: {score} ", Σ[ring_setting]);

    if score > best
    {
      eprint!("UPDATED");
      best = score;
      key.rotors[1].ring_setting = ring_setting;
      key.rotors[1].position = new_position;
    }

    eprintln!();
  }
}

fn select_plug(ciphertext: &String, key: &mut Key)
{
  let mut best_score = f64::NEG_INFINITY;
  let mut best_a = 0;
  let mut best_b = 0;

  let mut enigma = key.clone();
  for a in 0..25
  {
    for b in 0..25
    {
      if b < a
      { continue; }

      if key.plugboard.has_plugged(a) || key.plugboard.has_plugged(b)
      { continue; }

      enigma.plugboard.connect(a, b);
      let plaintext = decrypt(ciphertext, &mut enigma);
      let score = fitness::score(&plaintext);
      enigma.plugboard.disconnect(a, b);

      if score > best_score
      {
        best_score = score;
        best_a = a;
        best_b = b;
      }
    }
  }

  key.plugboard.connect(best_a, best_b);
}

pub fn select_plugboard(ciphertext: &String, key: &mut Key)
{
  let mut enigma = key.clone();
  for i in 0..13
  { eprintln!("selecting plug {i}");select_plug(ciphertext, &mut enigma); }
}
