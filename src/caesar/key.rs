use std::error::Error;

#[derive(Debug)]
pub struct Key
{ pub shift: usize }

impl Key
{
  pub fn parse<T>(mut args: T) -> Result<Key, Box<dyn Error>>
  where T: Iterator<Item = String>
  {
    if let Some(key) = args.next()
    {
      let shift = key.parse::<isize>()?;
      let shift = (shift % 26) as usize;
      Ok(Key{ shift })
    }
    else
    { Err("No key provided. 🔑")? }
  }

  pub fn default() -> Key
  { Key{ shift: 0 } }
}
