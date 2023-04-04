use std::io;

use super::key::Key;

pub fn run(key: Key) -> String
{
  let mut output = String::new();
  for line in io::stdin().lines().enumerate()
  {
    let line = line.1.unwrap();
    let mut result = String::new();

    for c in line.chars()
    {
      let c = c as u8;
      let c = c.wrapping_add(key.shift as u8);
      let c = c as char;
      result.push(c);
    }

    output.push_str(&result);
  }

  output
}
