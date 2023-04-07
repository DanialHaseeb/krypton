use std::error::Error;

use crate::Γ;

pub fn parse<T>(args: &mut T) -> Result<usize, Box<dyn Error>>
where T: Iterator<Item = String>
{
  if let Some(position) = args.next()
  {
    let mut position: char = position.parse()?;
    if position.is_ascii_alphabetic()
    {
      position = position.to_ascii_uppercase();
      Ok(Γ[&position])
    }
    else
    { Err("Invalid rotor position. 🧐")? }
  }
  else
  { Err("Missing rotor position(s). 🧐")? }
}
