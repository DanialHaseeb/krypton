use std::error::Error;

#[derive(Debug)]
pub struct Key
{
  pub factor: usize,
  pub shift: usize
}

impl Key
{
  pub fn parse<T>(mut args: T) -> Result<Key, Box<dyn Error>>
  where T: Iterator<Item = String>
  {
    if let Some(key) = args.next()
    {
      let factor = key.parse::<usize>()?;
      if let Some(key) = args.next()
      {
        let shift = key.parse::<usize>()?;
        Ok(Key{ factor, shift })
      }
      else
      { Err("No shift provided. 🔑")? }
    }
    else
    { Err("No factor provided. 🔑")? }
  }
}
