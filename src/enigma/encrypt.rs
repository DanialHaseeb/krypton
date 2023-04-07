use std::io;

use super::key::Key;

pub fn run(key: Key)
{
  while let Some(Ok(line)) = io::stdin().lines().next()
  {
    println!("{line}");
  }
}
