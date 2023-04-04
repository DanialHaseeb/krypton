use std::error::Error;

use crate::config::mode::Mode;

pub mod key;
pub mod encrypt;
pub mod decrypt;
pub mod analyse;

use key::Key;

pub fn run<T>(mode: Mode, mut args: T) -> Result<(), Box<dyn Error>>
where T: Iterator<Item = String>
{
  let result =
  {
    if mode == Mode::Analyse
    { analyse::run() }
    else
    {
      let key = Key::parse(&mut args)?;
      match mode
      {
        Mode::Encrypt => encrypt::run(key),
        Mode::Decrypt => decrypt::run(key),
        _ => unreachable!()
      }
    }
  };
  println!("{result}");

  Ok(())
}
