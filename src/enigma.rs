use std::error::Error;

use crate::config::mode::Mode;

pub mod key;
pub mod encrypt;
pub mod decrypt;
pub mod analyse;

pub fn run<T>(mode: Mode, args: T) -> Result<(), Box<dyn Error>>
where T: Iterator<Item = String>
{
  use key::Key;

  if mode == Mode::Analyse
  { return Ok(analyse::run()) }

  let key = Key::parse(args)?;
  if mode == Mode::Encrypt
  { Ok(encrypt::run(key)) }
  else
  { Ok(decrypt::run(key)) }
}
