use std::error::Error;

use crate::config::mode::Mode;

pub mod key;
pub mod encrypt;
pub mod decrypt;
pub mod analyse;

use key::Key;

pub fn run<T>(mode: Mode, args: T) -> Result<(), Box<dyn Error>>
where T: Iterator<Item = String>
{
  if mode == Mode::Analyse
  { return Ok(analyse::run()); }

  let key = Key::parse(args)?;
  match mode
  {
    Mode::Encrypt => Ok(encrypt::run(key)),
    Mode::Decrypt => Ok(decrypt::run(key)),
    _ => unreachable!()
  }
}
